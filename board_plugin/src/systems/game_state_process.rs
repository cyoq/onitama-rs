use bevy::{log, prelude::*};

use crate::{
    events::{ChangeGuideTextEvent, NextTurnEvent, RandomBotMoveEvent, TurnProcessEvent},
    resources::game_state::{GameState, PlayerColor, PlayerType},
};

pub fn turn_process(
    game_state: Res<GameState<'static>>,
    mut turn_process_rdr: EventReader<TurnProcessEvent>,
    mut random_bot_move_ewr: EventWriter<RandomBotMoveEvent>,
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

pub fn next_turn_event(
    mut game_state: ResMut<GameState<'static>>,
    mut next_turn_rdr: EventReader<NextTurnEvent>,
    mut turn_process_ewr: EventWriter<TurnProcessEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
) {
    for _ in next_turn_rdr.iter() {
        game_state.next_turn();

        let color = match game_state.curr_color {
            PlayerColor::Red => "Red".to_owned(),
            PlayerColor::Blue => "Blue".to_owned(),
        };
        change_guide_text_ewr.send(ChangeGuideTextEvent {
            text: format!("{} to move. Select a card.", color),
        });

        log::info!("GameState updated! {:?}", game_state);

        turn_process_ewr.send(TurnProcessEvent);
    }
}
