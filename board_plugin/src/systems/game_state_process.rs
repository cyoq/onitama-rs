use bevy::prelude::*;

use crate::{events::{TurnProcessEvent, RandomBotMoveEvent}, resources::game_state::{GameState, PlayerType}};

pub fn turn_process(
    game_state: Res<GameState<'static>>,
    mut turn_process_rdr: EventReader<TurnProcessEvent>,
    mut random_bot_move_ewr: EventWriter<RandomBotMoveEvent>
) {
    for _ in turn_process_rdr.iter() {
        let curr_player = game_state.get_current_player();

        match curr_player.player_type {
            PlayerType::Human => break,
            PlayerType::Random => random_bot_move_ewr.send(RandomBotMoveEvent),
            PlayerType::AlphaBeta => (),
        }
    }
}
