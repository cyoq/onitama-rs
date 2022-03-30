#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Pawn,
    King,
}

impl Tile {
    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::King => "R".bright_red(),
                Tile::Pawn => "r".bright_red(),
                Tile::Empty => " ".normal(),
            }
        )
    }
}
