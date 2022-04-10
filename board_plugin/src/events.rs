use crate::components::{card_index::CardIndex, coordinates::Coordinates};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct ChangeGuideText {
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct CardTriggerEvent(pub CardIndex);

#[derive(Clone, Copy)]
pub struct ResetSelectedCardColor(pub Entity);

#[derive(Clone, Copy)]
pub struct ColorSelectedCard(pub Entity);

#[derive(Clone, Copy)]
pub struct NoCardSelected;