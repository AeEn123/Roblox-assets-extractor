#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod logic;
mod updater;

use clap::{Parser, ValueEnum};

// CLI stuff
#[derive(ValueEnum, Clone, Debug)]
enum Category {
    Music,
    Sounds,
    Images,
    Ktx,
    Rbxm,
}

// Implement `Display` for `Category`

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// List assets
    #[arg(short, long)]
    list: bool,

    /// Set mode
    #[arg(short, long, value_name = "CATAGORY")]
    mode: Option<Category>,

    /// Extract asset
    #[arg(short, long)]
    extract: Option<String>,

    /// Define a destination path
    #[arg(short, long)]
    dest: Option<String>,

    
}

fn main() {
    let args = Cli::parse();

    if args.list {
        if let Some(category) = args.mode {
            let tab = category.to_string().to_lowercase();

            let cache_directory = {
                let cache_dir = logic::get_cache_directory();
                // Music tab just adds .ogg while other tabs scrape the header files from HTTP to allow all media players to play it
                if tab == "music" {
                    format!("{}/sounds", cache_dir)
                } else {
                    format!("{}/http", cache_dir)
                }
            };
            logic::refresh(cache_directory, tab, true, true); // cli_list_mode is set to true, this will print assets to console
        } else {
            // Not enough arguments
            eprintln!("Category argument required for list mode! --help for details.")
        }


    } else if let Some(asset) = args.extract  {
        println!("{}", asset)
    } else {
        // If nothing passed, run GUI
        gui::run_gui();
    }
    
    if !logic::run_install_script(false) {
        // Only run if the install script hasn't ran
        logic::clean_up(); // Remove the temporary directory if one has been created
    }
    
}
