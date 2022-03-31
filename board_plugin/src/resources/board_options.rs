use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

/// Tile size option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

/// Board generation options. Must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    pub position: Vec3,
    pub tile_size: TileSize,
    pub tile_padding: f32,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 100.0,
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
        }
    }
}
