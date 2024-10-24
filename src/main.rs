#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod gui;
mod logic;

fn main() -> eframe::Result {
    // TODO: CLI mode
    // Detect directory and run
    logic::detect_directory();                         
    gui::run_gui()
}