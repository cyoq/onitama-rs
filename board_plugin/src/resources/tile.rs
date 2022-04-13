#[cfg(feature = "debug")]
use colored::Colorize;

use crate::components::pieces::Piece;

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
        use crate::resources::game::PlayerColor::*;
        use crate::components::pieces::PieceKind::*;
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
