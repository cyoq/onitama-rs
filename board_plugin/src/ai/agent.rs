use core::fmt::Debug;

use crate::{components::coordinates::Coordinates, resources::board::Board};

pub trait Agent: Debug + Sync {
    fn make_move(&self, curr_pos: &Coordinates, board: &Board) -> Coordinates;
}
