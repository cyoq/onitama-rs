use bevy::{log, prelude::Entity};
use rand::{thread_rng, Rng};

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
    // pub analysed_moves: BinaryHeap<MoveEvaluation>,
}

impl AlphaBetaAgent {
    pub fn new(max_depth: u8) -> Self {
        // let heap = BinaryHeap::new();
        Self {
            max_depth,
            // analysed_moves: heap,
        }
    }

    fn alpha_beta(
        &self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        board: &mut Board,
        game_state: &mut GameState,
        deck: &mut Deck,
        move_result: Option<&MoveResult>,
        positions: &mut i32,
    ) -> (Option<Move>, i32) {
        *positions += 1;
        let player_color = game_state.curr_color;

        if depth == self.max_depth || move_result == Some(&MoveResult::Win) {
            return (
                None,
                Evaluation::evaluate(&board, depth, &player_color, move_result),
            );
        }

        let card_idx: usize = if player_color == PlayerColor::Red {
            thread_rng().gen_range(3..5)
        } else {
            thread_rng().gen_range(0..2)
        };

        let card = &deck.cardboards.get(&deck.cards[card_idx]).unwrap().card;

        let possible_moves = board
            .tile_map
            .generate_possible_moves_for_card(&player_color, &card);

        let mut best_score;
        let mut best_move = if possible_moves.is_empty() {
            None
        } else {
            Some(possible_moves[0])
        };

        if player_color == PlayerColor::Red {
            best_score = std::i32::MIN;
        } else {
            best_score = std::i32::MAX;
        }

        for mov in possible_moves.iter() {
            let possible_piece_lose =
                board.tile_map.map[mov.to.y as usize][mov.to.x as usize].clone();

            let result = board.tile_map.make_a_move(mov.from, mov.to);

            game_state.next_turn();
            deck.swap_card_with_neutral(card_idx);

            // go deeper the tree
            let (_, score) = self.alpha_beta(
                depth + 1,
                alpha,
                beta,
                &mut board.clone(),
                &mut game_state.clone(),
                &mut deck.clone(),
                Some(&result),
                positions,
            );

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
                }
                alpha = std::cmp::max(alpha, score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = Some(*mov);
                }
                beta = std::cmp::min(beta, score);
            }

            if alpha >= beta {
                break;
            }
        }

        (best_move, best_score)
    }
}

impl Agent for AlphaBetaAgent {
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move) {
        let cards = deck.get_player_cards(game_state);

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
        log::info!("Evaluation score: {:?}", result.1);
        log::info!("Analyzed over {:?} positions", positions);

        (cards[0].0, result.0.unwrap())
    }

    fn clone_dyn(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}
