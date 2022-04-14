use bevy::{log, prelude::EventReader};

use crate::{
    components::{
        guide_text_timer::GuideTextTimer,
        texts::{GuideText, TurnText},
    },
    events::ChangeGuideTextEvent,
    resources::{
        board_assets::BoardAssets,
        game_state::{GameState, PlayerColor},
    },
};

use bevy::prelude::*;

pub fn process_guide_text(
    mut change_guide_text_rdr: EventReader<ChangeGuideTextEvent>,
    // query all children of the parents with GuideText component(mut be only one parent)
    parents: Query<&Children, With<GuideText>>,
    // get children of that parent
    mut children: Query<&mut Text>,
    board_assets: Res<BoardAssets>,
) {
    for event in change_guide_text_rdr.iter() {
        // iterate through the parents children
        for children_components in parents.iter() {
            // get entities of the children
            for child_entity in children_components.iter() {
                // Get the needed component
                match children.get_mut(*child_entity) {
                    Ok(mut text) => {
                        text.sections = vec![TextSection {
                            value: event.text.clone(),
                            style: TextStyle {
                                color: Color::WHITE,
                                font: board_assets.font.clone(),
                                font_size: board_assets.guide_text_size,
                            },
                        }];
                        log::info!("Changed a guide text to {}", event.text);
                    }
                    Err(e) => {
                        log::error!("An error in changing guide text occured: {:?}", e);
                        continue;
                    }
                }
            }
        }
    }
}

pub fn process_guide_text_change_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_q: Query<(Entity, &mut GuideTextTimer)>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
) {
    for (entity, mut timer) in timer_q.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(entity).despawn();
            change_guide_text_ewr.send(ChangeGuideTextEvent {
                text: timer.old_text.clone(),
            });
        }
    }
}

pub fn change_turn_text(
    game_state: Res<GameState<'static>>,
    board_assets: Res<BoardAssets>,
    parents_q: Query<&Children, With<TurnText>>,
    mut text_q: Query<&mut Text>,
) {
    if game_state.is_changed() {
        let (value, color) = match game_state.curr_color {
            PlayerColor::Red => (format!("Red turn: {}", game_state.turn), Color::RED),
            PlayerColor::Blue => (format!("Blue turn: {}", game_state.turn), Color::BLUE),
        };
        for children_components in parents_q.iter() {
            for child_entity in children_components.iter() {
                match text_q.get_mut(*child_entity) {
                    Ok(mut text) => {
                        text.sections = vec![TextSection {
                            value: value.clone(),
                            style: TextStyle {
                                color: color,
                                font: board_assets.font.clone(),
                                font_size: board_assets.turn_text_size,
                            },
                        }];
                        log::info!("Changed a turn text to {}", value);
                    }
                    Err(e) => {
                        log::error!("An error in changing guide text occured: {:?}", e);
                        continue;
                    }
                }
            }
        }
    }
}