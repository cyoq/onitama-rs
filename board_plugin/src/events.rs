use crate::components::{card_index::CardIndex, coordinates::Coordinates};
use bevy::{prelude::*};

#[derive(Debug, Clone)]
pub struct ChangeGuideTextEvent {
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceSelectEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct ResetSelectedPieceColorEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct ColorSelectedPieceEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct CardTriggerEvent(pub CardIndex);

#[derive(Debug, Clone, Copy)]
pub struct ResetSelectedCardColorEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct ColorSelectedCardEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct NoCardSelectedEvent;

#[derive(Debug, Clone, Copy)]
pub struct GenerateAllowedMovesEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct ResetAllowedMovesEvent;

#[derive(Debug, Clone, Copy)]
pub struct RandomBotMoveEvent;

#[derive(Debug, Clone, Copy)]
pub struct NextTurnEvent;

#[derive(Debug, Clone, Copy)]
pub struct TurnProcessEvent;