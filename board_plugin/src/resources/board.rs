use bevy::prelude::*;

use crate::bounds::Bounds2;
use crate::components::coordinates::Coordinates;

use super::tile_map::TileMap;

/// Base tile map
#[derive(Debug, Clone)]
pub struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub padding: f32,
    pub tile_map: TileMap,
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }
        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u8,
            y: (coordinates.y / self.tile_size) as u8,
        })
    }

    pub fn in_bounds(&self, window: &Window, position: Vec2) -> bool {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        self.bounds.in_bounds(position)
    }
}
