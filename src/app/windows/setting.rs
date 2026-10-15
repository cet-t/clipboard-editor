use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn window_setting(&mut self, ctx: &egui::Context) {
        egui::Window::new("setting")
            .resizable(false)
            .open(&mut self.window_flags.setting)
            .show(ctx, |ui| {
                ui.label("not yet");
                ui.label("not yet");
            });
    }
}
