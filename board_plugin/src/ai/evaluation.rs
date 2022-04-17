use crate::{
    components::{coordinates::Coordinates, pieces::PieceKind},
    resources::{
        board::Board,
        game_state::PlayerColor,
        tile_map::{MoveResult, BLUE_TEMPLE, RED_TEMPLE},
    },
};

// PST is taken from: https://github.com/maxbennedich/onitama/blob/master/src/main/java/onitama/ai/evaluation/PieceSquareTables.java#L10
const PIECE_SQUARE_TABLE: [[i32; 5]; 5] = [
    [0, 4, 8, 4, 0],
    [4, 8, 12, 8, 4],
    [8, 12, 16, 12, 8],
    [4, 8, 12, 8, 4],
    [0, 4, 8, 4, 0],
];

#[derive(Debug)]
pub struct Evaluation;

impl Evaluation {
    pub fn evaluate(
        board: &Board,
        curr_color: &PlayerColor,
        move_result: &Option<MoveResult>,
    ) -> i32 {
        let mut sign = 1;

        // let curr_color = curr_color.enemy();
        let curr_color = *curr_color;
        if curr_color == PlayerColor::Blue {
            sign = -1;
        }

        if let Some(move_result) = move_result {
            if *move_result == MoveResult::Win {
                return -sign * 10000;
            }
        }

        let mut my_piece_score_sum = 0;
        let mut enemy_piece_score_sum = 0;
        let mut enemy_temple_distance = 0;
        let mut my_temple_distance = 0;
        let mut enemy_close_enemies = 0;
        let mut my_close_enemies = 0;
        let mut my_piece_square = 0;
        let mut enemy_piece_square = 0;

        let (enemy_temple, my_temple) = match curr_color {
            PlayerColor::Red => (BLUE_TEMPLE, RED_TEMPLE),
            PlayerColor::Blue => (RED_TEMPLE, BLUE_TEMPLE),
        };

        let mut my_king_coords = my_temple;
        let mut enemy_king_coords = enemy_temple;
        let mut king_amount = 0;

        for (y, line) in board.tile_map.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let Some(piece) = tile.piece {
                    if piece.kind == PieceKind::King && piece.color == curr_color {
                        my_king_coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        king_amount += 1;
                    } else if piece.kind == PieceKind::King && piece.color != curr_color {
                        enemy_king_coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        king_amount += 1;
                    }
                    if king_amount == 2 {
                        break;
                    }
                }
            }
        }

        for (y, line) in board.tile_map.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let Some(piece) = tile.piece {
                    let piece_score = match piece.kind {
                        PieceKind::Pawn => 10,
                        PieceKind::King => 10000,
                    };

                    if piece.color == curr_color {
                        my_piece_score_sum += piece_score;

                        // how close are the enemies to the king
                        let coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        enemy_close_enemies += Self::manhattan_distance(coords, enemy_king_coords);

                        my_piece_square += PIECE_SQUARE_TABLE[y][x];
                    } else {
                        enemy_piece_score_sum += piece_score;

                        let coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        my_close_enemies += Self::manhattan_distance(coords, my_king_coords);

                        enemy_piece_square += PIECE_SQUARE_TABLE[y][x];
                    }

                    // how king is far from temple
                    if piece.kind == PieceKind::King && piece.color == curr_color {
                        enemy_temple_distance =
                            Self::manhattan_distance(my_king_coords, enemy_temple);
                    } else if piece.kind == PieceKind::King && piece.color != curr_color {
                        my_temple_distance = Self::manhattan_distance(enemy_king_coords, my_temple);
                    }
                }
            }
        }
        sign * ((my_piece_score_sum - enemy_piece_score_sum)
            - (my_temple_distance - enemy_temple_distance)
            + (my_close_enemies - enemy_close_enemies)
            + (my_piece_square - enemy_piece_square))
    }

    fn manhattan_distance(from: Coordinates, to: Coordinates) -> i32 {
        (to.x as i32 - from.x as i32).abs() + (to.y as i32 - from.y as i32).abs()
    }
}
