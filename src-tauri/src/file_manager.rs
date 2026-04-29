use crate::app_config::get_install_root;
use crate::github_api::get_latest_release;
use crate::integration_version::save_installed_version;
use reqwest::blocking::get;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::{System, SystemExt};

pub const DOWNLOAD_DIR: &str = "./downloads";

/// Verify if a process is running
pub fn is_process_running(process_name: &str) -> bool {
    let system = System::new_all();
    system.processes_by_name(process_name).count() > 0
}

fn normalize_install_folder_name(integration_name: &str) -> String {
    let normalized = integration_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();

    normalized.trim_matches('_').to_string()
}

fn copy_directory_contents(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|e| format!("Failed to create destination directory: {}", e))?;

    for entry in fs::read_dir(source).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let entry_path = entry.path();
        let target_path = destination.join(entry.file_name());

        if entry_path.is_dir() {
            copy_directory_contents(&entry_path, &target_path)?;
        } else {
            fs::copy(&entry_path, &target_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }

    Ok(())
}

fn find_manifest_file(root: &Path) -> Result<PathBuf, String> {
    if root.is_file() {
        return Err("Install root must be a directory.".to_string());
    }

    for entry in fs::read_dir(root).map_err(|e| format!("Failed to read extracted files: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to inspect extracted files: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            if let Ok(found) = find_manifest_file(&path) {
                return Ok(found);
            }
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.eq_ignore_ascii_case("manifest.json"))
            .unwrap_or(false)
        {
            return Ok(path);
        }
    }

    Err("manifest.json was not found in the extracted release.".to_string())
}

fn read_and_validate_manifest(manifest_path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(manifest_path)
        .map_err(|e| format!("Failed to read manifest.json: {}", e))?;
    let manifest: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse manifest.json: {}", e))?;

    if !manifest.is_object() {
        return Err("manifest.json must contain a JSON object.".to_string());
    }

    Ok(())
}

fn extract_archive(zip_path: &Path, destination: &Path) -> Result<(), String> {
    let status = if cfg!(target_os = "windows") {
        let extract_command = format!(
            "Expand-Archive -Path \"{}\" -DestinationPath \"{}\" -Force",
            zip_path.display(),
            destination.display()
        );

        Command::new("powershell")
            .args(["-NoProfile", "-Command", &extract_command])
            .output()
            .map_err(|e| format!("Failed to extract ZIP file: {}", e))?
    } else {
        Command::new("unzip")
            .args([
                "-o",
                &zip_path.to_string_lossy(),
                "-d",
                &destination.to_string_lossy(),
            ])
            .output()
            .map_err(|e| format!("Failed to extract ZIP file: {}", e))?
    };

    if !status.status.success() {
        return Err(format!(
            "Extraction failed: {}",
            String::from_utf8_lossy(&status.stderr)
        ));
    }

    Ok(())
}

fn clean_up_temp_files(paths: &[&Path]) {
    for path in paths {
        if path.is_dir() {
            let _ = fs::remove_dir_all(path);
        } else if path.exists() {
            let _ = fs::remove_file(path);
        }
    }
}

/// Function to download, extract, install, and clean up temporary files
#[tauri::command]
pub fn download_file(integration_name: &str) -> Result<String, String> {
    if cfg!(target_os = "windows") && is_process_running("GalaxyClient.exe") {
        return Err(
            "The Galaxy Client is currently running. Please close it before proceeding."
                .to_string(),
        );
    }

    let latest_release = get_latest_release(integration_name)?;
    let install_root = get_install_root()?;
    fs::create_dir_all(&install_root)
        .map_err(|e| format!("Failed to create install root: {}", e))?;

    let download_dir = PathBuf::from(DOWNLOAD_DIR).join(integration_name);
    let zip_path = download_dir.with_extension("zip");
    let install_target = install_root.join(normalize_install_folder_name(integration_name));

    let result = (|| -> Result<String, String> {
        fs::create_dir_all(&download_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        let response = get(&latest_release.download_url)
            .map_err(|e| format!("Failed to download file: {}", e))?;
        let bytes = response
            .bytes()
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let mut dest_file =
            File::create(&zip_path).map_err(|e| format!("Failed to create archive file: {}", e))?;
        copy(&mut &bytes[..], &mut dest_file)
            .map_err(|e| format!("Failed to write archive file: {}", e))?;

        extract_archive(&zip_path, &download_dir)?;

        let manifest_path = find_manifest_file(&download_dir)?;
        read_and_validate_manifest(&manifest_path)?;

        let payload_root = manifest_path
            .parent()
            .ok_or_else(|| "Unable to determine the extracted payload root.".to_string())?;

        if install_target.exists() {
            fs::remove_dir_all(&install_target)
                .map_err(|e| format!("Failed to clear previous installation: {}", e))?;
        }

        copy_directory_contents(payload_root, &install_target)?;
        save_installed_version(integration_name, &latest_release.tag_name)?;

        Ok(format!(
            "Installed {} {} to {}",
            integration_name,
            latest_release.tag_name,
            install_target.display()
        ))
    })();

    clean_up_temp_files(&[&download_dir, &zip_path]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_install_folder_name() {
        assert_eq!(normalize_install_folder_name("Epic Games"), "epic_games");
        assert_eq!(
            normalize_install_folder_name("Ubisoft Connect"),
            "ubisoft_connect"
        );
    }

    #[test]
    fn can_find_manifest_in_nested_directory() {
        let temp_root = std::env::temp_dir().join(format!("orion-manifest-{}", std::process::id()));
        let nested = temp_root.join("bundle");
        let manifest = nested.join("manifest.json");
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&nested).expect("temp structure should be creatable");
        fs::write(&manifest, "{\"name\":\"Steam\"}").expect("manifest should be writable");

        let found = find_manifest_file(&temp_root).expect("manifest should be found");
        assert_eq!(found, manifest);

        let _ = fs::remove_dir_all(&temp_root);
    }
}
