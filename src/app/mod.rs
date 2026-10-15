mod body;
mod exit;
mod header;
mod state;
mod windows;

use std::rc::Rc;

use eframe::egui;

pub use state::WindowFlags;

pub struct CbEditor {
    pub window_flags: WindowFlags,
    pub editor_entry: CbEntry,
    pub histries: Vec<clipboard::HistoryEntry>,
}

impl CbEditor {
    pub fn with_histries(histries: Vec<clipboard::HistoryEntry>) -> Self {
        Self {
            histries,
            window_flags: WindowFlags::default(),
            editor_entry: CbEntry::default(),
        }
    }
}

impl eframe::App for CbEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.header(ui);
        ui.separator();
        self.body(ui);
        self.windows(ui.ctx());

        self.try_exit(ui);
    }
}

#[derive(Clone, Default)]
pub struct CbEntry {
    text: std::rc::Rc<String>,
    id: usize,
}

impl CbEntry {
    pub fn set(&mut self, id: usize, text: &std::rc::Rc<String>) {
        self.id = id;
        self.text = Rc::clone(text);
    }

    pub fn reset(&mut self) {
        self.id = 0;
        self.text = Rc::new(String::new());
    }
}
