use crate::app_config::get_install_root;
use crate::github_api::get_latest_release;
use crate::integration_version::save_installed_version;
use reqwest::blocking::get;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::{System, SystemExt};

pub const DOWNLOAD_DIR: &str = "./downloads";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct InstallReadiness {
    pub client_running: bool,
    pub client_names: Vec<String>,
    pub message: String,
}

fn gog_client_process_names() -> Vec<&'static str> {
    if cfg!(target_os = "windows") {
        vec!["GalaxyClient.exe", "GalaxyClient"]
    } else if cfg!(target_os = "macos") {
        vec!["GOG Galaxy", "GalaxyClient"]
    } else {
        vec!["GalaxyClient", "goggalaxy", "GOG Galaxy"]
    }
}

/// Verify if a process is running
pub fn is_process_running(process_name: &str) -> bool {
    let system = System::new_all();
    system.processes_by_name(process_name).count() > 0
}

fn detect_gog_client_running() -> Vec<String> {
    gog_client_process_names()
        .into_iter()
        .filter(|name| is_process_running(name))
        .map(|name| name.to_string())
        .collect()
}

pub fn install_readiness() -> InstallReadiness {
    let running = detect_gog_client_running();

    if running.is_empty() {
        InstallReadiness {
            client_running: false,
            client_names: running,
            message: "GOG is closed. Installation is allowed.".to_string(),
        }
    } else {
        InstallReadiness {
            client_running: true,
            client_names: running.clone(),
            message: "Close GOG Galaxy before installing to avoid overwriting plugin files."
                .to_string(),
        }
    }
}

#[tauri::command]
pub fn get_install_readiness() -> InstallReadiness {
    install_readiness()
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
        let metadata = fs::symlink_metadata(&entry_path)
            .map_err(|e| format!("Failed to inspect release contents: {}", e))?;

        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Refusing to copy symlinked release content: {}",
                entry_path.display()
            ));
        }

        if metadata.is_dir() {
            copy_directory_contents(&entry_path, &target_path)?;
        } else if metadata.is_file() {
            fs::copy(&entry_path, &target_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        } else {
            return Err(format!(
                "Unsupported release content type: {}",
                entry_path.display()
            ));
        }
    }

    Ok(())
}

fn is_forbidden_release_extension(extension: &str) -> bool {
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "bat"
            | "cmd"
            | "ps1"
            | "exe"
            | "com"
            | "scr"
            | "msi"
            | "vbs"
            | "js"
            | "jse"
            | "wsf"
            | "wsh"
            | "lnk"
    )
}

fn validate_release_tree(root: &Path) -> Result<(), String> {
    for entry in
        fs::read_dir(root).map_err(|e| format!("Failed to inspect release contents: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to inspect release contents: {}", e))?;
        let entry_path = entry.path();
        let metadata = fs::symlink_metadata(&entry_path)
            .map_err(|e| format!("Failed to inspect release contents: {}", e))?;

        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Refusing to use symlinked release content: {}",
                entry_path.display()
            ));
        }

        if metadata.is_dir() {
            validate_release_tree(&entry_path)?;
            continue;
        }

        if metadata.is_file() {
            if let Some(extension) = entry_path.extension().and_then(|ext| ext.to_str()) {
                if is_forbidden_release_extension(extension) {
                    return Err(format!(
                        "Refusing to install executable or script file: {}",
                        entry_path.display()
                    ));
                }
            }
            continue;
        }

        return Err(format!(
            "Unsupported release content type: {}",
            entry_path.display()
        ));
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

fn parse_sha256_digest(digest: &str) -> Result<Vec<u8>, String> {
    let value = digest.trim();
    let encoded = value
        .strip_prefix("sha256:")
        .ok_or_else(|| "Unsupported digest algorithm. Expected sha256.".to_string())?;

    hex::decode(encoded).map_err(|e| format!("Failed to decode sha256 digest: {}", e))
}

fn verify_downloaded_archive(bytes: &[u8], digest: Option<&str>) -> Result<(), String> {
    let Some(digest) = digest else {
        return Ok(());
    };

    let expected = parse_sha256_digest(digest)?;
    let actual = Sha256::digest(bytes);

    if expected != actual.as_slice() {
        return Err("Downloaded archive hash does not match the release digest.".to_string());
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
    let readiness = install_readiness();
    if readiness.client_running {
        return Err(readiness.message);
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

        verify_downloaded_archive(&bytes, latest_release.asset_digest.as_deref())?;

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

        validate_release_tree(payload_root)?;

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

    #[test]
    fn rejects_forbidden_script_files() {
        let temp_root = std::env::temp_dir().join(format!("orion-script-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("temp structure should be creatable");
        fs::write(temp_root.join("install.bat"), "echo hi").expect("script should be writable");

        let result = validate_release_tree(&temp_root);
        assert!(result.is_err());

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn verifies_matching_sha256_digest() {
        let bytes = b"orion";
        let digest = format!("sha256:{:x}", Sha256::digest(bytes));

        assert!(verify_downloaded_archive(bytes, Some(&digest)).is_ok());
    }

    #[test]
    fn rejects_mismatched_sha256_digest() {
        let bytes = b"orion";
        let digest = "sha256:000102";

        let result = verify_downloaded_archive(bytes, Some(digest));
        assert!(result.is_err());
    }
}
