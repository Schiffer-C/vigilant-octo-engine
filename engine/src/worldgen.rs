use crate::render::{RenderData, Rgb24};

/// -------------------------
/// Enums and Structs
/// -------------------------

#[deprecated]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Grass,
    Water,
    Rock
}

#[deprecated]
impl Tile {
    pub fn is_traversable(self) -> bool {
        matches!(self, Tile::Grass)
    }
}

// Biome decided at worldgen. Determines probability of other features appearing during later stages of worldgen (layer0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Forest,
    Desert,
    Tundra,
    Plains
}

impl Biome {
    pub fn bg_color(self) -> Rgb24 {
        match self {
            Biome::Forest => 0x32a852, // #32a852
            Biome::Desert => 0xdeecb6, // #deecb6
            Biome::Tundra => 0xd9e7f0, // #d9e7f0
            Biome::Plains => 0xd7ffe9 // #d7ffe9
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidType {
    Water,
    Lava,
    Oil,
    Acid
}

impl FluidType {
    pub fn render_data(self) -> RenderData {
        match self {
            FluidType::Water => RenderData { color: 0x2A5CAA, glyph: '≈' }, // #2A5CAA
            FluidType::Lava => RenderData { color: 0xec874c, glyph: '≈' }, // #ec874c
            FluidType::Oil => RenderData { color: 0x0e1f3a, glyph: '≈' }, // #0e1f3a
            FluidType::Acid => RenderData { color: 0x54fc2a, glyph: '≈' }, // #54fc2a
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Iron,
    Copper,
    Coal,
    Crystal
}

impl ResourceType {
    pub fn render_data(self) -> RenderData {
        match self {
            ResourceType::Iron   => RenderData { color: 0xB0B0B0, glyph: '⛏' }, // #B0B0B0
            ResourceType::Copper => RenderData { color: 0xC07030, glyph: '⛏' }, // #C07030
            ResourceType::Coal   => RenderData { color: 0x303030, glyph: '⛏' }, // #303030
            ResourceType::Crystal   => RenderData { color: 0xf3d5ef, glyph: '⛏' }, // #f3d5ef
        }
    }
}

// Decided after the Biome in worldgen, decides if a fluid or resource node is present (layer1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceLayer {
    None,
    Fluid(FluidType),
    Resource(ResourceType)
}

impl ResourceLayer {
    /// None means "draw nothing for layer1"
    pub fn render_data(self) -> Option<RenderData> {
        match self {
            ResourceLayer::None => None,
            ResourceLayer::Fluid(f) => Some(f.render_data()),
            ResourceLayer::Resource(n) => Some(n.render_data()),
        }
    }
}

// Decided after the ResourceLayer in worldgen, decides if a feature is present (layer2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureLayer {
    None,
    Tree,
    Rock
}

// Struct containing worldgen related tile data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldgenTile {
    pub biome_layer: Biome,
    pub resource_layer: ResourceLayer,
    pub feature_layer: FeatureLayer
}

impl WorldgenTile {
    pub fn is_traversable(self) -> bool {
        !matches!(self.resource_layer, ResourceLayer::Fluid(_))
    }
}


/// -------------------------
/// Hashing and Helper Functions
/// -------------------------


/// Produces a unique hash value for every tile [x,y,salt] combination (seeded)
pub fn hash(seed: u32, x: i32, y: i32, salt: u32) -> u32 {
    let mut h = seed ^ salt as u32;

    h ^= x as u32;
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

/// Normalizes 32-bit integer into float value from range [0, 1)
fn floatify(val: u32) -> f32 {
    let mantissa = val >> 8;
    (mantissa as f32) / ((1u32 << 24) as f32)
}

//. Smoothstep function to ease transition between biomes etc.
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Linear Interpolation. Returns a value that is t% between (a, b)
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/** Noise function for worldgen.
 * Takes in the world seed, coordinates, size of cell, and salt value.
 * Salt is used to differentiate between different worldgen layers.
 * Uses linear interpolation with the hash function to create blob-shaped patches of noise
 * Each layer of worldgen is executing with the same values but a unique salt.
 * Using hashed-values with a seed ensures results are reproducible every time.
*/
fn value_noise_2d(seed: u32, x: i32, y: i32, cell_size: i32, salt: u32) -> f32 {
    // Calculate the coordinates in terms of cells (converts from global coords --> cell coords)
    let cx = div_floor(x, cell_size);
    let cy = div_floor(y, cell_size);

    // Get the actual value we're finding
    let fx = ((x - cx * cell_size) as f32) / (cell_size as f32);
    let fy = ((y - cy * cell_size) as f32) / (cell_size as f32);

    // Smooth the edges of the curve
    let sx = smoothstep(fx);
    let sy = smoothstep(fy);

    // Take the four corner samples
    let v00 = floatify(hash(seed, cx, cy, salt));
    let v01 = floatify(hash(seed, cx, cy + 1, salt));
    let v10 = floatify(hash(seed, cx + 1, cy, salt));
    let v11 = floatify(hash(seed, cx + 1, cy + 1, salt));

    // Perform linear interpolation between the corners
    let ix0 = lerp(v00, v10, sx);
    let ix1 = lerp(v01, v11, sx);
    lerp(ix0, ix1, sy)
}

/// Floor division
fn div_floor(a: i32, b: i32) -> i32 {
    let q = a / b;
    let r = a % b;
    if (r != 0) && ((r > 0) != (b > 0)) { q - 1 } else { q }
}

pub fn tile_at(seed: u32, x: i32, y: i32) -> WorldgenTile {
    let biome = biome_at(seed, x, y);
    let resources = resource_at(seed, x, y, biome);
    let features = feature_at(seed, x, y, biome, resources);

    WorldgenTile { biome_layer: biome, resource_layer: resources, feature_layer: features }
}

/// -------------------------
/// Layer0 - Biomes
/// -------------------------

pub fn biome_at(seed: u32, x: i32, y: i32) -> Biome {
    // TODO: Replace magic number(s) with config file
    let cell_size = 96; 

    let temp = value_noise_2d(seed, x, y, cell_size, 0xA1B2_C3D4);
    let humidity = value_noise_2d(seed, x, y, cell_size, 0x1A2B_3C4D);

    if temp < 0.30 {
        Biome::Tundra
    } else if temp > 0.70 && humidity < 0.35 {
        Biome::Desert
    } else if humidity > 0.60 {
        Biome::Forest
    } else {
        Biome::Plains
    }
}

/// -------------------------
/// Layer1 - Resources
/// -------------------------

pub fn resource_at(seed: u32, x: i32, y: i32, biome: Biome) -> ResourceLayer {
    
    let fluid_noise = value_noise_2d(seed, x, y, 48, 0xF00D_1234);
    let node_noise  = value_noise_2d(seed, x, y, 32, 0xC0FF_EE77);

    // 1) Fluids first (because they affect traversal).
    // Make water more common in some biomes.
    let water_threshold = match biome {
        Biome::Desert => 0.08,  // rare
        Biome::Tundra => 0.18,  // some
        Biome::Plains => 0.22,
        Biome::Forest => 0.28,  // more lakes/ponds
    };

    // Lower noise values become water blobs.
    if fluid_noise < water_threshold {
        return ResourceLayer::Fluid(FluidType::Water);
    }

    // 2) Nodes (mineable), only if not water.
    // Use a "peak" test to create patches: require node_noise to be very high.
    let node_threshold = match biome {
        Biome::Desert => 0.865, // more nodes
        Biome::Tundra => 0.955,
        Biome::Plains => 0.950,
        Biome::Forest => 0.955,
    };

    if node_noise > node_threshold {
        // Choose node type deterministically from a hash.
        let pick = hash(seed, x, y, 0xBEEF_CAFE) % 100;
        let node = match biome {
            Biome::Desert => {
                // Example bias: more copper/coal, less iron
                if pick < 40 { ResourceType::Copper } else if pick < 80 { ResourceType::Coal } else { ResourceType::Iron }
            }
            Biome::Tundra => {
                if pick < 55 { ResourceType::Iron } else if pick < 80 { ResourceType::Coal } else { ResourceType::Copper }
            }
            _ => {
                if pick < 55 { ResourceType::Iron } else if pick < 80 { ResourceType::Copper } else { ResourceType::Coal }
            }
        };
        return ResourceLayer::Resource(node);
    }

    ResourceLayer::None
}

/// -------------------------
/// Layer2: Features (trees/rocks/etc)
/// -------------------------

pub fn feature_at(seed: u32, x: i32, y: i32, biome: Biome, resource: ResourceLayer) -> FeatureLayer {
    // Don’t place trees/rocks on resource tiles (for now).
    if !matches!(resource, ResourceLayer::None) {
        return FeatureLayer::None;
    }

    // Small-scale noise for “clumps.”
    let feat = value_noise_2d(seed, x, y, 16, 0xDEADBEEF);

    match biome {
        Biome::Forest => {
            // Lots of trees, some rocks.
            if feat > 0.62 { FeatureLayer::Tree }
            else if feat < 0.06 { FeatureLayer::Rock }
            else { FeatureLayer::None }
        }
        Biome::Plains => {
            // Few trees, few rocks.
            if feat > 0.90 { FeatureLayer::Tree }
            else if feat < 0.05 { FeatureLayer::Rock }
            else { FeatureLayer::None }
        }
        Biome::Desert => {
            // Mostly rocks (or later: cacti), no trees.
            if feat < 0.09 { FeatureLayer::Rock } else { FeatureLayer::None }
        }
        Biome::Tundra => {
            // Some rocks, sparse trees (later: pines).
            if feat > 0.94 { FeatureLayer::Tree }
            else if feat < 0.08 { FeatureLayer::Rock }
            else { FeatureLayer::None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::worldgen::hash;

    use super::*;

    #[test]
    fn deterministic() {
        let seed = 12345;
        let a = hash(seed, 10, -7, 1);
        let b = hash(seed, 10, -7, 1);
        assert_eq!(a, b);
    }

    #[test]
    fn different_coords_change_output() {
        let seed = 12345;
        let a = hash(seed, 10, -7, 1);
        let b = hash(seed, 10, -6, 1);
        assert_ne!(a, b);
    }
}