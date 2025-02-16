use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CONTROL_FILE: &str = "./integrations_control.json";

/// Structure to store installed integrations and their versions
#[derive(Serialize, Deserialize, Default)]
struct IntegrationControl {
    // Maps the integration name to the latest version
    integrations: HashMap<String, String>,
}

/// Get the installed version of an integration from the control file
fn get_installed_version(integration_name: &str) -> Result<String, String> {
    // If the control file doesn't exist, return "0.0.0"
    if !Path::new(CONTROL_FILE).exists() {
        return Ok("0.0.0".to_string());
    }
    // Reads the control file and parses the JSON
    let file_content = fs::read_to_string(CONTROL_FILE)
        .map_err(|e| format!("Failed to read control file: {}", e))?;
    let control: IntegrationControl = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    // Return the installed version of the integration or "0.0.0" if it's not found
    Ok(control
        .integrations
        .get(integration_name)
        .cloned()
        .unwrap_or_else(|| "0.0.0".to_string()))
}

/// Saves or updates the installed version of an integration to the control file
pub fn save_installed_version(integration_name: &str, version: &str) -> Result<(), String> {
    let mut control: IntegrationControl = if Path::new(CONTROL_FILE).exists() {
        let file_content = fs::read_to_string(CONTROL_FILE)
            .map_err(|e| format!("Failed to read control file: {}", e))?;
        serde_json::from_str(&file_content).unwrap_or_default()
    } else {
        IntegrationControl::default()
    };
    // Updates the installed version of the integration
    control.integrations.insert(integration_name.to_string(), version.to_string());
    // Serialize the structure and write it to the control file
    let json_content = serde_json::to_string_pretty(&control)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
    fs::write(CONTROL_FILE, json_content)
        .map_err(|e| format!("Failed to write control file: {}", e))?;

    Ok(())
}

/// Checks if a new version is available on GitHub by comparing the installed version with the latest version.
/// Receives the integration name and the latest version available as parameters.
#[tauri::command]
pub fn verify_latest_version(integration_name: &str, latest_version: &str) -> Result<bool, String> {
    let installed_version = get_installed_version(integration_name)?;
    // Returns true if a new version is available
    Ok(installed_version.as_str() < latest_version)
}
