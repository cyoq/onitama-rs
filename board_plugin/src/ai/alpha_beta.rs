use bevy::prelude::Entity;

use crate::resources::{tile_map::Move, deck::Deck, game_state::GameState, board::Board};

use super::agent::Agent;

#[derive(Debug, Clone, Copy)]
pub struct AlphaBetaAgent;

impl Agent for AlphaBetaAgent {
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move) {
        todo!()
    }
}
