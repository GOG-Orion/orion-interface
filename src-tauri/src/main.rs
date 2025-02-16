// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod github_api;
mod file_manager;
mod integration_version;

use file_manager::download_file;
use integration_version::verify_latest_version;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_file,
            verify_latest_version
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
