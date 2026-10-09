mod body;
mod exit;
mod header;
mod state;
mod windows;

use eframe::egui;

use crate::app::state::WindowFlags;

#[derive(Default)]
pub struct CbEditor {
    pub window_flags: WindowFlags,
}

impl eframe::App for CbEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.header(ui);
        self.body(ui);
        self.windows(ui.ctx());

        self.try_exit(ui);
    }
}
