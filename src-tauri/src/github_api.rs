use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

// Structure to deserialize repository content
#[derive(Deserialize)]
pub struct RepoContent {
    name: String,
    path: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitHubRelease {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseInfo {
    pub integration_name: String,
    pub repo_name: String,
    pub tag_name: String,
    pub asset_name: String,
    pub download_url: String,
    pub asset_digest: Option<String>,
}

// Maps integration name to repository name
pub(crate) fn get_repo_name(integration_name: &str) -> Option<&'static str> {
    match integration_name.to_lowercase().as_str() {
        "steam" => Some("GOG-Orion/galaxy-integration-steam"),
        "epic games" => Some("GOG-Orion/galaxy-integration-epicgames"),
        "ubisoft connect" => Some("GOG-Orion/galaxy-integration-ubisoft"),
        _ => None,
    }
}

fn pick_asset<'a>(release: &'a GitHubRelease) -> Result<&'a ReleaseAsset, String> {
    release
        .assets
        .iter()
        .find(|asset| {
            asset.name.ends_with(".zip")
                || matches!(asset.content_type.as_deref(), Some("application/zip"))
        })
        .or_else(|| release.assets.first())
        .ok_or_else(|| "No assets found in the release.".to_string())
}

fn normalize_asset_digest(digest: Option<&str>) -> Option<String> {
    digest
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn is_sha256_digest(digest: &str) -> bool {
    digest.to_ascii_lowercase().starts_with("sha256:")
}

pub fn get_latest_release(integration_name: &str) -> Result<ReleaseInfo, String> {
    let repo_name = get_repo_name(integration_name)
        .ok_or_else(|| format!("No repository found for integration: {}", integration_name))?;

    let client = Client::new();
    let release_url = format!("https://api.github.com/repos/{}/releases/latest", repo_name);

    let mut request = client
        .get(&release_url)
        .header("User-Agent", "Rust-Integration-App");

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        request = request.header("Authorization", format!("token {}", token));
    }

    let response = request
        .send()
        .map_err(|e| format!("Failed to fetch GitHub release: {}", e))?;

    let status = response.status(); // Saves before `response.json()`

    if !status.is_success() {
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!(
            "GitHub API returned error: {} - {}",
            status, error_text
        ));
    }

    let release: GitHubRelease = response
        .json()
        .map_err(|e| format!("Failed to parse GitHub release response: {}", e))?;

    let asset = pick_asset(&release)?;

    Ok(ReleaseInfo {
        integration_name: integration_name.to_string(),
        repo_name: repo_name.to_string(),
        tag_name: release.tag_name,
        asset_name: asset.name.clone(),
        download_url: asset.browser_download_url.clone(),
        asset_digest: normalize_asset_digest(asset.digest.as_deref()).and_then(|digest| {
            if is_sha256_digest(&digest) {
                Some(digest)
            } else {
                None
            }
        }),
    })
}

// Function to get the download URL of the latest release
pub fn resolve_download_url(integration_name: &str) -> Result<String, String> {
    Ok(get_latest_release(integration_name)?.download_url)
}

// Function to fetch the contents of a repository at a given path
pub fn fetch_repo_contents(
    owner: &str,
    repo: &str,
    path: &str,
) -> Result<Vec<RepoContent>, String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        owner, repo, path
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "RustApp")
        .send()
        .map_err(|e| format!("Failed to fetch contents: {}", e))?;

    let status = response.status(); // Save before `response.json()`

    if status.is_success() {
        let contents: Vec<RepoContent> = response
            .json()
            .map_err(|e| format!("Failed to parse response (HTTP {}): {}", status, e))?;
        Ok(contents)
    } else {
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Failed to fetch contents. HTTP Status: {} - Error: {}",
            status, error_text
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_known_repo_name() {
        assert_eq!(
            get_repo_name("steam"),
            Some("GOG-Orion/galaxy-integration-steam")
        );
        assert_eq!(
            get_repo_name("ubisoft connect"),
            Some("GOG-Orion/galaxy-integration-ubisoft")
        );
        assert_eq!(get_repo_name("unknown"), None);
    }

    #[test]
    fn prefers_zip_asset() {
        let release = GitHubRelease {
            tag_name: "v1.0.0".to_string(),
            assets: vec![
                ReleaseAsset {
                    name: "readme.txt".to_string(),
                    browser_download_url: "https://example.com/readme.txt".to_string(),
                    content_type: Some("text/plain".to_string()),
                },
                ReleaseAsset {
                    name: "plugin.zip".to_string(),
                    browser_download_url: "https://example.com/plugin.zip".to_string(),
                    content_type: Some("application/zip".to_string()),
                    digest: Some("sha256:abc123".to_string()),
                },
            ],
        };

        let asset = pick_asset(&release).expect("asset should be selected");
        assert_eq!(asset.name, "plugin.zip");
    }

    #[test]
    fn keeps_sha256_digest_when_present() {
        let asset = ReleaseAsset {
            name: "plugin.zip".to_string(),
            browser_download_url: "https://example.com/plugin.zip".to_string(),
            content_type: Some("application/zip".to_string()),
            digest: Some("sha256:abc123".to_string()),
        };

        assert_eq!(
            normalize_asset_digest(asset.digest.as_deref()),
            Some("sha256:abc123".to_string())
        );
        assert!(is_sha256_digest("sha256:abc123"));
        assert!(!is_sha256_digest("md5:abc123"));
    }
}
