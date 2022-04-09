use bevy::prelude::Entity;

#[derive(Debug, Clone)]
pub struct SelectedCard(pub Option<Entity>);

impl Default for SelectedCard {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone)]
pub struct SelectedTile(pub Option<Entity>);