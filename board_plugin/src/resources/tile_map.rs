use crate::components::coordinates::Coordinates;
use crate::components::pieces::{Piece, PieceKind::*};
use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

use super::card::Card;
use super::game::PlayerColor::*;

const BOARD_SIZE: usize = 5;

/// Base tile map
#[derive(Debug, Clone, Copy)]
pub struct TileMap {
    height: u8,
    width: u8,
    pub map: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

impl TileMap {
    /// Generates an empty map
    pub fn new() -> Self {
        let height = BOARD_SIZE as u8;
        let width = BOARD_SIZE as u8;

        let blue_pawn = Piece::new(Pawn, Blue);
        let blue_king = Piece::new(King, Blue);

        let red_pawn = Piece::new(Pawn, Red);
        let red_king = Piece::new(King, Red);

        let map = [
            [
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_king)),
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_pawn.clone())),
            ],
            [
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
            ],
            [
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
            ],
            [
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
                Tile::new(None),
            ],
            [
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_king)),
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_pawn.clone())),
            ],
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

    pub fn generate_allowed_moves(
        &self,
        coordinates: &Coordinates,
        card: &Card,
    ) -> Vec<Coordinates> {
        card.directions
            .iter()
            .map(|tuple| {
                if card.is_mirrored {
                    *coordinates + (tuple.0, -tuple.1)
                } else {
                    *coordinates + *tuple
                }
            })
            .filter(|coords| {
                coords.x < 5
                    && coords.y < 5
                    && match self.map[coords.y as usize][coords.x as usize].piece {
                        Some(piece) => piece.color == piece.enemy(),
                        // no piece - it is good to go
                        None => true,
                    }
            })
            .collect::<Vec<_>>()
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

impl Deref for TileMap {
    type Target = [[Tile; BOARD_SIZE]; BOARD_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
