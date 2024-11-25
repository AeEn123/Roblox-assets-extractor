use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

pub struct MyApp {
    changelog: String,
}

impl MyApp {
    pub fn new(changelog: String) -> Self {
        Self { changelog }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Changelog");

            ui.separator();

            // Render the changelog using egui_commonmark
            // let mut viewer = CommonMarkViewer::default();
            // let mut cache = CommonMarkCache::default();
            // CommonMarkViewer::new()
            // .show(
            //     ui,
            //     &mut self.cache,
            //     &self.pages[self.curr_tab.unwrap_or(0)].content,
            // );

            ui.separator();
        });
    }
}

// Run the application with the changelog
pub fn run(changelog: String) -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Changelog Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(changelog)))),
    )
}
