use serde::{Deserialize, Serialize};
use bevy::prelude::*;

use super::board_options::TileSize;


/// Board generation options. Must be used as a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckOptions {
    pub position: Vec3,
    pub tile_size: TileSize,
    pub tile_padding: f32,
}

impl Default for DeckOptions {
    fn default() -> Self {
        Self {
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
        }
    }
}
