use bevy::prelude::*;

use crate::{bounds::Bounds2, resources::card::Card};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Component)]
pub struct CardBoard<'a> {
    #[cfg_attr(feature = "debug", inspectable(ignore))]
    pub card: Card<'a>,
    pub bounds: Bounds2,
}

impl<'a> CardBoard<'a> {
    pub fn in_bounds(&self, window: &Window, position: Vec2) -> bool {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        self.bounds.in_bounds(position)
    }
}
