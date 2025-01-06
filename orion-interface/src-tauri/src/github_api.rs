
use reqwest::blocking::Client;
use serde::Deserialize;

// Estrutura para deserializar o conteúdo do repositório
#[derive(Deserialize)]
struct RepoContent {
    name: String,
    path: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}

/// Busca o `download_url` usando a API do GitHub
pub fn resolve_download_url(integration_name: &str) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get(GITHUB_API_BASE)
        .header("User-Agent", "Rust-Integration-App") // Cabeçalho obrigatório para a API do GitHub
        .send()
        .map_err(|e| format!("Failed to fetch GitHub repositories: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "GitHub API returned error: {}",
            response.status()
        ));
    }

    let repos: Vec<serde_json::Value> = response
        .json()
        .map_err(|e| format!("Failed to parse GitHub API response: {}", e))?;

    // Encontra o repositório correspondente ao nome da integração
    let repo = repos
        .iter()
        .find(|repo| {
            repo["name"]
                .as_str()
                .map_or(false, |repo_name| repo_name.contains(integration_name.to_lowercase().as_str()))
        })
        .ok_or_else(|| format!("No repository found for integration: {}", integration_name))?;

    // Monta a URL do repositório
    let html_url = repo["html_url"]
        .as_str()
        .ok_or_else(|| "Repository does not contain a valid URL.".to_string())?;

    Ok(html_url.to_string())
}

pub fn fetch_repo_contents(owner: &str, repo: &str, path: &str, token: &str) -> Result<Vec<RepoContent>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "RustApp")
        .send()
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let contents: Vec<RepoContent> = response.json().map_err(|e| e.to_string())?;
        Ok(contents)
    } else {
        Err(format!(
            "Failed to fetch contents: {}",
            response.status()
        ))
    }
}

fn main() {
    let token = "YOUR_PERSONAL_ACCESS_TOKEN";
    match fetch_repo_contents("GOG-Orion", "galaxy-integration-steam", "", token) {
        Ok(contents) => {
            for content in contents {
                println!("Name: {}, URL: {:?}", content.name, content.download_url);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
