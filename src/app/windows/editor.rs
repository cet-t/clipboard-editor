use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn window_editor(&mut self, ctx: &egui::Context) {
        egui::Window::new("editor")
            .open(&mut self.window_flags.editor)
            .show(ctx, |_ui| {});
    }
}
