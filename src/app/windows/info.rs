use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn window_info(&mut self, ctx: &egui::Context) {
        egui::Window::new("app info")
            .open(&mut self.window_flags.info)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("version");
                    ui.label(env!("CARGO_PKG_VERSION"));
                });
            });
    }
}
