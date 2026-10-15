use std::rc::Rc;

use clipboard::ContentData;
use eframe::egui;
use tracing::{error, info};

use crate::app::CbEditor;

impl CbEditor {
    pub fn window_editor(&mut self, ctx: &egui::Context) {
        let id = self.editor_entry.id;
        let pre_entry = Rc::clone(&self.editor_entry.text);
        let entry = Rc::make_mut(&mut self.editor_entry.text);

        let mut editor = self.window_flags.editor;
        let mut close = false;
        let mut cancel = false;

        egui::Window::new("element editor")
            .open(&mut editor)
            .show(ctx, |ui| {
                ui.add(egui::TextEdit::multiline(entry));
                ui.horizontal(|ui| {
                    if ui.button("confirm").clicked() {
                        info!("changed: [{id}] {pre_entry} -> {entry}");

                        close = true;
                        cancel = false;
                    }

                    if ui.button("cancel").clicked() {
                        info!("cancelled: [{id}]");

                        close = true;
                        cancel = true;
                    }
                });
            });

        if close {
            if let Some(elem) = self.histries.get_mut(self.editor_entry.id) {
                elem.content = ContentData::Text(Rc::new(entry.to_owned()));
                if let Err(e) = clipboard::set_clipboard_history(&elem.id, &elem.content) {
                    error!("{}", e.message())
                } else {
                    info!("");
                }
            }

            self.editor_entry.reset();
        } else if cancel {
            self.editor_entry.reset();
        }

        self.window_flags.editor = editor && !close;
    }
}
