use std::time::Duration;

use crate::components::coordinates::Coordinates;
use crate::components::guide_text_timer::GuideTextTimer;
use crate::components::pieces::Piece;
use crate::events::{
    ChangeGuideTextEvent, ColorSelectedCardEvent, ColorSelectedPiece, NoCardSelectedEvent,
    PieceSelectEvent,
};
use crate::resources::selected::{SelectedCard, SelectedPiece};
use crate::Board;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<PieceSelectEvent>,
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
                            tile_trigger_ewr.send(PieceSelectEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn process_selected_tile(
    mut commands: Commands,
    selected_card: Res<SelectedCard>,
    // mut selected_piece: ResMut<SelectedPiece>,
    pieces_q: Query<(Entity, &Children), With<Piece>>,
    mut sprites: Query<&mut Sprite>,
    coordinates: Query<&Coordinates>,
    mut color_selected_piece: EventWriter<ColorSelectedPiece>,
    mut tile_trigger_event_rdr: EventReader<PieceSelectEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
    mut no_card_selected_ewr: EventWriter<NoCardSelectedEvent>,
) {
    for event in tile_trigger_event_rdr.iter() {
        if selected_card.entity == None {
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

        for (parent, children_component) in pieces_q.iter() {
            for child_entity in children_component.iter() {
                if let Ok(coords) = coordinates.get(parent) {
                    if *coords == event.0 {
                        log::info!("Coloring piece tile: {:?}", coords);
                        color_selected_piece.send(ColorSelectedPiece {
                            entity: *child_entity,
                            coords: *coords,
                        });
                        break;
                    }
                }
                // match (coordinates.get(parent), sprites.get_mut(*child_entity)) {
                //     (Ok(coords), Ok(mut sprite)) => {
                //         if *coords == event.0 {
                //             sprite.color = Color::AZURE;
                //             selected_piece.entity = Some(parent);
                //             break;
                //         }
                //     }
                //     _ => log::error!("Error retrieving sprite and coords for the piece: "),
                // }
            }
        }
    }
}

pub fn color_selected_piece(
    pieces_q: Query<(Entity, &Children), With<Piece>>,
    mut sprites: Query<&mut Sprite>,
    coordinates: Query<&Coordinates>,
    mut color_selected_piece_rdr: EventReader<ColorSelectedPiece>,
) {
    for event in color_selected_piece_rdr.iter() {
        // if let Ok((parent, children)) = pieces_q.get(event.entity) {
        //     for child_entity in children.iter() {
        //         match (coordinates.get(parent), sprites.get_mut(*child_entity)) {
        //             (Ok(coords), Ok(mut sprite)) => {
        //                 log::info!("Coords: {:?} event: {:?}", coords, event.coords);
        //                 if *coords == event.coords {
        //                     sprite.color = Color::AZURE;
        //                     break;
        //                 }
        //             }
        //             _ => log::error!("Error while coloring the piece tile: "),
        //         }
        //     }
        // }

        // if let Ok((parent, children)) = pieces_q.get(event.entity) {
        //     for child_entity in children.iter() {
        //         match sprites.get_mut(*child_entity) {
        //             Ok(mut sprite) => {
        //                 log::info!("Coords: {:?}", event.coords);
        //                 sprite.color = Color::AZURE;
        //                 break;
        //             }
        //             _ => log::error!("Error while coloring the piece tile: "),
        //         }
        //     }
        // }

        match sprites.get_mut(event.entity) {
            Ok(mut sprite) => {
                sprite.color = Color::AZURE;
                // break;
            }
            Err(e) => log::error!("Error while coloring the piece tile: {:?}", e),
        }
    }
}
