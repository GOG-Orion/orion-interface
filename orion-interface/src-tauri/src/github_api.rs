use reqwest::blocking::Client;
use serde::Deserialize;

// Structure to deserialize repository content
#[derive(Deserialize)]
pub struct RepoContent {
    name: String,
    path: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}

// Maps integration name to repository name
fn get_repo_name(integration_name: &str) -> Option<&'static str> {
    match integration_name.to_lowercase().as_str() {
        "steam" => Some("GOG-Orion/galaxy-integration-steam"),
        "epic games" => Some("GOG-Orion/galaxy-integration-epicgames"),
        "ubisoft connect" => Some("GOG-Orion/galaxy-integration-ubisoft"),
        _ => None,
    }
}

// Function to get the download URL of the latest release
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

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {} - {}", response.status(), response.text().unwrap_or_else(|_| "Unknown error".to_string())));
    }

    let release: serde_json::Value = response.json()
        .map_err(|e| format!("Failed to parse GitHub release response: {}", e))?;

    let assets = release.get("assets")
        .and_then(|a| a.as_array())
        .ok_or_else(|| "No assets field found in the release response.".to_string())?;

    let asset = assets.first()
        .ok_or_else(|| "No assets found in the release.".to_string())?;

    let download_url = asset.get("browser_download_url")
        .and_then(|url| url.as_str())
        .ok_or_else(|| "Repository does not contain a valid URL.".to_string())?;

    Ok(download_url.to_string())
}

// Function to fetch the contents of a repository at a given path
pub fn fetch_repo_contents(owner: &str, repo: &str, path: &str) -> Result<Vec<RepoContent>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner, repo, path);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "RustApp")
        .send()
        .map_err(|e| format!("Failed to fetch contents: {}", e))?;

    if response.status().is_success() {
        let contents: Vec<RepoContent> = response.json()
            .map_err(|e| format!("Failed to parse response (HTTP {}): {}", response.status(), e))?;
        Ok(contents)
    } else {
        Err(format!(
            "Failed to fetch contents. HTTP Status: {} - Error: {}",
            response.status(),
            response.text().unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}
