// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod github_api;
mod file_manager;
mod integrations;

use github_api::{fetch_repo_contents, resolve_download_url};
use file_manager::{is_process_running, download_file};
use integrations::{verify_latest_version};

use std::fs;
use std::fs::File;
use std::process::Command;
use reqwest::blocking;
use std::io::copy;
use sysinfo::{ProcessExt, System, SystemExt};
use std::path::Path;


// Função principal
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            download_file,
            verify_latest_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
