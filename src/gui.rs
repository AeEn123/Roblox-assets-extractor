// Used for gui
use eframe::egui;
use fluent::FluentArgs;
use native_dialog::{MessageDialog, FileDialog, MessageType};
use egui_dock::{DockArea, NodeIndex, DockState, SurfaceIndex, Style};
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;



use std::collections::HashMap; // Used for input
use crate::logic; // Used for functionality


const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename
const ICON: &[u8; 11400] = include_bytes!("../assets/icon.png");


struct TabViewer<'a> {
    // passing selected label to TabViewer
    selected: &'a mut Option<usize>,
    current_tab: &'a mut Option<String>,
    locale: &'a mut FluentBundle<Arc<FluentResource>>,
}

fn double_click(dir: String, value: String, mode: String) {
    let temp_dir = logic::get_temp_dir(true);
    let destination = format!("{}/{}", temp_dir, value); // Join both paths
    let origin = format!("{}/{}", dir, value);
    let new_destination = logic::extract_file(origin, mode, destination.clone(), true);
    if new_destination != "None" {
        let _ = open::that(new_destination); // Open when finished
    }
}


impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        logic::get_message(self.locale, &*tab, None).into()
        
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let cache_directory = {
            let cache_dir = logic::get_cache_directory();
            // Music tab just adds .ogg while other tabs scrape the header files from HTTP to allow all media players to play it
            if tab == "music" {
                format!("{}/sounds", cache_dir)
            } else {
                format!("{}/http", cache_dir)
            }
        };

        let file_list = logic::get_file_list(); // Get the file list as it is used throughout the GUI

        if tab != "settings" && tab != "about" {
            // This is only shown on tabs other than settings (Extracting assets)

            // Detect if tab changed and do a refresh if so
            if let Some(current_tab) = self.current_tab {
                if current_tab.to_owned() != tab.to_owned() {
                    *self.current_tab = Some(tab.to_owned());
                    logic::refresh(cache_directory.to_owned(), tab.to_owned(), false, false);
                }
            } else {
                *self.current_tab = Some(tab.to_owned());
                logic::refresh(cache_directory.to_owned(), tab.to_owned(), false, false);
            }
                        
            // GUI logic below here
            
            // Top UI buttons
            ui.horizontal(|ui| {
                if ui.button(logic::get_message(self.locale, "button-delete-this-dir", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                    // Confirmation dialog
                    let yes = MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title(&logic::get_message(self.locale, "confirmation-delete-confirmation-title", None))
                    .set_text(&logic::get_message(self.locale, "confirmation-delete-confirmation-description", None))
                    .show_confirm()
                    .unwrap();
                
                    if yes {
                        logic::delete_all_directory_contents(cache_directory.to_owned());
                    }                    
                }
                if ui.button(logic::get_message(self.locale, "button-extract-type", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::F3)) {
                    let mut no = logic::get_list_task_running();

                    // Confirmation dialog, the program is still listing files
                    if no {
                        // NOT result, will become false if user clicks yes
                        no = !MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title(&logic::get_message(self.locale, "confirmation-filter-confirmation-title", None))
                        .set_text(&logic::get_message(self.locale, "confirmation-filter-confirmation-description", None))
                        .show_confirm()
                        .unwrap();
                    }

                    // The user either agreed or the program is not listing files
                    if !no {
                        let option_path = FileDialog::new()
                        .show_open_single_dir()
                        .unwrap();

                        // If the user provides a directory, the program will extract the assets to that directory
                        if let Some(path) = option_path {
                            logic::extract_dir(cache_directory.to_string(), path.to_string_lossy().to_string(), tab.to_string(), file_list.clone(), false);
                        }
                    }
                }
                if ui.button(logic::get_message(self.locale, "button-refresh", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::F5)) {
                    logic::refresh(cache_directory.to_owned(), tab.to_owned(), false, false);
                }
            });

            
            let mut scroll_to: Option<usize> = None; // This is reset every frame, so it doesn't constantly scroll to the same label
            let mut none_selected: bool = false; // Used to scroll to the first value shown when none is selected
            // If the user presses up, decrement the selected value
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(selected) = *self.selected {
                    if selected > 0 { // Check if it is larger than 0 otherwise it'll attempt to select non-existant labels
                        *self.selected = Some(selected - 1);
                        scroll_to = Some(selected - 1); // This is also set to the same number, allowing for auto scrolling
                    }
                } else {
                    none_selected = true // Select the first visible entry
                }
            }

            // If the user presses down, increment the selected value
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if let Some(selected) = *self.selected {
                    if selected < file_list.len()-1 { // Stop it from overflowing otherwise it'll attempt to select non-existant labels
                        *self.selected = Some(selected + 1);
                        scroll_to = Some(selected + 1); // This is also set to the same number, allowing for auto scrolling
                    }
                } else {
                    none_selected = true // Select the first visible entry
                }
            }

            // Allow the user to confirm with enter
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Some(selected) = *self.selected {
                    // Get file name after getting the selected value
                    if let Some(file_name) = file_list.get(selected) {
                        double_click(cache_directory.clone(), file_name.to_string(), tab.to_string());
                    }                   
                }
            }

            let mut navigation_accepted: bool = false; // Used to check if the selected label is available to accept the keyboard navigation
            let mut first_iterated: bool = false; // Used to track if the first entry iterated.

            // File list for assets
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                ui.text_style_height(&egui::TextStyle::Body),
                file_list.len(),
                |ui, row_range| {
                for i in row_range {
                    if let Some(file_name) = file_list.get(i) {

                        let is_selected  = if none_selected && first_iterated { // Selecting the very first causes some issues
                            *self.selected = Some(i); // If there is none selected, Set selected and return true
                            none_selected = false; // Will select everything if this is not set to false immediately
                            true
                        } else {
                            *self.selected == Some(i) // Check if this current one is selected
                        };

                        let visuals = ui.visuals();

                        // Highlight the background when selected
                        let background_colour = if is_selected {
                            visuals.selection.bg_fill // Primary colour
                        } else {
                            egui::Color32::TRANSPARENT // No background colour
                        };

                        // Make the text have more contrast when selected
                        let text_colour = if is_selected {
                            visuals.strong_text_color() // Brighter
                        } else {
                            visuals.text_color() // Normal
                        };

                
                        // Using a rect to allow the user to click across the entire list, not just the text
                        let full_width = ui.available_width();
                        let desired_size = egui::vec2(full_width, ui.text_style_height(&egui::TextStyle::Body)); // Set height to the text style height
                        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                        // Draw the background colour
                        ui.painter().rect_filled(rect, 0.0, background_colour);

                        // Draw the text
                        ui.painter().text(
                            rect.min + egui::vec2(5.0, 0.0), // Add a bit of padding for the label text
                            egui::Align2::LEFT_TOP,
                            file_name, // Text is the file name
                            egui::TextStyle::Body.resolve(ui.style()),
                            text_colour,
                        );

                        // Handle the click/double click
                        if response.clicked() && is_selected {
                            double_click(cache_directory.clone(), file_name.to_string(), tab.to_string());
                        } else if response.clicked() {
                            *self.selected = Some(i);
                        }

                        // Handle keyboard scrolling
                        if scroll_to == Some(i) {
                            navigation_accepted = true;
                            response.scroll_to_me(Some(egui::Align::Center)) // Align to center to prevent scrolling off the edge
                        }

                        first_iterated = true // Set first_iterated to true to show that the first one has iterated, no difference if it happens for all
                    }
                }
            });

            if !navigation_accepted && scroll_to.is_some() {
                // If the keyboard navigation wasn't accepted and there is keyboard navigation then...
                *self.selected = None; // Set the selected to none, so it selects something on-screen
            }

        } else if tab == "settings" {
            // This is only shown in the settings tab
            ui.heading(logic::get_message(self.locale, "actions", None));

            // Clear cache description
            ui.label(logic::get_message(self.locale, "clear-cache-description", None));
            
            // Clear cache button
            if ui.button(logic::get_message(self.locale, "button-clear-cache", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                // Confirmation dialog
                let yes = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title(&logic::get_message(self.locale, "confirmation-clear-cache-title", None))
                .set_text(&logic::get_message(self.locale, "confirmation-clear-cache-description", None))
                .show_confirm()
                .unwrap();
        
                if yes {
                    logic::delete_all_directory_contents(logic::get_cache_directory().to_owned());
                }                    
            }

            // Extract all description
            ui.label(logic::get_message(self.locale, "extract-all-description", None));

            // Extract all button
            if ui.button(&logic::get_message(self.locale, "button-extract-all", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::F3)) {
                let mut no = logic::get_list_task_running();
            
                // Confirmation dialog, the program is still listing files
                if no {
                    // NOT result, will become false if user clicks yes
                    no = !MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title(&logic::get_message(self.locale, "confirmation-filter-confirmation-title", None))
                    .set_text(&logic::get_message(self.locale, "confirmation-filter-confirmation-description", None))
                    .show_confirm()
                    .unwrap();
                }
            
                // The user either agreed or the program is not listing files
                if !no {
                    let option_path = FileDialog::new()
                    .show_open_single_dir()
                    .unwrap();
            
                    // If the user provides a directory, the program will extract the assets to that directory
                    if let Some(path) = option_path {
                        logic::extract_all( path.to_string_lossy().to_string(), false)
                    }
                }
            }

            ui.label(logic::get_message(self.locale, "custom-cache-dir-description", None));

            let mut args = FluentArgs::new();
            args.set("directory", logic::get_cache_directory());

            ui.label(logic::get_message(self.locale, "cache-directory", Some(&args)));

            ui.horizontal(|ui| {
                if ui.button(logic::get_message(self.locale, "button-change-cache-dir", None)).clicked() {
                    let option_path = FileDialog::new()
                    .show_open_single_dir()
                    .unwrap();
            
                    // If the user provides a directory, the program will change the cache directory to the new one
                    if let Some(path) = option_path {
                        // Validation checks
                        match logic::validate_directory(&path.to_string_lossy().to_string()) {
                            Ok(directory) => {
                                logic::set_config_value("cache_directory", &directory);
                                logic::set_cache_directory(logic::detect_directory()); // Set directory to new one
                            }
                            Err(_) => {
                                MessageDialog::new()
                                .set_type(MessageType::Info)
                                .set_title(&logic::get_message(self.locale, "error-invalid-directory-title", None))
                                .set_text(&logic::get_message(self.locale, "error-invalid-directory-description", None))
                                .show_alert()
                                .unwrap();
                            }
                        }
                    }
                }
                if ui.button(logic::get_message(self.locale, "button-reset-cache-dir", None)).clicked() {
                    logic::set_config_value("cache_directory", "no directory set"); // Clear directory in config
                    logic::set_cache_directory(logic::detect_directory()); // Set it back to default
                }
            });

            ui.separator();
            
            ui.heading(logic::get_message(self.locale, "updates", None));
            ui.label(logic::get_message(self.locale, "no-function", None));


            // Config will be mutated as part of checkbox user interaction.
            let mut config = logic::get_config();

            // Get check_for_updates and automatically_install_updates into a variable for use for checkboxes
            let mut check_for_updates = if let Some(result) = config["check_for_updates"].as_bool() {
                result
            } else {
                true
            };

            let mut automatically_install_updates = if let Some(result) = config["automatically_install_updates"].as_bool() {
                result
            } else {
                false
            };
            

            ui.checkbox(&mut check_for_updates, logic::get_message(self.locale, "check-for-updates", None));
            ui.checkbox(&mut automatically_install_updates, logic::get_message(self.locale, "automatically-install-updates", None));

            // Add them to the config again
            config["check_for_updates"] = check_for_updates.into();
            config["automatically_install_updates"] = automatically_install_updates.into();

            logic::set_config(config); // Update config to new one


            
        } else {
            // This is only shown in the about tab
            ui.heading("Roblox Assets Extractor");
            

        }
    }
}

struct MyApp {
    tree: DockState<String>,
    tab_map: HashMap<u32, (SurfaceIndex, NodeIndex, usize)>, // Tab map for keyboard navigation
    selected: Option<usize>, // Used for storing selected state to retain keyboard navigation as seen in the tkinter version
    current_tab: Option<String>, // Allows for detecting when the user changes tabs to refresh automatically
    locale: FluentBundle<Arc<FluentResource>>
}

impl Default for MyApp {
    fn default() -> Self {
        let tree = DockState::new(vec!["music".to_owned(), "sounds".to_owned(), "images".to_owned(), "rbxm-files".to_owned(), "ktx-files".to_owned(), "settings".to_owned(), "about".to_owned()]);

        // Tab map for keyboard navigation
        let mut tab_map = HashMap::new();

        let surface = SurfaceIndex(0);
        let node = NodeIndex(0);
        for (i, _) in tree.iter_all_tabs().enumerate() {
            tab_map.insert((i as u32) + 1, (surface, node, i));
        }

        Self { 
            tree, 
            tab_map,
            selected: None,
            current_tab: None,
            locale: logic::get_locale(None),
        }
    }
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Display the status bar at the bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.add(egui::ProgressBar::new(logic::get_progress()).text(logic::get_status()));
        });

        // Switch tabs with keyboard input (num keys)
        if ctx.input(|input| input.modifiers.ctrl || input.modifiers.alt) {
            for i in 1..=self.tab_map.len() as u32 {
                if ctx.input(|input| input.key_pressed(egui::Key::from_name(&i.to_string()).expect("Invalid key"))) {
                    if let Some(&(surface, node, tab)) = self.tab_map.get(&i) {
                        self.tree.set_active_tab((surface, node, egui_dock::TabIndex(tab)));
                    }
                }
            }
        }


        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .draggable_tabs(false)
            .show(ctx, &mut TabViewer { 
                // Pass selected as a mutable referance
                selected: &mut self.selected,
                current_tab: &mut self.current_tab,
                locale: &mut self.locale,
            });
        
        {
            // Allow for different threads to request refresh
            if logic::get_request_repaint() {
                ctx.request_repaint_after(std::time::Duration::from_millis(250)); // Delay added here to prevent refreshes from stopping
            }
        }
    }
}

pub fn run_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(&ICON[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    
    eframe::run_native(
        &format!("Roblox Assets Extractor v{VERSION}").to_owned(),
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}