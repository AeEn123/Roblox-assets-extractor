// Used for gui
use eframe::egui;
use native_dialog::{MessageDialog, MessageType};
use egui_dock::{DockArea, NodeIndex, DockState, SurfaceIndex, Style};


use std::collections::HashMap; // Used for input
use crate::logic::{self}; // Used for functionality


const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename


struct TabViewer<'a> {
    // passing selected label to TabViewer
    selected: &'a mut Option<usize>,
    current_tab: &'a mut Option<String>
}


impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let cache_directory = {
            let cache_dir = logic::get_cache_directory();
            // Music tab just adds .ogg while other tabs scrape the header files from HTTP to allow all media players to play it
            if tab == "Music" {
                format!("{}/sounds", cache_dir)
            } else {
                format!("{}/http", cache_dir)
            }
        };


        if tab != "Settings" {
            // This is only shown on tabs other than settings (Extracting assets)

            // Detect if tab changed and do a refresh if so
            if let Some(current_tab) = self.current_tab {
                if current_tab.to_owned() != tab.to_owned() {
                    *self.current_tab = Some(tab.to_owned());
                    logic::refresh(cache_directory.to_owned(), tab.to_owned(), false);
                }
            } else {
                *self.current_tab = Some(tab.to_owned());
                logic::refresh(cache_directory.to_owned(), tab.to_owned(), false);
            }
            
            // GUI logic below here
            
            // Top UI buttons
            ui.horizontal(|ui| {
                if ui.button("Delete this directory <Del>").clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                    // Confirmation dialog
                    let yes = MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title("Confirmation")
                    .set_text("Are you sure you want to delete all files in this directory?")
                    .show_confirm()
                    .unwrap();
            
                    if yes {
                        logic::delete_all_directory_contents(cache_directory.to_owned());
                    }                    
                }
                if ui.button("Extract all of this type <F2>").clicked() || ui.input(|i| i.key_pressed(egui::Key::F2)) {
                    println!("Extract");
                }
                if ui.button("Refresh <F5>").clicked() || ui.input(|i| i.key_pressed(egui::Key::F5)) {
                    logic::refresh(cache_directory.to_owned(), tab.to_owned(), false);
                }
            });

            
            let mut scroll_to: Option<usize> = None; // This is reset every frame, so it doesn't constantly scroll to the same label

            // If the user presses up, decrement the selected value
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                if let Some(selected) = *self.selected {
                    if selected > 0 { // Check if it is larger than 0 otherwise it'll attempt to select non-existant labels
                        *self.selected = Some(selected - 1);
                        scroll_to = Some(selected - 1); // This is also set to the same number, allowing for auto scrolling
                    }
                } else {
                    *self.selected = Some(0);  // Start at the first label if nothing is selected
                }
            }

            let file_list = logic::get_file_list(); // Get file list here since it is used in file list and down arrow

            // If the user presses down, increment the selected value
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                if let Some(selected) = *self.selected {
                    if selected < file_list.len()-1 { // Stop it from overflowing otherwise it'll attempt to select non-existant labels
                        *self.selected = Some(selected + 1);
                        scroll_to = Some(selected + 1); // This is also set to the same number, allowing for auto scrolling
                    }
                } else {
                    *self.selected = Some(1);  // Start at the first label if nothing is selected
                }
            }

            // Allow the user to confirm with enter
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Some(selected) = *self.selected {
                    // Get file name after getting the selected value
                    if let Some(file_name) = file_list.get(selected) {
                        logic::double_click(file_name.to_string());
                    }                   
                }
            }

            
            // File list for assets
            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                ui.text_style_height(&egui::TextStyle::Body),
                file_list.len(),
                |ui, row_range| {
                for i in row_range {
                    if let Some(file_name) = file_list.get(i) {                        
                        let is_selected = *self.selected == Some(i); // Check if this current one is selected

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
                            logic::double_click(file_name.to_string());
                        } else if response.clicked() {
                            *self.selected = Some(i);
                        }

                        // Handle keyboard scrolling
                        if scroll_to == Some(i) {
                            response.scroll_to_me(Some(egui::Align::Center)) // Align to center to prevent scrolling off the edge
                        }
                    }
                }
            });
            

        } else {
            // This is only shown in the settings tab
            ui.heading("Settings");

            // Clear cache description
            ui.label("If it is taking too long to list files and extracting all from a directory, you can clear your roblox cache with the button below. This removes all files from your cache and your roblox client will automatically re-create these files when these assets are needed again.");
            
            // Clear cache button
            if ui.button("Clear roblox cache").clicked() || ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                // Confirmation dialog
                let yes = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title("Confirmation")
                .set_text("Are you sure you want to clear your roblox cache?")
                .show_confirm()
                .unwrap();
        
                if yes {
                    logic::delete_all_directory_contents(logic::get_cache_directory().to_owned());
                }                    
            }     
        }

    }
}

struct MyApp {
    tree: DockState<String>,
    tab_map: HashMap<u32, (SurfaceIndex, NodeIndex, usize)>, // Tab map for keyboard navigation
    selected: Option<usize>, // Used for storing selected state to retain keyboard navigation as seen in the tkinter version
    current_tab: Option<String> // Allows for detecting when the user changes tabs to refresh automatically
}

impl Default for MyApp {
    fn default() -> Self {
        let tree = DockState::new(vec!["Music".to_owned(), "Sounds".to_owned(), "Images".to_owned(), "RBXM files".to_owned(), "KTX files".to_owned(), "Settings".to_owned()]);

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
            current_tab: None
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
        for i in 1..=self.tab_map.len() as u32 {
            if ctx.input(|input| input.key_pressed(egui::Key::from_name(&i.to_string()).expect("Invalid key"))) {
                if let Some(&(surface, node, tab)) = self.tab_map.get(&i) {
                    self.tree.set_active_tab((surface, node, egui_dock::TabIndex(tab)));
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
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
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