mod worldgen;
mod render;

use wasm_bindgen::prelude::*;

use crate::{render::{RenderType, Rgb24}, worldgen::tile_at};

#[wasm_bindgen]
pub fn memory_access() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub struct Game {
    seed: u32,
    px: i32,
    py: i32,
    width: u32,
    height: u32,

    bg_rgb_buff: Vec<u32>,
    fg_rgb_buff: Vec<u32>,
    glyph_buff: Vec<u32>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, screen_w: u32, screen_h: u32) -> Game {
        let len = (screen_w as usize) * (screen_h as usize);
        Game {
            seed,
            px: 0,
            py: 0,
            width: screen_w,
            height: screen_h,
            bg_rgb_buff: vec![0; len],
            fg_rgb_buff: vec![0; len],
            glyph_buff: vec![0; len],
        }
    }

    pub fn set_viewport(&mut self, view_w: u32, view_h: u32) {
        self.width = view_w;
        self.height = view_h;
        let len = (view_w as usize) * (view_h as usize);
        self.bg_rgb_buff.resize(len, 0);
        self.fg_rgb_buff.resize(len, 0);
        self.glyph_buff.resize(len, 0);
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        let nx = self.px + dx;
        let ny = self.py + dy;
        let dest = tile_at(self.seed, nx, ny);
        if dest.is_traversable() {
            self.px = nx;
            self.py = ny;
        }
    }

    pub fn pos_x(&self) -> i32 {
        self.px
    }

    pub fn pos_y(&self) -> i32 {
        self.py
    }

    pub fn prepare_render_buff(&mut self) {
        let w = self.width as i32;
        let h = self.height as i32;

        let cam_x = self.px - (w / 2);
        let cam_y = self.py - (h / 2);

        for sy in 0..h {
            for sx in 0..w {
                let wx = cam_x + sx;
                let wy = cam_y + sy;

                let idx = (sy as usize) * (w as usize) + (sx as usize);

                let tile = tile_at(self.seed, wx, wy);

                // Layer0 bg color
                self.bg_rgb_buff[idx] = tile.biome_layer.bg_color();

                // Preparing glyphs
                let mut glyph_code: u32 = 0;
                let mut glyph_color: Rgb24 = 0;

                // TODO: Add feature layer rendering
                if let Some(rd) = tile.resource_layer.render_data() {
                    match rd {
                        RenderType::Static { glyph, color } => {
                            glyph_code = glyph as u32;
                            glyph_color = color;
                        }
                        _ => {
                            // Not implemented yet
                        }
                    }
                }

                self.fg_rgb_buff[idx] = glyph_color;
                self.glyph_buff[idx] = glyph_code;
            }
        }

        let center_x = (w / 2) as usize;
        let center_y = (h / 2) as usize;
        let center_idx = center_y * (w as usize) + center_x;

        self.glyph_buff[center_idx] = '@' as u32;
        self.fg_rgb_buff[center_idx] = 0xFFFFFF;
    }

    pub fn buff_len(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    pub fn bg_rgb_buff_ptr(&self) -> *const u32 {
        self.bg_rgb_buff.as_ptr()
    }

    pub fn fg_rgb_buff_ptr(&self) -> *const u32 {
        self.fg_rgb_buff.as_ptr()
    }

    pub fn glyph_buff_ptr(&self) -> *const u32 {
        self.glyph_buff.as_ptr()
    }
}
