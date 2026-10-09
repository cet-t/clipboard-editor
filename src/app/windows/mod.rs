mod editor;
mod info;

use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn windows(&mut self, ctx: &egui::Context) {
        self.window_info(ctx);
        self.window_editor(ctx);
    }
}
