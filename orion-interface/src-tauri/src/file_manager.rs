
use crate::{Path, Command, System, integrations};
use reqwest::blocking;
use std::fs;
use std::fs::copy;


const DOWNLOAD_DIR: &str = "./downloads"; // Diretório temporário de downloads

/// Verifica se o processo está em execução
pub fn is_process_running(process_name: &str) -> bool {
    let system = System::new_all(); // Inicializa informações do sistema
    system.processes_by_name(process_name).count() > 0
}

/// Função para download de arquivos, extração, execução e limpeza
#[tauri::command]
pub fn download_file(integration_name: &str) -> Result<String, String> {
    // Verifica se o GalaxyClient.exe está em execução
    if is_process_running("GalaxyClient.exe") {
        return Err("The Galaxy Client is currently running. Please close it before proceeding.".to_string());
    }

    // Resolve o URL de download usando a API do GitHub
    let download_url = integrations.resolve_download_url(integration_name)?;

    // Caminho para o arquivo temporário
    let dest_path = format!("{}/{}.zip", DOWNLOAD_DIR, integration_name);

    // Criar diretório de downloads, se necessário
    fs::create_dir_all(DOWNLOAD_DIR).map_err(|e| e.to_string())?;

    // Baixar o arquivo zip
    let response = blocking::get(&download_url).map_err(|e| e.to_string())?;
    let mut dest_file = File::create(Path::new(&dest_path)).map_err(|e| e.to_string())?;
    copy(&mut response.bytes().map_err(|e| e.to_string())?.as_ref(), &mut dest_file)
        .map_err(|e| e.to_string())?;

    // Extrair o arquivo zip usando PowerShell no Windows
    Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Expand-Archive -Path {} -DestinationPath {}/galaxy-integration-steam",
                dest_path, DOWNLOAD_DIR
            ),
        ])
        .output()
        .map_err(|e| format!("Failed to extract zip file: {}", e.to_string()))?;

    // Executar script de instalação
    Command::new("powershell")
        .arg(format!("{}/galaxy-integration-steam/install.bat", DOWNLOAD_DIR))
        .output()
        .map_err(|e| format!("Failed to run install script: {}", e.to_string()))?;

    // Limpar arquivos temporários
    clean_up_temp_files();

    Ok("Download, installation, and cleanup completed.".to_string())
}

fn clean_up_temp_files() {
    // Limpar arquivos temporários
    fs::remove_dir_all(DOWNLOAD_DIR).unwrap_or_else(|e| {
        eprintln!("Failed to clean up temporary files: {}", e);
    });
}