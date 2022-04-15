use bevy::{log, prelude::*};

use crate::{
    events::{
        ChangeGuideTextEvent, GenerateBotMoveEvent, NextTurnEvent, ProcessWinConditionEvent,
        TurnProcessEvent,
    },
    resources::{
        app_state::AppState,
        game_state::{GameState, PlayerColor, PlayerType},
        tile_map::MoveResult,
    },
};

pub fn turn_process(
    game_state: Res<GameState<'static>>,
    mut turn_process_rdr: EventReader<TurnProcessEvent>,
    mut bot_move_ewr: EventWriter<GenerateBotMoveEvent>,
) {
    for _ in turn_process_rdr.iter() {
        let curr_player = game_state.get_current_player();

        match curr_player.player_type {
            PlayerType::Human => break,
            PlayerType::Random => bot_move_ewr.send(GenerateBotMoveEvent),
            PlayerType::AlphaBeta => bot_move_ewr.send(GenerateBotMoveEvent),
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

pub fn process_win_condition(
    mut app_state: ResMut<State<AppState>>,
    game_state: Res<GameState<'static>>,
    mut check_win_condition_rdr: EventReader<ProcessWinConditionEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
) {
    for event in check_win_condition_rdr.iter() {
        let mut is_end = false;

        if event.0 == MoveResult::Win {
            let color = match game_state.curr_color {
                PlayerColor::Red => "Red".to_owned(),
                PlayerColor::Blue => "Blue".to_owned(),
            };
            change_guide_text_ewr.send(ChangeGuideTextEvent {
                text: format!("{} has won!", color),
            });
            is_end = true;
        }

        if game_state.turn > 200 {
            change_guide_text_ewr.send(ChangeGuideTextEvent {
                text: format!("It is a tie!"),
            });
            is_end = true;
        }

        if is_end {
            app_state.set(AppState::Out).unwrap();
        }
    }
}
