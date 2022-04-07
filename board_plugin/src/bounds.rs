use bevy::prelude::Vec2;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    #[inline]
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}
