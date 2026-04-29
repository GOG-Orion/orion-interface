use crate::github_api::get_latest_release;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const CONTROL_FILE_NAME: &str = "integrations_control.json";

/// Structure to store installed integrations and their versions
#[derive(Serialize, Deserialize, Default)]
struct IntegrationControl {
    // Maps the integration name to the installed version
    integrations: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionStatus {
    pub integration_name: String,
    pub installed_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub download_url: Option<String>,
}

fn control_file_path() -> PathBuf {
    if let Ok(path) = env::var("ORION_CONTROL_FILE") {
        return PathBuf::from(path);
    }

    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(CONTROL_FILE_NAME)
}

/// Get the installed version of an integration from the control file
fn get_installed_version(integration_name: &str) -> Result<String, String> {
    let control_file = control_file_path();
    // If the control file doesn't exist, return "0.0.0"
    if !Path::new(&control_file).exists() {
        return Ok("0.0.0".to_string());
    }
    // Reads the control file and parses the JSON
    let file_content = fs::read_to_string(&control_file)
        .map_err(|e| format!("Failed to read control file: {}", e))?;
    let control: IntegrationControl =
        serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    // Return the installed version of the integration or "0.0.0" if it's not found
    Ok(control
        .integrations
        .get(integration_name)
        .cloned()
        .unwrap_or_else(|| "0.0.0".to_string()))
}

/// Saves or updates the installed version of an integration to the control file
pub fn save_installed_version(integration_name: &str, version: &str) -> Result<(), String> {
    let control_file = control_file_path();
    let mut control: IntegrationControl = if Path::new(&control_file).exists() {
        let file_content = fs::read_to_string(&control_file)
            .map_err(|e| format!("Failed to read control file: {}", e))?;
        serde_json::from_str(&file_content).unwrap_or_default()
    } else {
        IntegrationControl::default()
    };
    // Updates the installed version of the integration
    control
        .integrations
        .insert(integration_name.to_string(), version.to_string());
    // Serialize the structure and write it to the control file
    let json_content = serde_json::to_string_pretty(&control)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
    if let Some(parent) = control_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create control file directory: {}", e))?;
    }
    fs::write(control_file, json_content)
        .map_err(|e| format!("Failed to write control file: {}", e))?;

    Ok(())
}

pub fn normalize_version_tag(version: &str) -> &str {
    version.trim_start_matches(['v', 'V'])
}

pub fn compare_versions(installed_version: &str, latest_version: &str) -> Result<bool, String> {
    let installed = Version::parse(normalize_version_tag(installed_version))
        .map_err(|e| format!("Invalid installed version '{}': {}", installed_version, e))?;
    let latest = Version::parse(normalize_version_tag(latest_version))
        .map_err(|e| format!("Invalid latest version '{}': {}", latest_version, e))?;

    Ok(installed < latest)
}

/// Checks if a new version is available on GitHub by comparing the installed version with the latest version.
#[tauri::command]
pub fn verify_latest_version(integration_name: &str) -> Result<VersionStatus, String> {
    let installed_version = get_installed_version(integration_name)?;
    let latest_release = get_latest_release(integration_name)?;
    let update_available = compare_versions(&installed_version, &latest_release.tag_name)?;

    Ok(VersionStatus {
        integration_name: integration_name.to_string(),
        installed_version,
        latest_version: latest_release.tag_name,
        update_available,
        download_url: Some(latest_release.download_url),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        env::temp_dir().join(format!("orion-{}-{}", name, nanos))
    }

    #[test]
    fn compares_versions_semantically() {
        assert!(compare_versions("v1.0.9", "v1.0.10").expect("comparison should succeed"));
        assert!(!compare_versions("v1.2.0", "v1.1.9").expect("comparison should succeed"));
    }

    #[test]
    fn saves_and_reads_installed_version() {
        let control_file = unique_path("control").with_extension("json");
        env::set_var("ORION_CONTROL_FILE", &control_file);

        save_installed_version("steam", "v1.2.3").expect("version should save");
        let installed = get_installed_version("steam").expect("version should read");
        assert_eq!(installed, "v1.2.3");

        let _ = fs::remove_file(&control_file);
        env::remove_var("ORION_CONTROL_FILE");
    }
}
