use crate::{events::RandomBotMoveEvent, resources::{game_state::GameState, board::Board}};
use bevy::prelude::*;

pub fn make_random_bot_move(
    board: Res<Board>,
    game_state: Res<GameState<'static>>,
    mut random_bot_move_rdr: EventReader<RandomBotMoveEvent>,
) {
    for _ in random_bot_move_rdr.iter() {}
}
