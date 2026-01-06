#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Grass,
    Water,
    Rock
}

impl Tile {
    pub fn is_traversable(self) -> bool {
        matches!(self, Tile::Grass)
    }
}


pub fn hash2(seed: u32, x: i32, y: i32) -> u32 {
    let mut h = seed ^ x as u32;
    h = h.wrapping_mul(0x9C2F_9653);
    h = h.rotate_left(16);

    h ^= y as u32;
    h = h.wrapping_mul(0xFA10_CDDF);
    h = h.rotate_left(13);

    h ^= h >> 16;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;

    h
}

pub fn tile_at(seed: u32, x: i32, y: i32) -> Tile {
    let r = hash2(seed, x, y);

    let p = (r % 100) as u8;

    match p {
        0..77 => Tile::Grass,
        77..91 => Tile::Rock,
        _ => Tile::Water
    }
}

#[cfg(test)]
mod tests {
    use crate::worldgen::tile_at;

    use super::*;

    #[test]
    fn deterministic() {
        let seed = 12345;
        let a = tile_at(seed, 10, -7);
        let b = tile_at(seed, 10, -7);
        assert_eq!(a, b);
    }

    #[test]
    fn different_coords_change_output() {
        let seed = 12345;
        let a = hash2(seed, 10, -7);
        let b = hash2(seed, 10, -6);
        assert_ne!(a, b);
    }
}