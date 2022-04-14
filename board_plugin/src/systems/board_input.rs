use std::time::Duration;

use crate::components::allowed_move::AllowedMove;
use crate::components::board_tile::BoardTile;
use crate::components::coordinates::Coordinates;
use crate::components::guide_text_timer::GuideTextTimer;
use crate::components::pieces::{Piece, PieceKind};
use crate::events::{
    CardSwapEvent, ChangeGuideTextEvent, ColorSelectedPieceEvent, GenerateAllowedMovesEvent,
    MovePieceEvent, NextTurnEvent, NoCardSelectedEvent, PieceSelectEvent, ProcessWinConditionEvent,
    ResetAllowedMovesEvent, ResetSelectedCardColorEvent, ResetSelectedPieceColorEvent,
};
use crate::resources::board::Board;
use crate::resources::board_assets::BoardAssets;
use crate::resources::deck::Deck;
use crate::resources::game_state::{GameState, PlayerColor, PlayerType};
use crate::resources::selected::{SelectedCard, SelectedPiece};
use crate::resources::tile_map::MoveResult;
use crate::BoardPlugin;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn input_handling(
    game_state: Res<GameState<'static>>,
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<PieceSelectEvent>,
) {
    // do not handle input when it is not a player turn
    if game_state.get_current_player().player_type != PlayerType::Human {
        return;
    }

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
    game_state: Res<GameState<'static>>,
    selected_card: Res<SelectedCard>,
    mut selected_piece: ResMut<SelectedPiece>,
    pieces_parents_q: Query<(Entity, &Coordinates), With<Piece>>,
    pieces_q: Query<&Piece>,
    allowed_moves_q: Query<&Coordinates, With<AllowedMove>>,
    mut color_selected_piece_ewr: EventWriter<ColorSelectedPieceEvent>,
    mut reset_sselected_piece_color_ewr: EventWriter<ResetSelectedPieceColorEvent>,
    mut tile_trigger_event_rdr: EventReader<PieceSelectEvent>,
    mut change_guide_text_ewr: EventWriter<ChangeGuideTextEvent>,
    mut no_card_selected_ewr: EventWriter<NoCardSelectedEvent>,
    mut generate_allowed_moves_ewr: EventWriter<GenerateAllowedMovesEvent>,
    mut reset_allowed_moves_ewr: EventWriter<ResetAllowedMovesEvent>,
    mut move_piece_ewr: EventWriter<MovePieceEvent>,
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

        // call a move piece event if the piece is being moved
        for allowed_move in allowed_moves_q.iter() {
            if *allowed_move == event.0 {
                log::info!("Pressed on the allowed move: {:?}", allowed_move);
                // reset the old selections
                reset_sselected_piece_color_ewr
                    .send(ResetSelectedPieceColorEvent(selected_piece.entity.unwrap()));
                reset_allowed_moves_ewr.send(ResetAllowedMovesEvent);

                // start a move process
                move_piece_ewr.send(MovePieceEvent(*allowed_move));

                return;
            }
        }

        for (parent, coords) in pieces_parents_q.iter() {
            // check if the event coordinates are equal to the one of the pieces coordinates
            if event.0 == *coords {
                // Do not rerender the same selected piece
                if let Some(entity) = selected_piece.entity {
                    if entity == parent {
                        return;
                    }
                }

                // Do not highlight opposite color selected
                if let Ok(piece) = pieces_q.get(parent) {
                    if piece.color != game_state.curr_color {
                        return;
                    }
                }

                // changing the piece we should reset the old one
                if selected_piece.entity != None {
                    reset_sselected_piece_color_ewr
                        .send(ResetSelectedPieceColorEvent(selected_piece.entity.unwrap()));
                    reset_allowed_moves_ewr.send(ResetAllowedMovesEvent);
                }

                log::info!("Coloring piece tile on coordinates: {:?}", coords);

                color_selected_piece_ewr.send(ColorSelectedPieceEvent(parent));
                generate_allowed_moves_ewr.send(GenerateAllowedMovesEvent(*coords));

                selected_piece.entity = Some(parent);
                selected_piece.coordinates = Some(*coords);
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
                            (PlayerColor::Red, PieceKind::Pawn) => {
                                board_assets.red_pawn_material.color
                            }
                            (PlayerColor::Red, PieceKind::King) => {
                                board_assets.red_king_material.color
                            }
                            (PlayerColor::Blue, PieceKind::Pawn) => {
                                board_assets.blue_pawn_material.color
                            }
                            (PlayerColor::Blue, PieceKind::King) => {
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
    game_state: Res<GameState<'static>>,
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
        let allowed_moves =
            board
                .tile_map
                .generate_allowed_moves(&event.0, &card_board.card, &game_state);

        log::info!("Allowed moves: {:?}", allowed_moves);

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

pub fn move_piece<T>(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut selected_card: ResMut<SelectedCard>,
    board_assets: Res<BoardAssets>,
    tiles_q: Query<(Entity, &Coordinates), With<BoardTile>>,
    children_q: Query<&Children, With<BoardTile>>,
    pieces_q: Query<&Piece>,
    mut move_piece_rdr: EventReader<MovePieceEvent>,
    mut reset_selected_card_ewr: EventWriter<ResetSelectedCardColorEvent>,
    mut card_swap_ewr: EventWriter<CardSwapEvent>,
    mut next_turn_ewr: EventWriter<NextTurnEvent>,
    mut process_win_condition_ewr: EventWriter<ProcessWinConditionEvent>,
) {
    // TODO: for a better handling of a piece movement, it could be better to use a bundle
    // with a piece and a sprite
    for event in move_piece_rdr.iter() {
        let move_result = board
            .tile_map
            .make_a_move(selected_piece.coordinates.unwrap(), event.0);

        #[cfg(feature = "debug")]
        {
            log::info!("{}", board.tile_map.console_output());
            log::info!("Move result is: {:?}", move_result);
        }

        // clear the sprite of old selected piece
        // we should have a selected piece for sure
        let mut piece = None;
        if let Ok((parent, _)) = tiles_q.get(selected_piece.entity.unwrap()) {
            if let Ok(children) = children_q.get(parent) {
                for child_entity in children.iter() {
                    commands.entity(*child_entity).despawn_recursive();
                    // save a piece type to know what to spawn on a new place
                    if let Ok(p) = pieces_q.get(parent) {
                        piece = Some(*p);
                    }
                    commands.entity(parent).remove::<Piece>();
                    break;
                }
            }
        }

        // set a piece on a new location
        for (parent, coords) in tiles_q.iter() {
            // despawn a captured figure
            if (move_result == MoveResult::Capture || move_result == MoveResult::Win)
                && *coords == event.0
            {
                if let Ok(children) = children_q.get(parent) {
                    for child_entity in children.iter() {
                        commands.entity(*child_entity).despawn_recursive();
                        commands.entity(parent).remove::<Piece>();
                    }
                }
            }

            if *coords == event.0 {
                let mut cmd = commands.entity(parent);
                BoardPlugin::<T>::spawn_a_piece(
                    piece,
                    &mut cmd,
                    &board_assets,
                    board.tile_size,
                    board.padding,
                );
                break;
            }
        }

        selected_piece.clear();
        card_swap_ewr.send(CardSwapEvent(selected_card.entity.unwrap()));
        reset_selected_card_ewr.send(ResetSelectedCardColorEvent(selected_card.entity.unwrap()));
        selected_card.entity = None;
        process_win_condition_ewr.send(ProcessWinConditionEvent(move_result));
        next_turn_ewr.send(NextTurnEvent);
    }
}
