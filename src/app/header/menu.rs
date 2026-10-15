use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("app", |ui| self.header_app(ui));
            ui.menu_button("setting", |ui| self.header_setting(ui));
            ui.menu_button("help", |ui| self.header_help(ui));
        });
    }

    fn header_app(&mut self, ui: &mut egui::Ui) {
        // app/
        //  reload
        //  quit

        #[allow(clippy::collapsible_if)]
        if ui
            .small_button("reload")
            .on_hover_ui(|ui| {
                ui.label("Reload the application");
            })
            .clicked()
        {
            tracing::info!("reload clicked");

            if let Ok(histries) = clipboard::get_clipboard_history() {
                self.histries = histries;
            }
        }

        if ui.small_button("quit").clicked() {
            tracing::info!("quit clicked");

            std::process::exit(0);
        }
    }

    fn header_setting(&mut self, ui: &mut egui::Ui) {
        // setting/
        //  preference

        if ui.small_button("preference").clicked() {
            tracing::info!("preference clicked");

            self.window_flags.setting = true;
        }
    }

    fn header_help(&mut self, ui: &mut egui::Ui) {
        // help/
        //  info
        //  github

        if ui.small_button("info").clicked() {
            tracing::info!("info clicked");

            self.window_flags.info = true;
        }

        if ui
            .add_enabled(false, egui::Button::new("source").small())
            .on_hover_ui(|ui| {
                ui.label("Open github with new tab on default browser");
            })
            .clicked()
        {
            tracing::info!("github clicked");

            ui.open_url(egui::OpenUrl::new_tab("https://github.com/cet-t"));
        }
    }
}
