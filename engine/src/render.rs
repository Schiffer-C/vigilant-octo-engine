pub type Rgb24 = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderData {
    pub color: Rgb24,
    pub glyph: char
}

impl RenderData {
    pub fn glyph_u32(self) -> u32 {
        self.glyph as u32
    }
}