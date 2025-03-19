use sysinfo::{System, SystemExt};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::github_api::resolve_download_url;
use reqwest::blocking::get;
use log::info;

pub const DOWNLOAD_DIR: &str = "./downloads";

/// Verifica se um processo está em execução
pub fn is_process_running(process_name: &str) -> bool {
    let system = System::new_all();
    system.processes_by_name(process_name).count() > 0
}

/// Busca recursivamente o arquivo "install.bat" no diretório
fn find_install_script(dir: &Path) -> Option<PathBuf> {
    let candidate = dir.join("auto-copy-windows.bat");
    if candidate.exists() {
        return Some(candidate);
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_install_script(&path) {
                    return Some(found);
                }
            }
        }
    }
    None
}

#[tauri::command]
pub fn download_file(integration_name: &str) -> Result<String, String> {
    if is_process_running("GalaxyClient.exe") {
        return Err("The Galaxy Client is currently running. Please close it before proceeding.".to_string());
    }
    
    // Obtém a URL de download resolvida a partir da API do GitHub
    let download_url = resolve_download_url(integration_name)?;
    
    // Define os caminhos para download e extração
    let integration_dir = PathBuf::from(DOWNLOAD_DIR).join(integration_name);
    let zip_path = integration_dir.with_extension("zip");
    
    // Cria o diretório de download
    fs::create_dir_all(&integration_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    
    info!("Starting download for integration: {}", integration_name);
    
    // Realiza o download do arquivo ZIP
    let response = get(&download_url).map_err(|e| format!("Failed to download file: {}", e))?;
    let bytes = response.bytes().map_err(|e| format!("Failed to read response bytes: {}", e))?;
    let mut dest_file = File::create(&zip_path).map_err(|e| format!("Failed to create file: {}", e))?;
    copy(&mut &bytes[..], &mut dest_file).map_err(|e| format!("Failed to write to file: {}", e))?;
    
    // Extrai o arquivo ZIP usando PowerShell
    let extract_path = integration_dir.clone();
    let extract_command = format!(
        "Expand-Archive -Path \"{}\" -DestinationPath \"{}\" -Force",
        zip_path.display(),
        extract_path.display()
    );
    
    let extraction_result = Command::new("powershell")
        .args(["-Command", &extract_command])
        .output()
        .map_err(|e| format!("Failed to extract ZIP file: {}", e))?;
    
    if !extraction_result.status.success() {
        return Err(format!(
            "Extraction failed: {}",
            String::from_utf8_lossy(&extraction_result.stderr)
        ));
    }
    
    // Procura recursivamente o arquivo "install.bat"
    let install_script = find_install_script(&extract_path)
        .ok_or_else(|| "Installation script not found.".to_string())?;
    
    // Executa o script de instalação usando cmd
    let install_result = Command::new("cmd")
        .args(["/C", install_script.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to run install script: {}", e))?;
    
    if !install_result.status.success() {
        return Err(format!(
            "Installation failed: {}",
            String::from_utf8_lossy(&install_result.stderr)
        ));
    }
    
    info!("Cleaning up temporary files in {}", integration_dir.display());
    clean_up_temp_files(&integration_dir)?;
    
    Ok("Download, installation, and cleanup completed successfully.".to_string())
}

fn clean_up_temp_files(path: &Path) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| format!("Failed to clean up temporary files: {}", e))
}
