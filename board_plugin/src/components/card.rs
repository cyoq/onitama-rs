#[derive(Debug, Clone, Copy)]
pub enum Edition {
    Original,
    SenseisPath,
}

#[derive(Debug, Clone, Component)]
pub struct Card<'a> {
    pub edition: Edition,
    pub directions: &'a [(i8, i8)],
    pub name: &'static str,
}

impl<'a> Card<'a> {
    const fn new(edition: Edition, directions: &'a [(i8, i8)], name: &'static str) -> Self {
        Self {
            edition,
            directions,
            name,
        }
    }
}

pub const CARDS: [Card; 5] = [
    Card::new(Edition::Original, &[(0, 2), (0, -1)], "Tiger"),
    Card::new(Edition::Original, &[(0, 1), (-2, 0), (2, 0)], "Crab"),
    Card::new(
        Edition::Original,
        &[(-1, 1), (-1, -1), (1, 1), (1, -1)],
        "Monkey",
    ),
    Card::new(Edition::Original, &[(0, 1), (-1, -1), (1, -1)], "Crane"),
    Card::new(
        Edition::Original,
        &[(-2, 1), (-1, -1), (2, 1), (1, -1)],
        "Dragon",
    ),
    // Card::new(Edition::Original, &[-1, 0, -1, 1, 1, 1, 1, 0], "Elephant"),
    // Card::new(Edition::Original, &[0, -1, -1, 1, 1, 1], "Mantis"),
    // Card::new(Edition::Original, &[0, 1, -1, 0, 1, 0], "Boar"),
    // Card::new(Edition::Original, &[-2, 0, -1, 1, 1, -1], "Frog"),
    // Card::new(Edition::Original, &[-1, 0, -1, 1, 1, 0, 1, -1], "Goose"),
    // Card::new(Edition::Original, &[0, 1, -1, 0, 0, -1], "Horse"),
    // Card::new(Edition::Original, &[-1, 1, -1, -1, 1, 0], "Eel"),
    // Card::new(Edition::Original, &[-1, -1, 1, 1, 2, 0], "Rabbit"),
    // Card::new(Edition::Original, &[-1, 0, -1, -1, 1, 0, 1, 1], "Rooster"),
    // Card::new(Edition::Original, &[0, 1, 0, -1, 1, 0], "Ox"),
    // Card::new(Edition::Original, &[-1, 0, 1, -1, 1, 1], "Cobra"),
];

// const TIGER: Card = Card::new(Edition::Original, &[0, 2, 0, -1], "Tiger");
// const CRAB: Card = Card::new(Edition::Original, &[0, 1, -2, 0, 2, 0], "Crab");
// const MONKEY: Card = Card::new(Edition::Original, &[-1, 1, -1, -1, 1, 1, 1, -1], "Monkey");
// const CRANE: Card = Card::new(Edition::Original, &[0, 1, -1, -1, 1, -1], "Crane");
// const DRAGON: Card = Card::new(Edition::Original, &[-2, 1, -1, -1, 2, 1, 1, -1], "Dragon");
// const ELEPHANT: Card = Card::new(Edition::Original, &[-1, 0, -1, 1, 1, 1, 1, 0], "Elephant");
// const MANTIS: Card = Card::new(Edition::Original, &[0, -1, -1, 1, 1, 1], "Mantis");
// const BOAR: Card = Card::new(Edition::Original, &[0, 1, -1, 0, 1, 0], "Boar");
// const FROG: Card = Card::new(Edition::Original, &[-2, 0, -1, 1, 1, -1], "Frog");
// const GOOSE: Card = Card::new(Edition::Original, &[-1, 0, -1, 1, 1, 0, 1, -1], "Goose");
// const HORSE: Card = Card::new(Edition::Original, &[0, 1, -1, 0, 0, -1], "Horse");
// const EEL: Card = Card::new(Edition::Original, &[-1, 1, -1, -1, 1, 0], "Eel");
// const RABBIT: Card = Card::new(Edition::Original, &[-1, -1, 1, 1, 2, 0], "Rabbit");
// const ROOSTER: Card = Card::new(Edition::Original, &[-1, 0, -1, -1, 1, 0, 1, 1], "Rooster");
// const OX: Card = Card::new(Edition::Original, &[0, 1, 0, -1, 1, 0], "Ox");
// const COBRA: Card = Card::new(Edition::Original, &[-1, 0, 1, -1, 1, 1], "Cobra");

use bevy::prelude::Component;
