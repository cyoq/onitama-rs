use std::time::Duration;

use crate::components::guide_text_timer::GuideTextTimer;
use crate::events::{ChangeGuideTextEvent, NoCardSelectedEvent, TileTriggerEvent};
use crate::resources::selected::SelectedCard;
use crate::Board;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        // If mouse button is pressed
        if let ElementState::Pressed = event.state {
            // get the current mouse position in the window
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                // if mouse was pressed on the tile map
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Pressed to tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn color_selected_tile(
    mut commands: Commands,
    selected_card: Res<SelectedCard>,
    mut tile_trigger_event_rdr: EventReader<TileTriggerEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
    mut no_card_selected_ewr: EventWriter<NoCardSelectedEvent>,
) {
    for event in tile_trigger_event_rdr.iter() {
        if selected_card.0 == None {
            change_guide_text_ewr.send(ChangeGuideTextEvent {
                text: "Please, select a card first!".to_owned(),
            });
            commands.spawn().insert(GuideTextTimer {
                old_text: "Red to move. Select a card!".to_owned(),
                timer: Timer::new(Duration::from_secs(1), false),
            });
            no_card_selected_ewr.send(NoCardSelectedEvent);
            return;
        }
    }
}
