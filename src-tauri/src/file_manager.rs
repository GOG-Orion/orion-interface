use sysinfo::{System, SystemExt};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::github_api::resolve_download_url;
use reqwest::blocking::get;

pub const DOWNLOAD_DIR: &str = "./downloads"; //Temp download directory

/// Verify if a process is running
pub fn is_process_running(process_name: &str) -> bool {
    let system = System::new_all();
    system.processes_by_name(process_name).count() > 0
}

/// Function to download, extract, install, and clean up temporary files
#[tauri::command]
pub fn download_file(integration_name: &str) -> Result<String, String> {
    // Verify if the Galaxy Client is running
    if is_process_running("GalaxyClient.exe") {
        return Err("The Galaxy Client is currently running. Please close it before proceeding.".to_string());
    }

    let download_url = resolve_download_url(integration_name)?;

    // Define paths for download and extraction
    let integration_dir = PathBuf::from(DOWNLOAD_DIR).join(integration_name);
    let zip_path = integration_dir.with_extension("zip");

    // Create the download directory
    fs::create_dir_all(&integration_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    // Download ZIP file
    let response = get(&download_url).map_err(|e| format!("Failed to download file: {}", e))?;
    let bytes = response.bytes().map_err(|e| format!("Failed to read response bytes: {}", e))?;
    
    let mut dest_file = File::create(&zip_path).map_err(|e| format!("Failed to create file: {}", e))?;
    copy(&mut &bytes[..], &mut dest_file).map_err(|e| format!("Failed to write to file: {}", e))?;

    // Extract the ZIP file using PowerShell
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

    let install_script = extract_path.join("install.bat");

    // Execute the installation script if it exists
    if install_script.exists() {
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
    } else {
        return Err("Installation script not found.".to_string());
    }

    // Clean up temporary files
    clean_up_temp_files(&integration_dir)?;

    Ok("Download, installation, and cleanup completed successfully.".to_string())
}

/// Function to clean up temporary files
fn clean_up_temp_files(path: &Path) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| format!("Failed to clean up temporary files: {}", e))
}
