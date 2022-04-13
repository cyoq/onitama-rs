use crate::{components::coordinates::Coordinates, resources::board::Board};

use super::agent::Agent;

#[derive(Debug)]
pub struct Human;

impl Agent for Human {
    fn make_move(
        &self,
        curr_pos: &Coordinates,
        board: &Board,
    ) -> crate::components::coordinates::Coordinates {
        Coordinates { x: 0, y: 0 }
    }
}
