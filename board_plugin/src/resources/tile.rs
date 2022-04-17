use bevy::prelude::Component;
#[cfg(feature = "debug")]
use colored::Colorize;

use crate::components::pieces::Piece;

#[derive(Debug, Component)]
pub struct TempleTile;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Tile {
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn new(piece: Option<Piece>) -> Self {
        Self { piece }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        use crate::components::pieces::PieceKind::*;
        use crate::resources::game_state::PlayerColor::*;
        format!(
            "{}",
            match self.piece {
                Some(piece) => {
                    match (piece.color, piece.kind) {
                        (Blue, King) => "B".cyan(),
                        (Blue, Pawn) => "b".cyan(),
                        (Red, King) => "R".bright_red(),
                        (Red, Pawn) => "r".bright_red(),
                    }
                }
                None => " ".normal(),
            }
        )
    }
}
