mod app;
mod localisation;

use std::collections::HashMap;

use tracing::error;
use unic_langid::langid;

use crate::app::CbEditor;

fn init_localise() -> crate::localisation::Locales {
    let mut locales = HashMap::new();
    crate::load_locales!(locales, ["en", "ja"]);

    locales
}

fn init_logger() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let s = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(s).expect("setting default subscriber failed");
}

fn main() -> anyhow::Result<()> {
    let locales = init_localise();
    init_logger();

    if !clipboard::is_history_enabled()? {
        error!(
            "{}",
            crate::value_locale!(locales, langid!("ja"), "disable-cb").unwrap()
        );
        return Ok(());
    }

    let Ok(histries) = clipboard::get_clipboard_history() else {
        error!("Failed to load clipboard data");
        return Ok(());
    };

    let cb = CbEditor::with_histries(histries);
    eframe::run_native(
        "Clipboard Editor",
        Default::default(),
        Box::new(|_| Ok(Box::new(cb))),
    )?;

    Ok(())
}
