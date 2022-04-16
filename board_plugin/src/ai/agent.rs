use core::fmt::Debug;

use bevy::prelude::Entity;

use crate::resources::{board::Board, deck::Deck, game_state::GameState, tile_map::Move};

pub trait Agent: Debug + Sync + Send {
    // returns a card entity and a desired move
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move);
    // To clone the agent, it requires quite awful construction: https://stackoverflow.com/a/69891769
    fn clone_dyn(&self) -> Box<dyn Agent>;
}

impl Clone for Box<dyn Agent> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
