// Used for gui
use eframe::egui;
use native_dialog::{MessageDialog, FileDialog, MessageType};
use egui_dock::{DockArea, NodeIndex, DockState, SurfaceIndex, Style};
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;



use std::collections::HashMap; // Used for input
use crate::{logic, updater}; // Used for functionality

mod welcome;
mod settings;


const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename
const ICON: &[u8; 11400] = include_bytes!("../assets/icon.png");
const CONTRIBUTERS: [&str; 4] = [
    "AeEn123",
    "MarcelDev",
    "Vonercent",
    "aaditkumar2009",
];
const DEPENDENCIES: [&str; 13] = [
    "https://github.com/emilk/egui",
    "https://github.com/Adanos020/egui_dock",
    "https://github.com/lampsitter/egui_commonmark",
    "https://github.com/native-dialog-rs/native-dialog-rs",
    "https://github.com/rust-lang-nursery/lazy-static.rs",
    "https://github.com/projectfluent/fluent-rs",
    "https://github.com/1Password/sys-locale",
    "https://github.com/zbraniecki/unic-locale",
    "https://github.com/Stebalien/tempfile",
    "https://github.com/clap-rs/clap",
    "https://github.com/ardaku/whoami",
    "https://github.com/seanmonstar/reqwest",
    "https://github.com/serde-rs/json",
];


struct TabViewer<'a> {
    // passing selected label to TabViewer
    selected: &'a mut Option<usize>,
    current_tab: &'a mut Option<String>,
    renaming: &'a mut bool,
    searching: &'a mut bool,
    search_query: &'a mut String,
    locale: &'a mut FluentBundle<Arc<FluentResource>>,
}

fn double_click(dir: String, value: String, mode: String) {
    let temp_dir = logic::get_temp_dir(true);
    let alias = logic::get_asset_alias(&value);
    let destination = format!("{}/{}", temp_dir, alias); // Join both paths
    let origin = format!("{}/{}", dir, value);
    let new_destination = logic::extract_file(origin, mode, destination.clone(), true);
    if new_destination != "None" {
        let _ = open::that(new_destination); // Open when finished
    }
}

fn add_dependency_credit(dependency: &str, ui: &mut egui::Ui) {
    ui.hyperlink_to(dependency.replace("https://github.com/", ""), dependency);
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

            let mut focus_search_box = false; // Focus the search box toggle for this frame

            // GUI logic below here
            
            // Top UI buttons
            ui.horizontal(|ui| {
                if ui.button(logic::get_message(self.locale, "button-search", None)).clicked() || ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) {
                    *self.searching = !*self.searching;
                    focus_search_box = true;          
                }
                if ui.button(logic::get_message(self.locale, "button-rename", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::F2)) {
                    *self.renaming = !*self.renaming;               
                }
                if ui.button(logic::get_message(self.locale, "button-delete-this-dir", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) && !*self.renaming { // del key used for editing, don't allow during editing
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
                            logic::extract_dir(cache_directory.to_string(), path.to_string_lossy().to_string(), tab.to_string(), file_list.clone(), false,logic::get_config_bool("use_alias").unwrap_or(false));
                        }
                    }
                }
                if ui.button(logic::get_message(self.locale, "button-refresh", None)).clicked() || ui.input(|i| i.key_pressed(egui::Key::F5)) {
                    logic::refresh(cache_directory.to_owned(), tab.to_owned(), false, false);
                }
            });

            
            let mut scroll_to: Option<usize> = None; // This is reset every frame, so it doesn't constantly scroll to the same label
            let mut none_selected: bool = false; // Used to scroll to the first value shown when none is selected
            
            // Only allow navigation of the user is not renaming
            if !*self.renaming {
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
            }


            let mut navigation_accepted: bool = false; // Used to check if the selected label is available to accept the keyboard navigation
            let mut first_iterated: bool = false; // Used to track if the first entry iterated.

            let file_list = if *self.searching {
                let old_search_query = self.search_query.clone();

                let response = ui.text_edit_singleline(self.search_query);

                if focus_search_box {
                    response.request_focus();
                }

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    *self.searching = false; // Remove the search bar when the use presses escape
                }

                if *self.search_query != old_search_query {
                    logic::filter_file_list(self.search_query.clone());
                }
                logic::get_filtered_file_list()
            } else {
                file_list
            };

            // File list for assets
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                ui.text_style_height(&egui::TextStyle::Body),
                file_list.len(),
                |ui, row_range| {
                for i in row_range {
                    if let Some(file_name) = file_list.get(i) {
                        let alias = logic::get_asset_alias(&file_name);

                        let is_selected  = if none_selected && first_iterated { // Selecting the very first causes some issues
                            *self.selected = Some(i); // If there is none selected, Set selected and return true
                            none_selected = false; // Will select everything if this is not set to false immediately
                            true
                        } else {
                            *self.selected == Some(i) // Check if this current one is selected
                        };

                        // Draw the text
                        if is_selected && *self.renaming {
                            let mut mutable_name = alias.to_string();
                            let response = ui.text_edit_singleline(&mut mutable_name);

                            if mutable_name != *alias {
                                logic::set_asset_alias(&file_name, &mutable_name);
                            }

                            if response.lost_focus() {
                                *self.renaming = false;
                                if mutable_name == "" {
                                    logic::set_asset_alias(&file_name, &file_name); // Set it to file name if blank
                                }
                            } else {
                                response.request_focus(); // Request focus if it hasn't lost focus
                            }
                        } else {
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

                            // Handle the click/double click
                            if response.clicked() && is_selected {
                                double_click(cache_directory.clone(), file_name.to_string(), tab.to_string());
                            } else if response.clicked() && !*self.renaming {
                                *self.selected = Some(i);
                            }

                            // Handle keyboard scrolling
                            if scroll_to == Some(i) {
                                navigation_accepted = true;
                                response.scroll_to_me(Some(egui::Align::Center)) // Align to center to prevent scrolling off the edge
                            }

                            ui.painter().text(
                                rect.min + egui::vec2(5.0, 0.0), // Add a bit of padding for the label text
                                egui::Align2::LEFT_TOP,
                                alias, // Text is the file name or the alias. Alias is user-defined
                                egui::TextStyle::Body.resolve(ui.style()),
                                text_colour,
                            );
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

            settings::actions(ui, self.locale);
            settings::cache_dir_management(ui, self.locale);
            settings::behavior(ui, &self.locale);  
            settings::updates(ui, self.locale);

            if settings::language(ui, self.locale) {
                // This returns true if the locales need to be refreshed
                *self.locale = logic::get_locale(None);
            }
            
        } else {
            // This is only shown in the about tab
            ui.heading("Roblox Assets Extractor");

            let mut args = fluent_bundle::FluentArgs::new();
            args.set("version", VERSION);

            ui.label(logic::get_message(self.locale, "version", Some(&args)));

            ui.separator();

            ui.heading(logic::get_message(self.locale, "contributers", None));
            for contributer in CONTRIBUTERS {
                ui.hyperlink_to(format!("@{}",contributer), format!("https://github.com/{}", contributer));
            }

            ui.separator();

            ui.heading(logic::get_message(self.locale, "dependencies", None));
            for dependency in DEPENDENCIES {
                add_dependency_credit(dependency, ui);
            } 

        }
    }
}

struct MyApp {
    tree: DockState<String>,
    tab_map: HashMap<u32, (SurfaceIndex, NodeIndex, usize)>, // Tab map for keyboard navigation
    selected: Option<usize>, // Used for storing selected state to retain keyboard navigation as seen in the tkinter version
    current_tab: Option<String>, // Allows for detecting when the user changes tabs to refresh automatically
    renaming: bool,
    searching: bool,
    search_query: String,
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
            renaming: false,
            searching: false,
            search_query: "".to_owned(),
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
                renaming: &mut self.renaming,
                searching: &mut self.searching,
                search_query: &mut self.search_query,
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

pub fn run_gui() {
    // If the user is not welcomed before, welcome them
    if !logic::get_config_bool("welcomed").unwrap_or(false) {
        let _ = welcome::run_gui();
    }
    
    // Only run GUI after user has been welcomed
    if logic::get_config_bool("welcomed").unwrap_or(true) {
        // Check for updates when running GUI
        if logic::get_config_bool("check_for_updates").unwrap_or(false) {
            updater::check_for_updates(true, logic::get_config_bool("automatically_install_updates").unwrap_or(false));
        }

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_icon(
                    eframe::icon_data::from_png_bytes(&ICON[..])
                        .expect("Failed to load icon"),
                ),
            ..Default::default()
        };
        
        let result = eframe::run_native(
            &format!("Roblox Assets Extractor v{VERSION}").to_owned(),
            options,
            Box::new(|_cc| Ok(Box::<MyApp>::default())),
        );

        if result.is_err() {
            eprintln!("GUI failed: {}", result.unwrap_err())
        }
    }

}