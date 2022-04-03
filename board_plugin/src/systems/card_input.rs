use crate::events::{TileTriggerEvent, CardTriggerEvent};
use crate::Board;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn card_input_handling(
    board: Res<Board>,
    windows: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut card_trigger_ewr: EventWriter<CardTriggerEvent>,
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
                            // card_trigger_ewr.send(CardTriggerEvent(0));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

