mod worldgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game { x: 0, y: 0 }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn pos_x(&self) -> i32 {
        self.x
    }

    pub fn pos_y(&self) -> i32 {
        self.y
    }
}
