use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Component)]
pub struct CardIndex(pub u8);
