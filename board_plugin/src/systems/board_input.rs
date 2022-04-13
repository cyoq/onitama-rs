use std::time::Duration;

use crate::components::allowed_move::AllowedMove;
use crate::components::board_tile::BoardTile;
use crate::components::coordinates::Coordinates;
use crate::components::guide_text_timer::GuideTextTimer;
use crate::components::pieces::{Piece, PieceColor, PieceKind};
use crate::events::{
    ChangeGuideTextEvent, ColorSelectedPieceEvent, GenerateAllowedMovesEvent, NoCardSelectedEvent,
    PieceSelectEvent, ResetAllowedMovesEvent, ResetSelectedPieceColorEvent,
};
use crate::resources::board_assets::BoardAssets;
use crate::resources::deck::Deck;
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
    pieces_q: Query<(Entity, &Coordinates), With<Piece>>,
    mut color_selected_piece_ewr: EventWriter<ColorSelectedPieceEvent>,
    mut reset_sselected_piece_color_ewr: EventWriter<ResetSelectedPieceColorEvent>,
    mut tile_trigger_event_rdr: EventReader<PieceSelectEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
    mut no_card_selected_ewr: EventWriter<NoCardSelectedEvent>,
    mut generate_allowed_moves_ewr: EventWriter<GenerateAllowedMovesEvent>,
    mut reset_allowed_moves_ewr: EventWriter<ResetAllowedMovesEvent>,
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
                // changing the piece we should reset the old one
                if selected_piece.entity != None {
                    reset_sselected_piece_color_ewr
                        .send(ResetSelectedPieceColorEvent(selected_piece.entity.unwrap()));
                    reset_allowed_moves_ewr.send(ResetAllowedMovesEvent);
                }

                // Do not rerender the same selected piece
                if let Some(entity) = selected_piece.entity {
                    if entity == parent {
                        return;
                    }
                }

                log::info!("Coloring piece tile on coordinates: {:?}", coords);

                color_selected_piece_ewr.send(ColorSelectedPieceEvent(parent));
                generate_allowed_moves_ewr.send(GenerateAllowedMovesEvent(*coords));

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
    mut color_selected_piece_rdr: EventReader<ColorSelectedPieceEvent>,
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
    mut reset_selected_piece_color_rdr: EventReader<ResetSelectedPieceColorEvent>,
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

pub fn generate_allowed_moves(
    mut commands: Commands,
    board: Res<Board>,
    deck: Res<Deck<'static>>,
    selected_card: Res<SelectedCard>,
    mut tiles_q: Query<(Entity, &Coordinates, &mut Sprite), With<BoardTile>>,
    mut generate_allowed_moves_rdr: EventReader<GenerateAllowedMovesEvent>,
) {
    for event in generate_allowed_moves_rdr.iter() {
        let card_entity = match selected_card.entity {
            Some(card) => card,
            None => {
                log::warn!("Selected card entity was empty when generating allowed moves");
                return;
            }
        };

        let card_board = deck.cardboards.get(&card_entity).unwrap();
        let allowed_moves = board
            .tile_map
            .generate_allowed_moves(&event.0, &card_board.card);

        for (entity, coords, mut sprite) in tiles_q.iter_mut() {
            // TODO: different color for enemy piece
            // TODO: bug: when the same spot is colored, its color is resetted for the second time
            if allowed_moves.contains(coords) {
                sprite.color = Color::TOMATO;
                commands.entity(entity).insert(AllowedMove);
            }
        }
    }
}

pub fn reset_allowed_moves(
    mut commands: Commands,
    board_assets: Res<BoardAssets>,
    mut tiles_q: Query<(Entity, &mut Sprite), With<AllowedMove>>,
    mut reset_allowed_moves_event: EventReader<ResetAllowedMovesEvent>,
) {
    for _ in reset_allowed_moves_event.iter() {
        for (entity, mut sprite) in tiles_q.iter_mut() {
            sprite.color = board_assets.tile_material.color;
            commands.entity(entity).remove::<AllowedMove>();
        }
    }
}
