
use reqwest::blocking::Client;
use serde::Deserialize;

// Estrutura para deserializar o conteúdo do repositório
#[derive(Deserialize)]
pub struct RepoContent {
    name: String,
    path: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}

/// Modificar a função resolve_download_url para acessar os assets de releases
pub fn resolve_download_url(integration_name: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let release_url = format!(
        "https://api.github.com/repos/{}/releases/latest", 
        integration_name
    );

    let response = client
        .get(&release_url)
        .header("User-Agent", "Rust-Integration-App") // User-Agent é obrigatório
        .send()
        .map_err(|e| format!("Failed to fetch GitHub release: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "GitHub API returned error: {}",
            response.status()
        ));
    }

    let release: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse GitHub release response: {}", e))?;

    // Encontra o repositório correspondente ao nome da integração
    /*let repo = repos
        .iter()
        .find(|repo| {
            repo["name"]
                .as_str()
                .map_or(false, |repo_name| repo_name.contains(integration_name.to_lowercase().as_str()))
        })
        .ok_or_else(|| format!("No repository found for integration: {}", integration_name))?;
    */// Busca o primeiro asset da release
    let asset = release["assets"]
        .as_array()
        .and_then(|assets| assets.first())
        .ok_or_else(|| "No assets found in the release.".to_string())?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .ok_or_else(|| "Repository does not contain a valid URL.".to_string())?;

    Ok(download_url.to_string())
}


pub fn fetch_repo_contents(owner: &str, repo: &str, path: &str) -> Result<Vec<RepoContent>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "RustApp")
        .send()
        .map_err(|e| format!("Failed to fetch contents: {}", e))?;

    if response.status().is_success() {
        let contents: Vec<RepoContent> = response.json().map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(contents)
    } else {
        Err(format!(
            "Failed to fetch contents. Status: {}",
            response.status()
        ))
    }
}

