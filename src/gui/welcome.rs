use eframe::egui;
use crate::{gui::settings, log};
use crate::logic;
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;


const VERSION: &str = env!("CARGO_PKG_VERSION"); // Get version for use in the filename

pub struct MyApp {
    first_frame: bool,
    locale: FluentBundle<Arc<FluentResource>>,
}

fn detect_japanese_font() -> Option<String> {
    let font_dirs = ["C:\\Windows\\Fonts\\msgothic.ttc", "/usr/share/fonts/noto-cjk/NotoSerifCJK-Regular.ttc", "~/.local/share/fonts/noto-cjk/NotoSerifCJK-Regular.ttc", "~/.fonts/noto-cjk/NotoSerifCJK-Regular.ttc"];
    
    for font in font_dirs {
        let resolved_font = logic::resolve_path(&font);
        match std::fs::metadata(&resolved_font) {
            Ok(metadata) => {
                if metadata.is_file() {
                    log::info(&format!("{}: valid", resolved_font));
                    return Some(resolved_font);
                }
            }
            Err(e) => {
                log::warn(&format!("{}: invalid - {}", resolved_font, e))
            }
        }
        
    };
    return None;
}

// https://users.rust-lang.org/t/is-posible-egui-change-fonts-to-japanese-how/59662/5
impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Custom font install
        // 1. Create a `FontDefinitions` object.
        let mut font = egui::FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        // 2. register the font content with a name.
        match detect_japanese_font() {
            Some(font_path) => {
                match std::fs::read(font_path) {
                    Ok(bytes) => {
                        font.font_data.insert("japanese".to_owned(),egui::FontData::from_owned(bytes).into());
                        font.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("japanese".to_owned());
                        font.families.get_mut(&egui::FontFamily::Proportional).unwrap().push("japanese".to_owned());
                        // 3. Configure context with modified `FontDefinitions`.
                        cc.egui_ctx.set_fonts(font);
                    }
                    Err(e) => {
                        log::error(&format!("Error loading Japanese fonts: {e}"))
                    }
                }
            }
            None => {
                log::warn("No Japanese fonts detected, Japanese characters will not render.")
            }
        }    
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(logic::get_message(&self.locale, "welcome", None));

            if settings::language(ui, &self.locale) {
                // This returns true if the locales need to be refreshed
                self.locale = logic::get_locale(None);
            }
            settings::behavior(ui, &self.locale);
            settings::updates(ui, &self.locale);
            if self.first_frame {
                logic::set_config_value("welcomed", false.into());
                self.first_frame = false
            }


            if ui.button(logic::get_message(&self.locale, "button-finish", None)).clicked() {
                logic::set_config_value("welcomed", true.into());
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }


        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            first_frame: true,
            locale: logic::get_locale(None),
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
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
    
}