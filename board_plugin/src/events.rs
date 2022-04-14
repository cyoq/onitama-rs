use crate::{components::{card_index::CardIndex, coordinates::Coordinates}, resources::tile_map::MoveResult};
use bevy::prelude::*;

// TODO: describe each event action

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

#[derive(Debug, Clone, Copy)]
pub struct MovePieceEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct CardSwapEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct MirrorCardEvent(pub Entity);

#[derive(Debug, Clone, Copy)]
pub struct ProcessWinConditionEvent(pub MoveResult);