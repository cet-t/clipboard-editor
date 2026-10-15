use std::rc::Rc;

use clipboard::ContentData;
use eframe::egui;
use tracing::error;

use crate::app::CbEditor;

impl CbEditor {
    pub fn body(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (i, elem) in self.histries.iter_mut().enumerate() {
                    let elem = match &elem.content {
                        ContentData::Text(v) => Rc::clone(v),
                        // ContentData::Image {mime, bytes} => {},
                        _ => Rc::new(format!("{i}: Failed to load")),
                    };

                    ui.horizontal(|ui| {
                        if ui.button(format!("{}", i + 1)).clicked() {
                            error!("{i} clicked");

                            self.editor_entry.set(i, &elem);
                            self.window_flags.editor = true;
                        }

                        ui.separator();

                        ui.label(Rc::unwrap_or_clone(Rc::clone(&elem)));
                    });
                }
            });
    }
}
