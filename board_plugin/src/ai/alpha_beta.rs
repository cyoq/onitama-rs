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
        move_result: Option<&MoveResult>,
        positions: &mut i32,
    ) -> (Option<Move>, i32) {
        *positions += 1;
        let player_color = game_state.curr_color;

        if depth == self.max_depth || move_result == Some(&MoveResult::Win) {
            // log::info!("Evaluation: {:?} for color {:?}",
            //     Evaluation::evaluate(&board, depth, &player_color, move_result), player_color);
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

        'br: for mov in possible_moves.iter() {
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
                board,
                game_state,
                deck,
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

                if score >= beta {
                    break 'br;
                }

                alpha = std::cmp::max(alpha, score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = Some(*mov);
                }

                if score <= alpha {
                    break 'br;
                }

                beta = std::cmp::min(beta, score);
            }
        }

        (best_move, best_score)
    }

    pub fn generate_move(
        &self,
        board: &Board,
        game_state: &GameState,
        deck: &Deck,
    ) -> (Option<Move>, i32) {
        let mut positions = 0;
        
        let clone = self.clone();
        let mut board = board.clone();
        let mut game_state = game_state.clone();
        let mut deck = deck.clone();

        let handle = std::thread::spawn(move || {
            clone.alpha_beta(
                0,
                std::i32::MIN,
                std::i32::MAX,
                &mut board,
                &mut game_state,
                &mut deck,
                None,
                &mut positions,
            )
        });

        handle.join().unwrap()
    }
}

impl Agent for AlphaBetaAgent {
    fn generate_move(&self, board: &Board, game_state: &GameState, deck: &Deck) -> (Entity, Move) {
        let cards = deck.get_player_cards(game_state);

        let mut positions = 0;

        let res1 = self.alpha_beta(
            0,
            std::i32::MIN,
            std::i32::MAX,
            &mut board.clone(),
            &mut game_state.clone(),
            &mut deck.clone(),
            None,
            &mut positions,
        );

        let res2 = self.generate_move(board, game_state, deck);

        let result = if res1.1 > res2.1 {
            res1
        } else {
            res2
        };

        log::info!("Got res1: {:?} and res2 {:?}", res1, res2);
        log::info!("Evaluation score: {:?}", result.1);
        log::info!("Analyzed over {:?} positions", positions);

        (cards[0].0, result.0.unwrap())
    }

    fn clone_dyn(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}
