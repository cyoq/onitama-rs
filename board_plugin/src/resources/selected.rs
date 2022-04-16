use bevy::prelude::Entity;

use crate::components::coordinates::Coordinates;

use super::game_state::PlayerType;

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
    pub coordinates: Option<Coordinates>,
}

impl SelectedPiece {
    pub fn clear(&mut self) {
        self.entity = None;
        self.coordinates = None;
    }
}

impl Default for SelectedPiece {
    fn default() -> Self {
        Self {
            entity: None,
            coordinates: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedPlayers {
    pub red_player: PlayerType,
    pub blue_player: PlayerType,
}

impl Default for SelectedPlayers {
    fn default() -> Self {
        Self {
            red_player: PlayerType::Human,
            blue_player: PlayerType::Human,
        }
    }
}
