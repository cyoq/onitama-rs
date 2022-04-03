use bevy::prelude::Component;

use crate::{resources::card::Card, bounds::Bounds2};

#[derive(Debug, Clone, Component)]
pub struct CardBoard<'a> {
    card: Card<'a>,
    bounds: Bounds2
}
