use bevy::{core::Timer, prelude::Component};

#[derive(Debug, Clone, Component)]
pub struct GuideTextTimer {
    pub old_text: String,
    pub timer: Timer,
}
