use reqwest::blocking::Client;
use std::thread;
use serde::Deserialize;

mod gui;

static URL: &str = "https://api.github.com/repos/AeEn123/Roblox-assets-extractor/releases/latest";

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    body: String,
    assets: Vec<Asset>, // List of assets
}

fn clean_version_number(version: &str) -> String {
    version.chars().filter(|c| c.is_digit(10) || *c == '.').collect()
}

pub fn check_for_updates(run_gui: bool) {
    thread::spawn(move || {
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
                            println!("{}", json.body);
                            if run_gui {
                                gui::run_gui(json.body);
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to parse json: {}", e)
                }
            }
            Err(e) => eprintln!("Failed to check for update: {}", e),
        }
    });
}
