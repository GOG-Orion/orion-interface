use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "orion_config.json";
const CONFIG_DIR_NAME: &str = "Orion";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub install_root: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            install_root: default_install_root().to_string_lossy().to_string(),
        }
    }
}

fn config_path_override() -> Option<PathBuf> {
    env::var_os("ORION_CONFIG_FILE").map(PathBuf::from)
}

fn install_root_override() -> Option<PathBuf> {
    env::var_os("ORION_INSTALL_ROOT").map(PathBuf::from)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
}

fn install_root_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if cfg!(target_os = "windows") {
        if let Some(local_app_data) = env::var_os("LOCALAPPDATA").map(PathBuf::from) {
            candidates.push(
                local_app_data
                    .join("GOG.com")
                    .join("Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }

        if let Some(app_data) = env::var_os("APPDATA").map(PathBuf::from) {
            candidates.push(
                app_data
                    .join("GOG.com")
                    .join("Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }

        if let Some(program_files) = env::var_os("ProgramFiles").map(PathBuf::from) {
            candidates.push(
                program_files
                    .join("GOG Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }

        if let Some(program_files_x86) = env::var_os("ProgramFiles(x86)").map(PathBuf::from) {
            candidates.push(
                program_files_x86
                    .join("GOG Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }
    } else if cfg!(target_os = "macos") {
        if let Some(home) = home_dir() {
            candidates.push(
                home.join("Library")
                    .join("Application Support")
                    .join("GOG.com")
                    .join("Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }
    } else if let Some(home) = home_dir() {
        if let Some(xdg_data_home) = env::var_os("XDG_DATA_HOME").map(PathBuf::from) {
            candidates.push(
                xdg_data_home
                    .join("GOG.com")
                    .join("Galaxy")
                    .join("Plugins")
                    .join("Installed"),
            );
        }

        candidates.push(
            PathBuf::from("/opt")
                .join("GOG Galaxy")
                .join("Plugins")
                .join("Installed"),
        );

        candidates.push(
            home.join(".local")
                .join("share")
                .join("GOG.com")
                .join("Galaxy")
                .join("Plugins")
                .join("Installed"),
        );
    }

    candidates
}

fn discover_install_root_from_candidates(candidates: Vec<PathBuf>) -> Option<PathBuf> {
    let first_candidate = candidates.first().cloned();

    candidates
        .into_iter()
        .find(|candidate| candidate.exists())
        .or(first_candidate)
}

fn discover_install_root() -> Option<PathBuf> {
    if let Some(path) = install_root_override() {
        return Some(path);
    }

    discover_install_root_from_candidates(install_root_candidates())
}

pub fn config_file_path() -> PathBuf {
    if let Some(path) = config_path_override() {
        return path;
    }

    if let Some(mut base_dir) = tauri::api::path::app_config_dir() {
        base_dir.push(CONFIG_DIR_NAME);
        base_dir.push(CONFIG_FILE_NAME);
        return base_dir;
    }

    PathBuf::from(CONFIG_FILE_NAME)
}

pub fn default_install_root() -> PathBuf {
    if let Some(mut base_dir) = discover_install_root() {
        return base_dir;
    }

    if let Some(mut base_dir) = tauri::api::path::app_data_dir() {
        base_dir.push(CONFIG_DIR_NAME);
        base_dir.push("plugins");
        return base_dir;
    }

    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("plugins")
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    Ok(())
}

pub fn load_config() -> Result<AppConfig, String> {
    let path = config_file_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read configuration file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse configuration file: {}", e))
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_file_path();
    ensure_parent_dir(&path)?;

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write configuration file: {}", e))
}

pub fn get_install_root() -> Result<PathBuf, String> {
    Ok(PathBuf::from(load_config()?.install_root))
}

#[tauri::command]
pub fn get_configuration() -> Result<AppConfig, String> {
    load_config()
}

#[tauri::command]
pub fn set_install_root(install_root: String) -> Result<AppConfig, String> {
    let trimmed = install_root.trim();
    if trimmed.is_empty() {
        return Err("Install root cannot be empty.".to_string());
    }

    let path = PathBuf::from(trimmed);
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create install root: {}", e))?;

    let config = AppConfig {
        install_root: path.to_string_lossy().to_string(),
    };
    save_config(&config)?;
    Ok(config)
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
    fn save_and_load_configuration_roundtrip() {
        let config_file = unique_path("config").with_extension("json");
        let install_root = unique_path("install-root");

        env::set_var("ORION_CONFIG_FILE", &config_file);

        let saved = set_install_root(install_root.to_string_lossy().to_string())
            .expect("configuration should save");
        assert_eq!(saved.install_root, install_root.to_string_lossy());

        let loaded = load_config().expect("configuration should load");
        assert_eq!(loaded.install_root, install_root.to_string_lossy());

        let _ = fs::remove_file(&config_file);
        let _ = fs::remove_dir_all(&install_root);
        env::remove_var("ORION_CONFIG_FILE");
    }

    #[test]
    fn discovers_first_existing_install_root_candidate() {
        let root = unique_path("discover");
        let first = root
            .join("first")
            .join("GOG.com")
            .join("Galaxy")
            .join("Plugins")
            .join("Installed");
        let second = root
            .join("second")
            .join("GOG.com")
            .join("Galaxy")
            .join("Plugins")
            .join("Installed");

        fs::create_dir_all(&second).expect("candidate should be creatable");

        let discovered = discover_install_root_from_candidates(vec![first.clone(), second.clone()]);

        assert_eq!(discovered, Some(second));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn honors_explicit_install_root_override() {
        let override_root = unique_path("override");
        env::set_var("ORION_INSTALL_ROOT", &override_root);

        let resolved = discover_install_root();
        assert_eq!(resolved, Some(override_root.clone()));

        env::remove_var("ORION_INSTALL_ROOT");
    }
}
