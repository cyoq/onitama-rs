use crate::components::{card_index::CardIndex, coordinates::Coordinates};

#[derive(Debug, Clone, Copy)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Clone, Copy)]
pub struct CardTriggerEvent(pub CardIndex);
