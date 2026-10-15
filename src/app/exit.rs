use eframe::egui;
use tracing::info;

use crate::app::CbEditor;

impl CbEditor {
    pub fn try_exit(&self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();
        ctx.input(|state| {
            if state.viewport().close_requested() {
                if self.window_flags.is_opening() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    info!("cancel close");
                    return;
                }

                info!("quit");
            }
        });
    }
}
