// Used for gui
use eframe::egui;
use native_dialog::{MessageDialog, FileDialog, MessageType};

const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename

pub struct MyApp {}

impl eframe::App for MyApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}
pub fn run_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../../assets/icon.png")[..])
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