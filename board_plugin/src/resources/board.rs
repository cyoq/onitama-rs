use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

const BOARD_SIZE: usize = 5;

/// Base tile map
#[derive(Debug, Clone)]
pub struct Board {
    height: u8,
    width: u8,
    map: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Generates an empty map
    pub fn new() -> Self {
        let height = BOARD_SIZE as u8;
        let width = BOARD_SIZE as u8;
        use Tile::*;
        let map = [
            [Pawn, Pawn, King, Pawn, Pawn],
            [Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty],
            [Pawn, Pawn, King, Pawn, Pawn],
        ];

        Self { height, width, map }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!("Board ({}, {}):\n", self.width, self.height);
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    #[inline]
    pub fn width(&self) -> u8 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u8 {
        self.height
    }
}

impl Deref for Board {
    type Target = [[Tile; BOARD_SIZE]; BOARD_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
