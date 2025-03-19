use reqwest::blocking::Client;
use serde::Deserialize;
use crate::integration_version::save_installed_version;

fn get_repo_name(integration_name: &str) -> Option<&'static str> {
    match integration_name.to_lowercase().as_str() {
        "steam" => Some("GOG-Orion/galaxy-integration-steam"),
        "epic games" => Some("GOG-Orion/galaxy-integration-epicgames"),
        "ubisoft connect" => Some("GOG-Orion/galaxy-integration-ubisoft"),
        _ => None,
    }
}

/// Resolve a URL de download e atualiza o arquivo de controle com a versão (tag) da release
pub fn resolve_download_url(integration_name: &str) -> Result<String, String> {
    let repo_name = get_repo_name(integration_name)
        .ok_or_else(|| format!("No repository found for integration: {}", integration_name))?;
    let client = Client::new();
    let release_url = format!("https://api.github.com/repos/{}/releases/latest", repo_name);

    let mut request = client.get(&release_url)
        .header("User-Agent", "Rust-Integration-App");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        request = request.header("Authorization", format!("token {}", token));
    }
    let response = request.send()
        .map_err(|e| format!("Failed to fetch GitHub release: {}", e))?;
    let status = response.status();
    if !status.is_success() {
        let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("GitHub API returned error: {} - {}", status, err_text));
    }
    let release: serde_json::Value = response.json()
        .map_err(|e| format!("Failed to parse release response: {}", e))?;

    let assets = release.get("assets")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "No assets field found in the release response.".to_string())?;

    // Detecta o sistema operacional atual
    let os = std::env::consts::OS; // "windows", "macos", "linux", etc.
    let asset = if os == "windows" {
        // Procura asset que contenha "windows" no nome
        assets.iter().find(|a| {
            a.get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_lowercase().contains("windows"))
                .unwrap_or(false)
        })
    } else if os == "macos" {
        // Procura asset que contenha "mac" no nome
        assets.iter().find(|a| {
            a.get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_lowercase().contains("mac"))
                .unwrap_or(false)
        })
    } else {
        // Se não for Windows nem macOS, pega o primeiro asset
        assets.first()
    }.ok_or_else(|| "No suitable asset found in the release.".to_string())?;

    let download_url = asset.get("browser_download_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Repository does not contain a valid URL.".to_string())?;

    Ok(download_url.to_string())
}


/// (Opcional) Função para buscar o conteúdo do repositório
pub fn fetch_repo_contents(owner: &str, repo: &str, path: &str) -> Result<Vec<RepoContent>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);
    let client = Client::new();
    let response = client.get(&url)
        .header("User-Agent", "RustApp")
        .send()
        .map_err(|e| format!("Failed to fetch contents: {}", e))?;
    let status = response.status();
    if status.is_success() {
        let contents: Vec<RepoContent> = response.json()
            .map_err(|e| format!("Failed to parse contents (HTTP {}): {}", status, e))?;
        Ok(contents)
    } else {
        let err_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to fetch contents. HTTP {} - {}", status, err_text))
    }
}

#[derive(Deserialize)]
pub struct RepoContent {
    name: String,
    path: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}
