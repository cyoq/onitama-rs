use bevy::prelude::Entity;

use crate::components::coordinates::Coordinates;

#[derive(Debug, Clone)]
pub struct SelectedCard {
    pub entity: Option<Entity>,
}

impl Default for SelectedCard {
    fn default() -> Self {
        Self { entity: None }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}

impl Default for SelectedPiece {
    fn default() -> Self {
        Self { entity: None }
    }
}