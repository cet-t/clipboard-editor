#[derive(Clone, Copy, Default)]
pub struct WindowFlags {
    pub info: bool,
    pub editor: bool,
}

impl WindowFlags {
    pub fn is_opening(self) -> bool {
        self.info || self.editor
    }
}
