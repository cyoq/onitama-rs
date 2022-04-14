use bevy::{ecs::system::Resource, prelude::EventWriter};

use super::agent::Agent;

#[derive(Debug)]
pub struct Human;

impl Agent for Human {}
