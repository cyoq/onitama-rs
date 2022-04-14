use bevy::{prelude::Entity, log};
use rand::Rng;

use crate::resources::{tile_map::Move, deck::Deck, game_state::GameState, board::Board};

use super::agent::Agent;

#[derive(Debug)]
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move) {
        let all_moves = board
            .tile_map
            .generate_all_possible_moves(&game_state, &deck);
        let card_idx: usize = rand::thread_rng().gen_range(0..2);

        let moves = &all_moves[card_idx];
        let size = moves.moves.len();
        let mov_idx: usize = rand::thread_rng().gen_range(0..size);

        let mov = &moves.moves[mov_idx];
        log::info!("Random bot chose a move {:?}", mov);

        (moves.card, *mov)
    }
}
