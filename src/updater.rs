use std::fs;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::logic;

mod gui;

static URL: &str = "https://api.github.com/repos/AeEn123/Roblox-assets-extractor/releases/latest";

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct Release {
    name: String,
    tag_name: String,
    body: String,
    assets: Vec<Asset>, // List of assets
}

fn clean_version_number(version: &str) -> String {
    version.chars().filter(|c| c.is_digit(10) || *c == '.').collect()
}

fn detect_download_binary(assets: &Vec<Asset>) -> &Asset {
    let os = std::env::consts::OS; // Get the user's operating system to download the correct binary    

    for asset in assets {
        if asset.name.to_lowercase().contains(os) {
            return asset // Return the correct binary based on OS
        }
    }

    eprintln!("Failed to find asset, going for first asset listed.");
    return &assets[0];
}

pub fn download_update(url: &str) {
    let client = Client::new();
    let filename = std::env::current_exe().unwrap().file_name().unwrap().to_string_lossy().to_string();
    let temp_dir = logic::get_temp_dir(true);

    let response = client
        .get(url)
        .header("User-Agent", "Roblox-assets-extractor (Rust)") // Set a User-Agent otherwise it returns 403
        .send();

    match response {
        Ok(data) => {
            match data.bytes() {
                Ok(bytes) => {
                    #[cfg(target_os = "windows")]
                    let path = format!("{}\\{}", temp_dir, filename);
                    #[cfg(target_family = "unix")]
                    let path = format!("{}/{}", temp_dir, filename);
                    match fs::write(path.clone(), bytes) {
                        Ok(_) => logic::set_update_file(path),
                        Err(e) => eprintln!("Failed to write file: {}", e)
                    }
                }
                Err(e) => eprintln!("Failed to parse: {}", e)
            }
        }
        Err(e) => eprintln!("Failed to download: {}", e),
    }
}

pub fn check_for_updates(run_gui: bool, auto_download_update: bool) {
    let client = Client::new();

    let response = client
        .get(URL)
        .header("User-Agent", "Roblox-assets-extractor (Rust)") // Set a User-Agent otherwise it returns 403
        .send();

    match response {
        Ok(data) => {
            let text = data.text().unwrap_or("No text".to_string());
            match serde_json::from_str::<Release>(&text) {
                Ok(json) => {
                    let clean_tag_name = clean_version_number(&json.tag_name);
                    let clean_version = clean_version_number(env!("CARGO_PKG_VERSION"));
                    if clean_tag_name != clean_version {
                        println!("An update is available.");
                        println!("{}", json.name);
                        println!("{}", json.body);

                        let correct_asset = detect_download_binary(&json.assets);

                        if auto_download_update {
                            download_update(&correct_asset.browser_download_url);
                        } else if run_gui {
                            match gui::run_gui(json.body, json.name, correct_asset.browser_download_url.clone()) {
                                Ok(_) => println!("User exited GUI"),
                                Err(e) => println!("GUI failed: {}",e)
                            }
                        }
                    } else {
                        println!("No updates are available.")
                    }
                }
                Err(e) => eprintln!("Failed to parse json: {}", e)
            }
        }
        Err(e) => eprintln!("Failed to check for update: {}", e),
    }
}
