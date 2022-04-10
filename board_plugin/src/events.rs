use crate::components::{card_index::CardIndex, coordinates::Coordinates};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct ChangeGuideTextEvent {
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct CardTriggerEvent(pub CardIndex);

#[derive(Clone, Copy)]
pub struct ResetSelectedCardColorEvent(pub Entity);

#[derive(Clone, Copy)]
pub struct ColorSelectedCardEvent(pub Entity);

#[derive(Clone, Copy)]
pub struct NoCardSelectedEvent;