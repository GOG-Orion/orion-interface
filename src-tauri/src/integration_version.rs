use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const INTEGRATIONS_FILE: &str = "../src/components/utils/integrations.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IntegrationInfo {
    pub name: String,
    pub image: String,
    pub download_url: String,
    pub installed_version: String,
}

/// Lê as informações das integrações a partir do arquivo JSON
pub fn read_integrations() -> Result<Vec<IntegrationInfo>, String> {
    if !Path::new(INTEGRATIONS_FILE).exists() {
        return Err(format!("File {} not found", INTEGRATIONS_FILE));
    }
    let content = fs::read_to_string(INTEGRATIONS_FILE)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
}

/// Atualiza a versão instalada de uma integração no arquivo JSON
pub fn save_installed_version(integration_name: &str, version: &str) -> Result<(), String> {
    let mut integrations = read_integrations()?;
    // Atualiza o registro da integração
    for integration in integrations.iter_mut() {
        if integration.name.to_lowercase() == integration_name.to_lowercase() {
            integration.installed_version = version.to_string();
        }
    }
    let json_content = serde_json::to_string_pretty(&integrations)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
    fs::write(INTEGRATIONS_FILE, json_content)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

/// Verifica se a versão instalada é menor que a última versão disponível
#[tauri::command]
pub fn verify_latest_version(integration_name: &str, latest_version: &str) -> Result<bool, String> {
    let integrations = read_integrations()?;
    let installed_version = integrations.iter()
        .find(|i| i.name.to_lowercase() == integration_name.to_lowercase())
        .map(|i| i.installed_version.clone())
        .unwrap_or_else(|| "0.0.0".to_string());
    Ok(installed_version.as_str() < latest_version)
}
