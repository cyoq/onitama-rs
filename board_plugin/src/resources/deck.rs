use bevy::{prelude::Entity, utils::HashMap};

use crate::components::card_board::CardBoard;

use super::{
    card::Card,
    game_state::{GameState, PlayerColor},
};

// use super::card::{Card, CARDS};

const BLUE_PLAYER_FIRST_CARD: usize = 0;
const BLUE_PLAYER_SECOND_CARD: usize = 1;
pub const NEUTRAL_CARD_IDX: usize = 2;
const RED_PLAYER_FIRST_CARD: usize = 3;
const RED_PLAYER_SECOND_CARD: usize = 4;

#[derive(Debug, Clone)]
pub struct Deck<'a> {
    pub cardboards: HashMap<Entity, CardBoard<'a>>,
    pub cards: Vec<Entity>,
}

impl<'a> Deck<'a> {
    #[inline]
    pub fn get_player_cards(&self, game_state: &GameState) -> [(Entity, &Card<'a>); 2] {
        if game_state.curr_color == PlayerColor::Blue {
            return [
                (
                    self.cards[BLUE_PLAYER_FIRST_CARD],
                    &self
                        .cardboards
                        .get(&self.cards[BLUE_PLAYER_FIRST_CARD])
                        .unwrap()
                        .card,
                ),
                (
                    self.cards[BLUE_PLAYER_SECOND_CARD],
                    &self
                        .cardboards
                        .get(&self.cards[BLUE_PLAYER_SECOND_CARD])
                        .unwrap()
                        .card,
                ),
            ];
        }
        [
            (
                self.cards[RED_PLAYER_FIRST_CARD],
                &self
                    .cardboards
                    .get(&self.cards[RED_PLAYER_FIRST_CARD])
                    .unwrap()
                    .card,
            ),
            (
                self.cards[RED_PLAYER_SECOND_CARD],
                &self
                    .cardboards
                    .get(&self.cards[RED_PLAYER_SECOND_CARD])
                    .unwrap()
                    .card,
            ),
        ]
    }

    #[inline]
    pub fn swap_card_with_neutral(&mut self, card_idx: usize) {
        if card_idx < 2 {
            self.cardboards.get_mut(&self.cards[card_idx]).unwrap().card.is_mirrored = false;
            self.cardboards.get_mut(&self.cards[NEUTRAL_CARD_IDX]).unwrap().card.is_mirrored = true;
        }
        self.cards.swap(card_idx, NEUTRAL_CARD_IDX);
    }
}
