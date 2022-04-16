use rand::{thread_rng, Rng};

use super::card::{Card, CARDS};

// Deck which contains the cards used in the game
#[derive(Debug)]
pub struct PhysicalDeck {
    pub cards: Vec<Card>,
}

impl PhysicalDeck {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(5),
        }
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn take_random_cards(&mut self) {
        let mut rng = thread_rng();
        let mut indices: Vec<u8> = Vec::with_capacity(5);

        while indices.len() != 5 {
            let index = rng.gen_range(0..CARDS.len());
            if indices.contains(&(index as u8)) {
                continue;
            }
            indices.push(index as u8);
        }

        self.take_cards_from_indices(&indices);
    }

    pub fn take_cards_from_indices(&mut self, indices: &Vec<u8>) {
        assert!(indices.len() == 5);
        for index in indices.iter() {
            self.cards.push(CARDS[*index as usize].clone());
        }

        // Reversing because red should get the cards that are in the end of the array
        self.cards.reverse();
    }

    pub fn take_some_random_cards(&mut self, indices: &Vec<u8>) {
        assert!(indices.len() <= 5);
        let mut rng = thread_rng();

        let mut indices: Vec<u8> = indices.clone();

        while indices.len() != 5 - self.cards.len() {
            let index = rng.gen_range(0..CARDS.len());

            if indices.contains(&(index as u8)) {
                continue;
            }
            indices.push(index as u8);
        }

        for index in indices.iter() {
            self.cards.push(CARDS[*index as usize].clone());
        }

        // Reversing because red should get the cards that are in the end of the array
        self.cards.reverse();
    }
}
