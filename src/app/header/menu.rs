use eframe::egui;

use crate::app::CbEditor;

impl CbEditor {
    pub fn header(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|headers: &mut [egui::Ui; 2]| {
            for header in headers.iter_mut() {
                header.set_width(25.);
            }

            headers[0].menu_button("app", |ui| self.header_app(ui));
            headers[1].menu_button("help", |ui| self.header_help(ui));
        });
    }

    fn header_app(&mut self, ui: &mut egui::Ui) {
        // app/
        //  reload
        //  quit

        if ui
            .small_button("reload")
            .on_hover_ui(|ui_hover| {
                ui_hover.label("Reload the application");
            })
            .clicked()
        {
            std::process::exit(0);
        }

        if ui.small_button("quit").clicked() {
            std::process::exit(0);
        }
    }

    fn header_help(&mut self, ui_help: &mut egui::Ui) {
        // help/
        //  info
        //  github

        if ui_help.small_button("info").clicked() {
            self.window_flags.info = true;
        }

        if ui_help
            .small_button("github")
            .on_hover_ui(|ui_hover| {
                ui_hover.label("Open github with new tab on default browser");
            })
            .clicked()
        {
            ui_help.open_url(egui::OpenUrl::new_tab("https://github.com/cet-t"));
        }
    }
}
