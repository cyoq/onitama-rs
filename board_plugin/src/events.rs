use crate::components::{card_index::CardIndex, coordinates::Coordinates};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct ChangeGuideTextEvent {
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceSelectEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct ResetSelectedPieceColor(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct ColorSelectedPiece(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct CardTriggerEvent(pub CardIndex);

#[derive(Debug, Clone, Copy)]
pub struct ResetSelectedCardColorEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct ColorSelectedCardEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct NoCardSelectedEvent;
