use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::github_api::fetch_latest_version;

const CONTROL_FILE: &str = "./integrations_control.json"; // Caminho do arquivo de controle

/// Estrutura para armazenar as versões das integrações
#[derive(Serialize, Deserialize, Default)]
struct IntegrationControl {
    integrations: HashMap<String, String>,
}

/// Obtém a versão instalada de uma integração
fn get_installed_version(integration_name: &str) -> Result<String, String> {
    // Se o arquivo não existir, retorna uma versão padrão "0.0.0"
    if !Path::new(CONTROL_FILE).exists() {
        return Ok("0.0.0".to_string());
    }

    // Lê e desserializa o JSON
    let file_content = fs::read_to_string(CONTROL_FILE).map_err(|e| format!("Failed to read control file: {}", e))?;
    let control: IntegrationControl = serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Retorna a versão da integração ou "0.0.0" se não estiver no controle
    Ok(control.integrations.get(integration_name).cloned().unwrap_or("0.0.0".to_string()))
}

/// Salva a versão instalada de uma integração no arquivo
pub fn save_installed_version(integration_name: &str, version: &str) -> Result<(), String> {
    // Cria um mapa de controle
    let mut control: IntegrationControl = if Path::new(CONTROL_FILE).exists() {
        let file_content = fs::read_to_string(CONTROL_FILE).map_err(|e| format!("Failed to read control file: {}", e))?;
        serde_json::from_str(&file_content).unwrap_or_default()
    } else {
        IntegrationControl::default()
    };

    // Atualiza a versão da integração
    control.integrations.insert(integration_name.to_string(), version.to_string());

    // Serializa e salva o JSON atualizado
    let json_content = serde_json::to_string_pretty(&control).map_err(|e| format!("Failed to serialize JSON: {}", e))?;
    fs::write(CONTROL_FILE, json_content).map_err(|e| format!("Failed to write control file: {}", e))?;

    Ok(())
}

/// Verifica se há uma nova versão disponível no GitHub
#[tauri::command]
pub fn verify_latest_version(integration_name: &str) -> Result<bool, String> {
    let installed_version = get_installed_version(integration_name)?;
    let latest_version = fetch_latest_version(integration_name)?;

    Ok(installed_version < latest_version) // Retorna true se houver uma atualização disponível
}
