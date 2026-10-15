#[derive(Clone, Copy, Default)]
pub struct WindowFlags {
    pub info: bool,
    pub editor: bool,
    pub setting: bool,
}

impl WindowFlags {
    pub const fn is_opening(&self) -> bool {
        self.info || self.editor || self.setting
    }
}
