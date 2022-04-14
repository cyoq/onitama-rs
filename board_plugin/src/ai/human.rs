use bevy::prelude::Entity;

use crate::resources::{board::Board, deck::Deck, game_state::GameState, tile_map::Move};

use super::agent::Agent;

#[derive(Debug)]
pub struct Human;

impl Agent for Human {
    fn generate_move(
        &self,
        _board: &Board,
        _game_state: &GameState,
        _deck: &Deck,
    ) -> (Entity, Move) {
        todo!();
    }
}
