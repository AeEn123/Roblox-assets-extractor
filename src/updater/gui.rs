use eframe::egui;
use crate::logic;
use crate::updater;
use egui_commonmark::*;
use fluent_bundle::{FluentBundle, FluentResource};
use std::sync::Arc;

struct App {
    locale: FluentBundle<Arc<FluentResource>>,
    cache: CommonMarkCache,
    changelog: String,
    url: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(logic::get_message(&self.locale, "new-updates", None));
            ui.label(logic::get_message(&self.locale, "update-changelog", None));

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                CommonMarkViewer::new()
                    .max_image_width(Some(512))
                    .show(ui, &mut self.cache, &self.changelog);
            });
        });
        egui::TopBottomPanel::bottom("buttons").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Do you want to install the new update?");
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

pub fn run_gui(text: String, url: String) -> eframe::Result {
    let mut args = std::env::args();
    args.next();

    eframe::run_native(
        "Roblox Assets Extractor Updater",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            if let Some(theme) = args.next() {
                if theme == "light" {
                    cc.egui_ctx.set_theme(egui::Theme::Light);
                } else if theme == "dark" {
                    cc.egui_ctx.set_theme(egui::Theme::Dark);
                }
            }

            cc.egui_ctx.style_mut(|style| {
                // Show the url of a hyperlink on hover
                style.url_in_tooltip = true;
            });

            Ok(Box::new(App {
                cache: CommonMarkCache::default(),
                changelog: text,
                locale: logic::get_locale(None),
                url: url
            }))
        }),
    )
}