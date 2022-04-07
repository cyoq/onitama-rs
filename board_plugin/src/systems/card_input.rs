use crate::components::background::Background;
use crate::components::card_board::CardBoard;
use crate::components::card_index::CardIndex;
use crate::events::{ColorSelectedCard, ResetSelectedCardColor};
use crate::resources::deck::Deck;
use crate::resources::selected_card::SelectedCard;
use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;

pub fn card_input_handling<'a>(
    // mut selected_card: ResMut<SelectedCard>,
    mut commands: Commands,
    windows: Res<Windows>,
    deck: Res<Deck<'static>>,
    // mut sprites: Query<&mut Sprite>,
    mut sprites: Query<(Entity, &mut Sprite), With<Background>>,
    mut parents_query: Query<(Entity, &Children), With<CardIndex>>,
    // mut transforms: Query<(&mut Transform, &CardIndex)>,
    // mut colors: Query<(Entity, &mut Sprite, &Background)>,
    mut button_evr: EventReader<MouseButtonInput>,
    // mut card_trigger_ewr: EventWriter<CardTriggerEvent>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        // If mouse button is pressed
        if let ElementState::Pressed = event.state {
            // get the current mouse position in the window
            let position = window.cursor_position();
            for card_board in deck.cardboards.iter() {
                if let Some(pos) = position {
                    log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);

                    if card_board.in_bounds(&window, pos) {
                        match event.button {
                            MouseButton::Left => {
                                log::info!("Pressed to card on index");
                                log::info!("Entity: {:?}", card_board.entity);
                                log::info!("Sprites: {:?}", sprites.iter().collect::<Vec<_>>());

                                log::info!(
                                    "Parents and children: {:?}",
                                    parents_query.iter().collect::<Vec<_>>()
                                );

                                if let Ok((_, children)) = parents_query.get(card_board.entity) {
                                    log::info!(
                                        "Pressed to card on entity: {:?}",
                                        card_board.entity
                                    );
                                    for child in children.iter() {
                                        log::info!("Got child: {:?}", child);
                                        if let Ok(mut sprite) = sprites.get_mut(*child) {
                                            sprite.1.color = Color::YELLOW;
                                            break;
                                        }
                                    }
                                }

                                // log::info!(
                                //     "Transforms: {:?}",
                                //     transforms.iter().collect::<Vec<_>>()
                                // );
                                // if let Ok((mut transform, _)) = transforms.get_mut(card_board.entity) {
                                //     log::info!("Transform is {:?}", transform);
                                //     transform.translation = Vec3::new(0., 0., 0.);
                                // }
                                // (*sprites.get_mut(card_board.entity).unwrap()).color =
                                //     Color::PURPLE;

                                // selected_card.0 = Some(entity);
                                // card_trigger_ewr.send(CardTriggerEvent(0));
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

pub fn card_selection_handling(
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
    }
}

pub fn color_selected_card(
    mut sprites: Query<&mut Sprite, With<Background>>,
    parents: Query<(Entity, &Children), With<CardIndex>>,
    mut color_selected_card_rdr: EventReader<ColorSelectedCard>,
) {
    for event in color_selected_card_rdr.iter() {
        if let Ok((_, children)) = parents.get(event.0) {
            for child in children.iter() {
                log::info!("Got child: {:?}", child);
                if let Ok(mut sprite) = sprites.get_mut(*child) {
                    sprite.color = Color::rgb_u8(255, 154, 154);
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
                log::info!("Got child: {:?}", child);
                if let Ok(mut sprite) = sprites.get_mut(*child) {
                    sprite.color = Color::WHITE;
                    break;
                }
            }
        }
    }
}
