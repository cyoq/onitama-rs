use std::time::Duration;

use crate::components::coordinates::Coordinates;
use crate::components::guide_text_timer::GuideTextTimer;
use crate::components::pieces::{Piece, PieceColor, PieceKind};
use crate::events::{
    ChangeGuideTextEvent, ColorSelectedPiece, NoCardSelectedEvent, PieceSelectEvent,
    ResetSelectedPieceColor,
};
use crate::resources::board_assets::BoardAssets;
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
    mut selected_piece: ResMut<SelectedPiece>,
    // pieces_q: Query<(Entity, &Children), With<Piece>>,
    pieces_q: Query<(Entity, &Coordinates), With<Piece>>,
    mut color_selected_piece_ewr: EventWriter<ColorSelectedPiece>,
    mut reset_sselected_piece_color_ewr: EventWriter<ResetSelectedPieceColor>,
    mut tile_trigger_event_rdr: EventReader<PieceSelectEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
    mut no_card_selected_ewr: EventWriter<NoCardSelectedEvent>,
) {
    for event in tile_trigger_event_rdr.iter() {
        // if no cards selected, do not allow to choose a piece
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

        for (parent, coords) in pieces_q.iter() {
            // check if the event coordinates are equal to the one of the pieces coordinates
            if event.0 == *coords {
                if selected_piece.entity != None {
                    reset_sselected_piece_color_ewr
                        .send(ResetSelectedPieceColor(selected_piece.entity.unwrap()));
                }

                // Do not rerender the same selected piece
                if let Some(entity) = selected_piece.entity {
                    if entity == parent {
                        return;
                    }
                }

                log::info!("Coloring piece tile on coordinates: {:?}", coords);

                color_selected_piece_ewr.send(ColorSelectedPiece(parent));

                selected_piece.entity = Some(parent);
                break;
            }
        }
    }
}

pub fn color_selected_piece(
    board_assets: Res<BoardAssets>,
    pieces_q: Query<&Children, With<Piece>>,
    mut sprites: Query<&mut Sprite>,
    mut color_selected_piece_rdr: EventReader<ColorSelectedPiece>,
) {
    for event in color_selected_piece_rdr.iter() {
        if let Ok(children) = pieces_q.get(event.0) {
            for child_entity in children.iter() {
                match sprites.get_mut(*child_entity) {
                    Ok(mut sprite) => {
                        sprite.color = board_assets.selected_piece_material.color;
                        break;
                    }
                    e => log::error!("Error while coloring the piece tile: {:?}", e),
                }
            }
        }
    }
}

pub fn reset_selected_piece_color(
    board_assets: Res<BoardAssets>,
    pieces_q: Query<(&Children, &Piece)>,
    mut sprites: Query<&mut Sprite>,
    mut reset_selected_piece_color_rdr: EventReader<ResetSelectedPieceColor>,
) {
    for event in reset_selected_piece_color_rdr.iter() {
        if let Ok((children, piece)) = pieces_q.get(event.0) {
            for child_entity in children.iter() {
                match sprites.get_mut(*child_entity) {
                    Ok(mut sprite) => {
                        sprite.color = match (piece.color, piece.kind) {
                            (PieceColor::Red, PieceKind::Pawn) => {
                                board_assets.red_pawn_material.color
                            }
                            (PieceColor::Red, PieceKind::King) => {
                                board_assets.red_king_material.color
                            }
                            (PieceColor::Blue, PieceKind::Pawn) => {
                                board_assets.blue_pawn_material.color
                            }
                            (PieceColor::Blue, PieceKind::King) => {
                                board_assets.blue_king_material.color
                            }
                        };
                        break;
                    }
                    Err(e) => log::error!("Error while coloring the piece tile: {:?}", e),
                }
            }
        }
    }
}
