mod app;

use crate::app::CbEditor;

fn main() -> anyhow::Result<()> {
    eframe::run_native(
        "Clipboard Editor",
        Default::default(),
        Box::new(|_| Ok(Box::new(CbEditor::default()))),
    )?;

    Ok(())
}
