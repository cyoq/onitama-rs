use bevy::prelude::Entity;

use crate::components::coordinates::Coordinates;
use crate::components::pieces::{Piece, PieceKind::*};
use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

use super::card::Card;
use super::deck::Deck;
use super::game_state::{
    GameState,
    PlayerColor::{self, *},
};

const BOARD_SIZE: usize = 5;

pub const RED_TEMPLE: Coordinates = Coordinates { x: 2, y: 0 };
pub const BLUE_TEMPLE: Coordinates = Coordinates { x: 2, y: 4 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Win,
    Tie,
    Capture,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Coordinates,
    pub to: Coordinates,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            from: Coordinates { x: 0, y: 0 },
            to: Coordinates { x: 0, y: 0 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PossibleMoves {
    pub card: Entity,
    pub moves: Vec<Move>,
}

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
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_king)),
                Tile::new(Some(red_pawn.clone())),
                Tile::new(Some(red_pawn.clone())),
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
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_king)),
                Tile::new(Some(blue_pawn.clone())),
                Tile::new(Some(blue_pawn.clone())),
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
        game_state: &GameState,
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
                        Some(piece) => piece.color != game_state.curr_color,
                        // no piece - it is good to go
                        None => true,
                    }
            })
            .collect::<Vec<_>>()
    }

    pub fn make_a_move(&mut self, start: Coordinates, end: Coordinates) -> MoveResult {
        let start_tile = self.map[start.y as usize][start.x as usize];
        let end_tile = self.map[end.y as usize][end.x as usize];

        // we can be sure that the start tile must have a piece
        let start_piece = start_tile.piece.unwrap();

        if let Some(end_piece) = end_tile.piece {
            let mut result = MoveResult::Move;
            if start_piece.color != end_piece.color && end_piece.kind == King {
                result = MoveResult::Win;
            } else if start_piece.color != end_piece.color {
                result = MoveResult::Capture;
            }

            self.map[end.y as usize][end.x as usize] = self.map[start.y as usize][start.x as usize];
            self.map[start.y as usize][start.x as usize].piece = None;
            return result;
        }

        self.map[start.y as usize][start.x as usize].piece = None;
        self.map[end.y as usize][end.x as usize] = start_tile;

        let end_tile = self.map[end.y as usize][end.x as usize];
        if let Some(piece) = end_tile.piece {
            if piece.kind == King && piece.color == Red && end == BLUE_TEMPLE {
                return MoveResult::Win;
            }
            if piece.kind == King && piece.color == Blue && end == RED_TEMPLE {
                return MoveResult::Win;
            }
        }

        MoveResult::Move
    }

    pub fn undo_move(
        &mut self,
        from: Coordinates,
        to: Coordinates,
        prev_move_result: &MoveResult,
        tile: Tile,
    ) {
        let from_tile = self.map[from.y as usize][from.x as usize];
        let to_tile = self.map[to.y as usize][to.x as usize];

        if *prev_move_result == MoveResult::Capture || *prev_move_result == MoveResult::Win {
            self.map[to.y as usize][to.x as usize] = from_tile;
            self.map[from.y as usize][from.x as usize] = tile;
            return;
        }

        if from_tile.piece == None {
            panic!("Cancelling empty tile");
        }

        if to_tile.piece != None {
            panic!("Moving to the non-empty tile");
        }

        self.map[to.y as usize][to.x as usize] = from_tile;
        self.map[from.y as usize][from.x as usize] = to_tile;
    }

    pub fn generate_all_possible_moves(
        &self,
        game_state: &GameState,
        deck: &Deck,
    ) -> Vec<PossibleMoves> {
        let cards = deck.get_player_cards(game_state);
        let mut possible_moves = Vec::with_capacity(2);
        for (e, card) in cards.iter() {
            let moves = self.generate_possible_moves_for_card(&game_state.curr_color, card);
            possible_moves.push(PossibleMoves { card: *e, moves });
        }
        possible_moves
    }

    pub fn generate_possible_moves_for_card(
        &self,
        curr_player_color: &PlayerColor,
        card: &Card,
    ) -> Vec<Move> {
        let mut moves = vec![];
        for (y, line) in self.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let Some(piece) = tile.piece {
                    if piece.color != *curr_player_color {
                        continue;
                    }

                    let coordinates = Coordinates {
                        x: x as u8,
                        y: y as u8,
                    };
                    for dir in card.directions {
                        let mov = if card.is_mirrored {
                            Move {
                                from: coordinates,
                                to: coordinates + (dir.0, -dir.1),
                            }
                        } else {
                            Move {
                                from: coordinates,
                                to: coordinates + *dir,
                            }
                        };

                        if mov.to.x < 5
                            && mov.to.y < 5
                            && match self.map[mov.to.y as usize][mov.to.x as usize].piece {
                                Some(piece) => piece.color != *curr_player_color,
                                // no piece - it is good to go
                                None => true,
                            }
                        {
                            moves.push(mov);
                        }
                    }
                }
            }
        }
        moves
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
