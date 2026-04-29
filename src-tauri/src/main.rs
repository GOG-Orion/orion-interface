// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_config;
mod file_manager;
mod github_api;
mod integration_version;

use app_config::{get_configuration, set_install_root};
use file_manager::download_file;
use integration_version::verify_latest_version;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_configuration,
            download_file,
            set_install_root,
            verify_latest_version
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
