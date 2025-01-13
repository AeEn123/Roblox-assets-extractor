use eframe::egui;
use crate::{log, logic};
use crate::logic::get_config_string;
use crate::updater;
use egui_commonmark::*;
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;

struct App {
    locale: FluentBundle<Arc<FluentResource>>,
    cache: CommonMarkCache,
    changelog: String,
    url: String,
    name: String,
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
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, text: String, name: String, url: String) -> Self {
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

        // Set system theme from config
        match get_config_string("theme").unwrap_or("system".to_owned()).as_str() {
            "dark" => cc.egui_ctx.set_theme(egui::Theme::Dark),
            "light" => cc.egui_ctx.set_theme(egui::Theme::Light),
            _ => ()
        }

        // Return self
        Self {
            cache: CommonMarkCache::default(),
            changelog: text,
            locale: logic::get_locale(None),
            name: name,
            url: url
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(logic::get_message(&self.locale, "new-updates", None));
            ui.label(logic::get_message(&self.locale, "update-changelog", None));

            ui.separator();

            ui.heading(&self.name);

            egui::ScrollArea::vertical().show(ui, |ui| {
                CommonMarkViewer::new()
                    .max_image_width(Some(512))
                    .show(ui, &mut self.cache, &self.changelog);
            });
        });
        egui::TopBottomPanel::bottom("buttons").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(logic::get_message(&self.locale, "download-update-question", None));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button(logic::get_message(&self.locale, "button-no", None)).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button(logic::get_message(&self.locale, "button-yes", None)).clicked() {
                        updater::download_update(&self.url);
                        logic::run_install_script(true);
                    }
                })
            });

        });
    }
}

pub fn run_gui(text: String, name: String, url: String) -> eframe::Result {
    eframe::run_native(
        &format!("Roblox Assets Extractor Updater v{}", VERSION),
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            cc.egui_ctx.style_mut(|style| {
                // Show the url of a hyperlink on hover
                style.url_in_tooltip = true;
            });

            Ok(Box::new(App::new(cc, text, name, url)))
        }),
    )
}