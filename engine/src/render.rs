use std::{collections::HashMap, sync::LazyLock};

pub type Rgb24 = u32;
const ASSEMBLER_SPRITE: &str = include_str!("../sprites/ascii-art.txt");
static SPRITE_LIBRARY: LazyLock<Vec<SpriteConfig>> = LazyLock::new(|| {
    let mut lib: Vec<SpriteConfig> = Vec::new();
    lib.push(SpriteConfig::from_ascii(ASSEMBLER_SPRITE, 
        &HashMap::from([
            ('X', 0xeeff88),
            ('└', 0x2fb1ac)
        ]), 0xffffff));
        lib
});

pub static ASSEMBLER_ID: usize = 0;

pub fn get_sprite(id: usize) -> Option<&'static SpriteConfig> {
    SPRITE_LIBRARY.get(id)
}

pub enum RenderType {
    Static {
        glyph: char,
        color: Rgb24
    },

    SpriteCell {
        sprite_id: usize,
        local_x: u8,
        local_y: u8
    }
}

pub struct SpriteConfig {
    pub width: u16,
    pub height: u16,
    pub glyphs: Vec<char>,
    pub colors: Vec<Rgb24>
}

impl SpriteConfig {
    pub fn from_ascii(ascii: &str, palette: &HashMap<char, Rgb24>, default_color: Rgb24) -> SpriteConfig {
        let rows: Vec<&str> = ascii.lines().collect();

        let height = rows.len() as u16;
        let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as u16;

        let mut glyphs = Vec::with_capacity((width as usize) * (height as usize));
        let mut colors = Vec::with_capacity(glyphs.capacity());

        for line in rows.iter() {
            let mut chars = line.chars();

            for _x in 0..width {
                let ch = chars.next().unwrap_or(' ');
                let color = palette.get(&ch).copied().unwrap_or(default_color);

                glyphs.push(ch);
                colors.push(color);
            }   
        }

        SpriteConfig { width, height, glyphs, colors }
    }

    pub fn glyph_at(&self, x: u16, y: u16) -> Option<(char, Rgb24)> {
        let x = x as usize;
        let y = y as usize;
        let w = self.width as usize;

        if x >= w || y >= self.height as usize {
            return None;
        }

        let idx = y * w + x;
        Some((self.glyphs[idx], self.colors[idx]))
    }
}



