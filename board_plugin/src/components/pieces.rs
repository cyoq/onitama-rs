use bevy::prelude::Component;

use crate::resources::game::PlayerColor;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PieceKind {
    Pawn,
    King,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Piece {
    pub color: PlayerColor,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PlayerColor) -> Self {
        Self { kind, color }
    }

    #[inline]
    pub const fn enemy(&self) -> PlayerColor {
        match self.color {
            PlayerColor::Red => PlayerColor::Blue,
            PlayerColor::Blue => PlayerColor::Red,
        }
    }
}
