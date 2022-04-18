use bevy::{log, prelude::Entity};

use crate::resources::{
    board::Board,
    deck::Deck,
    game_state::{GameState, PlayerColor},
    tile_map::{Move, MoveResult},
};

use super::{agent::Agent, evaluation::Evaluation};

#[derive(Debug, Clone, Copy)]
pub struct MoveEvaluation {
    pub mov: Move,
    pub score: i32,
}
#[derive(Debug, Clone)]
pub struct AlphaBetaAgent {
    pub max_depth: u8,
}

struct CalculationResult {
    best_move: Option<Move>,
    best_card: Option<Entity>,
    best_score: i32,
}

impl AlphaBetaAgent {
    pub fn new(max_depth: u8) -> Self {
        Self { max_depth }
    }

    fn alpha_beta(
        &self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        board: &mut Board,
        game_state: &mut GameState,
        deck: &mut Deck,
        move_result: Option<MoveResult>,
        positions: &mut i32,
    ) -> CalculationResult {
        *positions += 1;
        let player_color = game_state.curr_color;

        if depth == self.max_depth || move_result == Some(MoveResult::Win) {
            return CalculationResult {
                best_move: None,
                best_card: None,
                best_score: Evaluation::evaluate(&board, &player_color, &move_result),
            };
        }

        let cards = deck.get_player_cards(&game_state);

        let mut best_score;
        if player_color == PlayerColor::Red {
            best_score = std::i32::MIN;
        } else {
            best_score = std::i32::MAX;
        }

        let mut best_move = None;
        let mut best_card = None;

        for (entity, card) in cards.into_iter() {
            let card_idx = deck.cards.iter().position(|e| *e == entity).unwrap();

            let possible_moves = board
                .tile_map
                .generate_possible_moves_for_card(&player_color, &card);

            'br: for mov in possible_moves.iter() {
                let possible_piece_lose =
                    board.tile_map.map[mov.to.y as usize][mov.to.x as usize].clone();

                let result = board.tile_map.make_a_move(mov.from, mov.to);

                game_state.next_turn();
                deck.swap_card_with_neutral(card_idx);

                // go deeper the tree
                let calc_result = self.alpha_beta(
                    depth + 1,
                    alpha,
                    beta,
                    board,
                    game_state,
                    deck,
                    Some(result),
                    positions,
                );

                let score = calc_result.best_score;

                // Undo all made moves
                board
                    .tile_map
                    .undo_move(mov.to, mov.from, &result, possible_piece_lose);
                game_state.undo_next_turn();
                deck.swap_card_with_neutral(card_idx);

                if player_color == PlayerColor::Red {
                    if score > best_score {
                        best_score = score;
                        best_move = Some(*mov);
                        best_card = Some(entity);
                    }

                    if score >= beta {
                        break 'br;
                    }

                    alpha = std::cmp::max(alpha, score);
                } else {
                    if score < best_score {
                        best_score = score;
                        best_move = Some(*mov);
                        best_card = Some(entity);
                    }

                    if score <= alpha {
                        break 'br;
                    }

                    beta = std::cmp::min(beta, score);
                }

                if alpha >= beta {
                    break 'br;
                }
            }
        }

        CalculationResult {
            best_move,
            best_card,
            best_score,
        }
    }
}

impl Agent for AlphaBetaAgent {
    fn generate_move(
        &self,
        board: &Board,
        game_state: &GameState,
        deck: &Deck,
    ) -> (Option<Entity>, Option<Move>, i32) {
        let mut positions = 0;

        let result = self.alpha_beta(
            0,
            std::i32::MIN,
            std::i32::MAX,
            &mut board.clone(),
            &mut game_state.clone(),
            &mut deck.clone(),
            None,
            &mut positions,
        );

        log::info!("Evaluation score: {:?}", result.best_score);
        log::info!("Analyzed over {:?} positions", positions);

        (
            result.best_card,
            result.best_move,
            result.best_score,
        )
    }

    fn clone_dyn(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}
