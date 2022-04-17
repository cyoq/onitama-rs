use bevy::prelude::Entity;

use crate::resources::{board::Board, deck::Deck, game_state::GameState, tile_map::Move};

use super::agent::Agent;

#[derive(Debug, Clone)]
pub struct Human;

impl Agent for Human {
    fn generate_move(
        &self,
        _board: &Board,
        _game_state: &GameState,
        _deck: &Deck,
    ) -> (Entity, Move, i32) {
        unimplemented!("This function should not be implemented, because human generates move via interface");
    }

    fn clone_dyn(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}
