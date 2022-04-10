use std::time::Duration;

use crate::components::background::Background;
use crate::components::card_index::CardIndex;
use crate::events::{ColorSelectedCard, NoCardSelected, ResetSelectedCardColor};
use crate::resources::board::Board;
use crate::resources::board_assets::BoardAssets;
use crate::resources::deck::Deck;
use crate::resources::selected::SelectedCard;
use bevy::log;
use bevy::prelude::*;

pub fn card_selection_handling(
    board: Res<Board>,
    mut selected_card: ResMut<SelectedCard>,
    deck: Res<Deck<'static>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut color_selected_card_ewr: EventWriter<ColorSelectedCard>,
    mut reset_selected_card_color_ewr: EventWriter<ResetSelectedCardColor>,
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

        for card_board in deck.cardboards.iter() {
            if let Some(pos) = position {
                if card_board.in_bounds(&window, pos) {
                    // Check if there is an already selected card. Clear its color
                    if let Some(entity) = selected_card.0 {
                        // skip rerendering the same selected card
                        if entity == card_board.entity {
                            was_card_selected = true;
                            continue;
                        }

                        reset_selected_card_color_ewr
                            .send(ResetSelectedCardColor(selected_card.0.unwrap()));
                        selected_card.0 = None;
                    }
                    // Set a new selected card
                    selected_card.0 = Some(card_board.entity);
                    color_selected_card_ewr.send(ColorSelectedCard(card_board.entity));
                    was_card_selected = true;
                }
            }
        }

        if !was_card_selected && selected_card.0 != None {
            reset_selected_card_color_ewr.send(ResetSelectedCardColor(selected_card.0.unwrap()));
            selected_card.0 = None;
        }
        log::info!("Selected card entity: {:?}", selected_card.0);
    }
}

pub fn color_selected_card(
    mut sprites: Query<&mut Sprite, With<Background>>,
    parents: Query<(Entity, &Children), With<CardIndex>>,
    mut color_selected_card_rdr: EventReader<ColorSelectedCard>,
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
    mut color_selected_card_rdr: EventReader<ResetSelectedCardColor>,
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
    mut no_card_selected_rdr: EventReader<NoCardSelected>,
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
