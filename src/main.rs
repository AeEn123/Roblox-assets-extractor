#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod logic;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Category {
    Music,
    Sounds,
    Images,
    KTX,
    Rbxm,
}

// Implement `Display` for `Category`
use std::fmt;
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// List assets
    #[arg(short, long, value_name = "CATAGORY")]
    list: Option<Category>,
}

fn main() {
    let args = Cli::parse();

    // Every command after this needs to have the directory detected.
    logic::detect_directory();

    if let Some(category) = args.list {
        // User passed --list
        let tab = category.to_string();

        let cache_directory = {
            let cache_dir = logic::get_cache_directory();
            // Music tab just adds .ogg while other tabs scrape the header files from HTTP to allow all media players to play it
            if tab == "Music" {
                format!("{}/sounds", cache_dir)
            } else {
                format!("{}/http", cache_dir)
            }
        };
        logic::refresh(cache_directory, tab, true); // cli_list_mode is set to true, this will print assets to console
    } else {
        // If nothing passed, run GUI
        match gui::run_gui() {
            Ok(_) => println!("GUI Stopped, exiting program..."),
            Err(e) => println!("GUI failed: {}", e) // Error handling, so the clean up function can still run after the GUI fails
        }
    }
    logic::clean_up(); // Remove the temporary directory if one has been created
}
