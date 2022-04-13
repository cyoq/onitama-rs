use bevy::{prelude::Entity, utils::HashMap};

use crate::components::card_board::CardBoard;

// use super::card::{Card, CARDS};

const BLUE_FIRST_CARD_IDX: usize = 0;
const BLUE_SECOND_CARD_IDX: usize = 1;
const NEUTRAL_CARD_IDX: usize = 2;
const RED_FIRST_CARD_IDX: usize = 3;
const RED_SECOND_CARD_IDX: usize = 4;

#[derive(Debug)]
pub struct Deck<'a> {
    pub cardboards: HashMap<Entity, CardBoard<'a>>,
}

// impl<'a> Deck<'a> {
//     pub fn new() -> Self {
//         let mut cards: Vec<Card> = Vec::with_capacity(5);
//         for i in 0..5 {
//             cards.push(CARDS[i].clone());
//         }
//         Self {
//             cards: cards.try_into().unwrap(),
//         }
//     }
// }
