use crate::components::background::Background;
use crate::components::card_board::CardOwner;
use crate::components::card_index::CardIndex;
use crate::events::{
    ColorSelectedCardEvent, NoCardSelectedEvent, ResetAllowedMovesEvent,
    ResetSelectedCardColorEvent, ResetSelectedPieceColorEvent,
};
use crate::resources::board::Board;
use crate::resources::board_assets::BoardAssets;
use crate::resources::deck::Deck;
use crate::resources::game_state::GameState;
use crate::resources::selected::{SelectedCard, SelectedPiece};
use bevy::log;
use bevy::prelude::*;

pub fn card_selection_handling(
    board: Res<Board>,
    game_state: Res<GameState<'static>>,
    mut selected_card: ResMut<SelectedCard>,
    mut selected_piece: ResMut<SelectedPiece>,
    deck: Res<Deck<'static>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    colors_q: Query<&CardOwner>,
    mut color_selected_card_ewr: EventWriter<ColorSelectedCardEvent>,
    mut reset_selected_card_color_ewr: EventWriter<ResetSelectedCardColorEvent>,
    mut reset_selected_piece_color_ewr: EventWriter<ResetSelectedPieceColorEvent>,
    mut reset_allowed_moves_ewr: EventWriter<ResetAllowedMovesEvent>,
) {
    let window = windows.get_primary().unwrap();

    let mut was_card_selected = false;

    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        let position = window.cursor_position();

        // Do not reset the selected card if the mouse position is within the board
        if let Some(pos) = position {
            if board.in_bounds(&window, pos) {
                return;
            }
        }

        let curr_color = game_state.curr_color;

        for (card_board_entity, card_board) in deck.cardboards.iter() {
            if let Some(pos) = position {
                if card_board.in_bounds(&window, pos) {
                    if let Ok(card_owner) = colors_q.get(*card_board_entity) {
                        if !card_owner.does_belong_to_player(&curr_color) {
                            log::info!("Cannot select a card with a different color!");
                            return;
                        }
                    }

                    // Check if there is an already selected card. Clear its color
                    if let Some(entity) = selected_card.entity {
                        // skip rerendering the same selected card
                        if entity == *card_board_entity {
                            was_card_selected = true;
                            continue;
                        }

                        reset_selected_card_color_ewr
                            .send(ResetSelectedCardColorEvent(selected_card.entity.unwrap()));
                        selected_card.entity = None;

                        // Reset the selected piece if the card was changed
                        if selected_piece.entity != None {
                            reset_selected_piece_color_ewr
                                .send(ResetSelectedPieceColorEvent(selected_piece.entity.unwrap()));
                            reset_allowed_moves_ewr.send(ResetAllowedMovesEvent);
                            selected_piece.clear();
                        }
                    }
                    // Set a new selected card
                    selected_card.entity = Some(*card_board_entity);
                    color_selected_card_ewr.send(ColorSelectedCardEvent(*card_board_entity));
                    was_card_selected = true;
                }
            }
        }

        if !was_card_selected && selected_card.entity != None {
            reset_selected_card_color_ewr
                .send(ResetSelectedCardColorEvent(selected_card.entity.unwrap()));
            selected_card.entity = None;
            // Reset the selected piece
            if selected_piece.entity != None {
                reset_selected_piece_color_ewr
                    .send(ResetSelectedPieceColorEvent(selected_piece.entity.unwrap()));
                reset_allowed_moves_ewr.send(ResetAllowedMovesEvent);
                selected_piece.clear();
            }
        }
        log::info!("Selected card entity: {:?}", selected_card.entity);
    }
}

pub fn color_selected_card(
    mut sprites: Query<&mut Sprite, With<Background>>,
    parents: Query<(Entity, &Children), With<CardIndex>>,
    mut color_selected_card_rdr: EventReader<ColorSelectedCardEvent>,
    board_assets: Res<BoardAssets>,
) {
    for event in color_selected_card_rdr.iter() {
        if let Ok((_, children)) = parents.get(event.0) {
            for child in children.iter() {
                if let Ok(mut sprite) = sprites.get_mut(*child) {
                    sprite.color = board_assets.selected_card_material.color;
                    break;
                }
            }
        }
    }
}

pub fn reset_selected_card_color(
    mut sprites: Query<&mut Sprite, With<Background>>,
    parents: Query<(Entity, &Children), With<CardIndex>>,
    mut color_selected_card_rdr: EventReader<ResetSelectedCardColorEvent>,
) {
    for event in color_selected_card_rdr.iter() {
        if let Ok((_, children)) = parents.get(event.0) {
            for child in children.iter() {
                if let Ok(mut sprite) = sprites.get_mut(*child) {
                    sprite.color = Color::WHITE;
                    break;
                }
            }
        }
    }
}

pub fn blink_non_selected_card(
    parents: Query<&Children, With<CardIndex>>,
    mut sprites: Query<&mut Sprite, With<Background>>,
    mut no_card_selected_rdr: EventReader<NoCardSelectedEvent>,
) {
    let colors = [Color::YELLOW, Color::WHITE, Color::YELLOW, Color::WHITE];
    for _ in no_card_selected_rdr.iter() {
        log::info!("Changing colors");
        for children_components in parents.iter() {
            for child_entity in children_components.iter() {
                match sprites.get_mut(*child_entity) {
                    Ok(mut sprite) => {
                        for color in colors.iter() {
                            sprite.color = *color;
                        }
                    }
                    Err(e) => log::info!("Was not able to query a sprite: {:?}", e),
                }
            }
        }
    }
}
