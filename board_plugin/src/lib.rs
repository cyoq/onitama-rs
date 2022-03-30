pub mod components;
pub mod resources;
pub mod systems;

use bevy::log;
use bevy::prelude::*;
use resources::board::Board;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board() {
        let mut _board = Board::new();
        #[cfg(feature = "debug")]
        log::info!("{}", _board.console_output());
    }
}
