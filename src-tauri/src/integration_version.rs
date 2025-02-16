use serde::Deserialize;
use std::fs;

// Estrutura para integração com o campo de URL de download
#[derive(Deserialize)]
pub struct Integration {
    name: String,
    download_url: String,
    version: String, // Campo de versão para verificação de atualização
}

// Função para verificar a última versão da integração
#[tauri::command]
pub fn verify_latest_version(integration_name: &str, current_version: &str) -> Result<bool, String> {
    // Caminho para o JSON de integrações
    let integrations_path = "./src/components/integrations.json";

    // Ler o arquivo de integrações
    let integrations: Vec<Integration> = {
        let file = fs::read_to_string(integrations_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&file).map_err(|e| e.to_string())?
    };

    // Encontrar a integração e verificar a versão
    let integration = integrations
        .iter()
        .find(|&integration| integration.name == integration_name)
        .ok_or_else(|| "Integration not found.".to_string())?;

    // Comparar a versão atual com a versão mais recente
    Ok(current_version < integration.version.as_str()) // Retorna true se houver nova versão
}

