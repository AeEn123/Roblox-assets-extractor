#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod logic;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Category {
    Music,
    Sounds,
    Images,
    Rbxl,
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
    /// List assets instead of running the GUI
    #[arg(short, long, value_name = "CATAGORY")]
    list: Option<Category>,
}

fn main() -> eframe::Result<()> {
    let args = Cli::parse();

    logic::detect_directory();

    if let Some(category) = args.list {
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

        if cache_directory == "" {
            panic!("Panic!ed due to safety. cache_directory was blank! Can possibly DELETE EVERYTHING!")
        }
        logic::refresh(cache_directory, tab, true);
        Ok(())
    } else {
        // Otherwise, run the GUI
        
        gui::run_gui()
    }
}
