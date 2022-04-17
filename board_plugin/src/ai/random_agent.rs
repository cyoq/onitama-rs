use bevy::{log, prelude::Entity};
use rand::Rng;

use crate::resources::{board::Board, deck::Deck, game_state::GameState, tile_map::Move};

use super::agent::Agent;

#[derive(Debug, Clone)]
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move, i32) {
        let all_moves = board
            .tile_map
            .generate_all_possible_moves(&game_state, &deck);
        let card_idx: usize = rand::thread_rng().gen_range(0..2);

        let moves = &all_moves[card_idx];
        let size = moves.moves.len();
        let mov_idx: usize = rand::thread_rng().gen_range(0..size);

        let mov = &moves.moves[mov_idx];
        log::info!("Random bot chose a move {:?}", mov);

        (moves.card, *mov, 0)
    }

    fn clone_dyn(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
    
}
