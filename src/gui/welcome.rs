use eframe::egui;
use crate::gui::settings;
use crate::logic;
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;


const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename

pub struct MyApp {
    locale: FluentBundle<Arc<FluentResource>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(logic::get_message(&self.locale, "welcome", None));

            if settings::language(ui, &self.locale) {
                // This returns true if the locales need to be refreshed
                self.locale = logic::get_locale(None);
            }

            ui.separator();

            // Config will be mutated as part of checkbox user interaction.
            let mut config = logic::get_config();
            settings::updates(ui, &self.locale);

            logic::set_config(config); // Update config to new one

            if ui.button(logic::get_message(&self.locale, "button-finish", None)).clicked() {
                logic::set_config_bool("welcomed", true);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            locale: logic::get_locale(None)
        }
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