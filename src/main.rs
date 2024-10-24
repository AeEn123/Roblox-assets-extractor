#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;

use eframe::{egui, NativeOptions};
use egui_dock::{DockArea, DockState, Style};

lazy_static! {
    static ref CACHE_DIRECTORY: Mutex<String> = Mutex::new(String::new());
    static ref STATUS: Mutex<String> = Mutex::new("Idling".to_owned());
    static ref REQUEST_REPAINT: Mutex<bool> = Mutex::new(false);
    static ref TASK_RUNNING: Mutex<bool> = Mutex::new(false);
    static ref STOP_RUNNING: Mutex<bool> = Mutex::new(false);
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_DIRECTORIES: [&str; 2] = ["%Temp%\\Roblox", "~/.var/app/org.vinegarhq.Sober/cache/sober"];
// For windows and linux (sober)

    fn main() -> eframe::Result<()> {                           
        // Directory detection
        for directory in DEFAULT_DIRECTORIES {
            let resolved_directory = directory
            .replace("%Temp%", &format!("C:\\Users\\{}\\AppData\\Local\\Temp", whoami::username()))
            .replace("~", &format!("/home/{}", whoami::username()));
            // There's probably a better way of doing this... It works though :D

            match fs::metadata(&resolved_directory) { // Directory detection
                Ok(metadata) => {
                    if metadata.is_dir() {
                        let mut cache_dir = CACHE_DIRECTORY.lock().unwrap();
                        *cache_dir = resolved_directory;
                        break;
                    }
                }
                Err(e) => {
                    println!("{directory}: {e}\n↑↑↑Happened during directory detection, you can safely ignore it.");
                }
            }

        }
    let options = NativeOptions::default();
    eframe::run_native(
        &format!("Roblox Assets Extractor v{VERSION}").to_owned(),
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct TabViewer<'a> {
    // passing selected label to TabViewer
    selected: &'a mut Option<usize>,
}

fn delete_all_directory_contents(dir: String) {
    match fs::metadata(dir.clone()) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let running = {
                    let task = TASK_RUNNING.lock().unwrap();
                    task.clone()
                };
                // Stop multiple threads from running
                if running == false {
                    thread::spawn(|| {
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = true;
                        }
                        for entry in fs::read_dir(dir).unwrap() {
                            println!("{}", entry.unwrap().path().display());
                        }
                        for i in 1..10000 {
                            // Update status and request refresh
                            {
                                let mut status = STATUS.lock().unwrap();
                                *status = format!("Deleting files ({i}/10000)");
                                let mut request = REQUEST_REPAINT.lock().unwrap();
                                *request = true;
                            }
                            thread::sleep(Duration::from_millis(1));
                        }
                        { 
                            let mut task = TASK_RUNNING.lock().unwrap();
                            *task = false;
                        }
                    });
                }
            // Error handling just so the program doesn't crash for seemingly no reason
            } else {
                let mut status = STATUS.lock().unwrap();
                *status = format!("Error: check logs for more details.");
            }
        }
        Err(e) => {
            let mut status = STATUS.lock().unwrap();
            *status = format!("Error: '{dir}' is not a valid directory: {e}");
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let cache_directory = {
            let cache_dir = CACHE_DIRECTORY.lock().unwrap();
            // Music tab just adds .ogg while other tabs scrape the header files from HTTP to allow all media players to play it
            if tab == "Music" {
                format!("{}/sounds", cache_dir.clone())
            } else {
                format!("{}/http", cache_dir.clone())
            }
        };
        if cache_directory == "" {
            panic!("Panic!ed due to safety. cache_directory was blank! Can possibly DELETE EVERYTHING!")
        }
        if tab != "Settings" {
            // This is only shown on tabs other than settings (Extracting assets)
            
            // Top UI buttons
            ui.horizontal(|ui| {
                if ui.button("Delete All Files <Del>").clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                    delete_all_directory_contents(cache_directory.to_owned());
                }
                if ui.button("Extract all from this directory <F2>").clicked() || ui.input(|i| i.key_pressed(egui::Key::F2)) {
                    println!("Extract");
                }
                if ui.button("Refresh <F5>").clicked() || ui.input(|i| i.key_pressed(egui::Key::F5)) {
                    println!("Refresh");
                }
            });

            // Scrolling
            let mut scroll_to: Option<usize> = None;
            // Allow the user to select up and down using arrow key
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(selected) = *self.selected {
                    if selected > 1 {
                        *self.selected = Some(selected - 1);
                        scroll_to = Some(selected - 1);
                    }
                } else {
                    *self.selected = Some(1);  // Start at the first label if nothing is selected
                }
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if let Some(selected) = *self.selected {
                    if selected < 1499 { // Stop it from overflowing
                        *self.selected = Some(selected + 1);
                        scroll_to = Some(selected + 1);
                    }
                } else {
                    *self.selected = Some(1);  // Start at the first label if nothing is selected
                }
            }

            // Allow the user to confirm with enter
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Some(selected) = *self.selected {
                    let mut status = STATUS.lock().unwrap();
                    *status = format!("Double clicked {}", selected);
                }
            }
            
            // Scroll area which contains the assets
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                ui.text_style_height(&egui::TextStyle::Body),
                1500,
                |ui, row_range| {
                    for i in row_range {
                    let label_text = format!("Clickable Label {i}");
                    
                    let is_selected = *self.selected == Some(i); // Check if this current one is selected

                    let visuals = ui.visuals();

                    let background_colour = if is_selected {
                        // Use different colours for light mode and dark mode for good contrast
                        if visuals.dark_mode {
                            egui::Color32::from_rgb(0, 50, 100) // This colour matches with dark egui nicely
                        } else {
                            egui::Color32::from_rgb(0, 140, 255) // Not sure if this is the best colour for light theme, do a pull request to change the colour if you want
                        }
                    } else {
                        egui::Color32::TRANSPARENT // No background colour if not selected
                    };
            
                    // Using a rect to allow the user to click across the entire list, not just the text
                    let full_width = ui.available_width();
                    let desired_size = egui::vec2(full_width, ui.text_style_height(&egui::TextStyle::Body)); // Set height to the text style height
                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                    // Draw the background color
                    ui.painter().rect_filled(rect, 0.0, background_colour);

                    // Draw the text
                    ui.painter().text(
                        rect.min + egui::vec2(5.0, 0.0), // Add a bit of padding for the label text
                        egui::Align2::LEFT_TOP,
                        label_text,
                        egui::TextStyle::Body.resolve(ui.style()),
                        ui.visuals().text_color(),
                    );

                    // Handle the click/double click
                    if response.clicked() && is_selected {
                        let mut status = STATUS.lock().unwrap();
                        *status = format!("Double clicked {}", i);
                    } else if response.clicked() {
                        *self.selected = Some(i);
                    }

                    // Handle keyboard scrolling
                    if scroll_to == Some(i) {
                        response.scroll_to_me(Some(egui::Align::Center)) // Align to center to prevent scrolling off the edge
                    }
                }
            });
            

        } else {
            // This is only shown in the settings tab
            ui.label("Settings");
        }

    }
}

struct MyApp {
    tree: DockState<String>,
    selected: Option<usize>, // Used for storing selected state to retain keyboard navigation as seen in the tkinter version
}

impl Default for MyApp {
    fn default() -> Self {
        let tree = DockState::new(vec!["Music".to_owned(), "Sounds".to_owned(), "Images".to_owned(), "RBXL files".to_owned(), "Settings".to_owned()]);

        Self { 
            tree, 
            selected: None,
        }
    }
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Display the status bar at the bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status_message = STATUS.lock().unwrap();
                ui.label(&*status_message);
            });
        });
        
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .draggable_tabs(false)
            .show(ctx, &mut TabViewer { 
                // Pass selected as a mutable referance
                selected: &mut self.selected
            });
        
        {
            // allow for different threads to request refresh
            let mut request = REQUEST_REPAINT.lock().unwrap();
            if *request == true {
                *request = false;
                ctx.request_repaint();
            }
        }
    }
}