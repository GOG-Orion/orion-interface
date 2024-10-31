// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::process::Command;
use reqwest;
use serde::Deserialize; // Add this for deserializing JSON
use std::io::{self, Write}; // For writing to files

// Define the structure for integration
#[derive(Deserialize)]
struct Integration {
    name: String,
    image: String,
    download_url: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn lastver(ver: &str) -> String {
    ver.to_string()
}

#[tauri::command]
async fn download_and_install(integration_name: &str) -> Result<String, String> {
    // Read integrations.json
    let integrations: Vec<Integration> = {
        let file = fs::read_to_string("path/to/integrations.json") // Update the path to your integrations.json file
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&file).map_err(|e| e.to_string())?
    };

    // Find the download URL based on the integration name
    let download_url = integrations.iter()
        .find(|integration| integration.name == integration_name)
        .map(|integration| &integration.download_url)
        .ok_or_else(|| "Unknown integration".to_string())?;

    let dest_path = "./downloads/galaxy-integration-steam.zip";

    // Ensure the downloads directory exists
    fs::create_dir_all("./downloads").map_err(|e| e.to_string())?;

    // Download the zip file
    let response = reqwest::get(download_url).await.map_err(|e| e.to_string())?;
    
    // Check if the response is successful
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()));
    }

    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    
    // Create the file and write bytes to it
    let mut file = std::fs::File::create(dest_path).map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;

    // Continue with the extraction and execution of the script
    Command::new("powershell")
        .args(["Expand-Archive", dest_path, "-DestinationPath", "./downloads/galaxy-integration-steam"])
        .output()
        .map_err(|e| e.to_string())?;

    // Execute installation script
    Command::new("powershell")
        .arg("./downloads/galaxy-integration-steam/install.bat")
        .output()
        .map_err(|e| e.to_string())?;

    // Clean temp files
    fs::remove_file(dest_path).map_err(|e| e.to_string())?;
    fs::remove_dir_all("./downloads/galaxy-integration-steam").map_err(|e| e.to_string())?;

    Ok("Installation completed and files deleted.".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, lastver, download_and_install])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
