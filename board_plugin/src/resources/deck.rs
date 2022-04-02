use super::card::{Card, CARDS};



#[derive(Debug)]
pub struct Deck<'a> {
    pub cards: [Card<'a>; 5]
}

impl<'a> Deck<'a> {

    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(5);
        for i in 0..5 {
            cards.push(CARDS[i].clone());
        }
        Self {
            cards: cards.try_into().unwrap()
        }
    }
}