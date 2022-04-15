use crate::{
    components::{coordinates::Coordinates, pieces::PieceKind},
    resources::{
        board::Board,
        game_state::PlayerColor,
        tile_map::{BLUE_TEMPLE, RED_TEMPLE, MoveResult},
    },
};

#[derive(Debug)]
pub struct Evaluation;

impl Evaluation {
    pub fn evaluate(board: &Board, curr_color: &PlayerColor, move_result: Option<&MoveResult>) -> i32 {

        let mut sign = 1;
        let curr_color = curr_color.enemy();
        if curr_color == PlayerColor::Blue {
            sign = -1;
        }

        if let Some(move_result) = move_result {
            if *move_result == MoveResult::Win {
                return sign * 10000;
            }
        }

        let mut piece_score_sum = 0;
        let mut temple_distance = 0;
        let mut close_enemies = 0;

        let (enemy_temple, my_temple) = match curr_color {
            PlayerColor::Red => (BLUE_TEMPLE, RED_TEMPLE),
            PlayerColor::Blue => (RED_TEMPLE, BLUE_TEMPLE),
        };

        let mut king_coords = my_temple;
        for (y, line) in board.tile_map.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let Some(piece) = tile.piece {
                    if piece.kind == PieceKind::King && piece.color == curr_color {
                        king_coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        break;
                    }
                }
            }
        }

        for (y, line) in board.tile_map.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if let Some(piece) = tile.piece {
                    let piece_score = match piece.kind {
                        PieceKind::Pawn => 1,
                        PieceKind::King => 100,
                    };

                    if piece.color == curr_color {
                        piece_score_sum += piece_score;
                    }

                    // how king is far from temple
                    if piece.kind == PieceKind::King && piece.color == curr_color {
                        temple_distance =
                            Self::manhattan_distance(king_coords, enemy_temple);
                    }

                    if piece.color != curr_color {
                        let coords = Coordinates {
                            x: x as u8,
                            y: y as u8,
                        };
                        close_enemies += Self::manhattan_distance(coords, king_coords);
                    }
                }
            }
        }
        sign * (5 * piece_score_sum - 2 * temple_distance - 3 * close_enemies)
    }

    fn manhattan_distance(from: Coordinates, to: Coordinates) -> i32 {
        (to.x as i32 - from.x as i32) + (to.y as i32 - from.y as i32)
    }
}
