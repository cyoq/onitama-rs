use crate::components::background::Background;
use crate::components::board_tile::BoardTile;
use crate::components::card_board::CardOwner;
use crate::components::card_index::CardIndex;
use crate::components::coordinates::Coordinates;
use crate::events::{
    CardSwapEvent, ColorSelectedCardEvent, MirrorCardEvent, NoCardSelectedEvent,
    ResetAllowedMovesEvent, ResetSelectedCardColorEvent, ResetSelectedPieceColorEvent,
};
use crate::resources::board::Board;
use crate::resources::board_assets::BoardAssets;
use crate::resources::deck::{Deck, NEUTRAL_CARD_IDX};
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

pub fn card_swap(
    mut deck: ResMut<Deck<'static>>,
    mut transform_q: Query<(&mut Transform, &mut CardIndex, &mut CardOwner)>,
    mut card_swap_rdr: EventReader<CardSwapEvent>,
    mut mirror_card_ewr: EventWriter<MirrorCardEvent>,
) {
    for event in card_swap_rdr.iter() {
        // saving the entity ids for swapping
        let neutral_entity = deck.cards[NEUTRAL_CARD_IDX];
        let swapping_entity = event.0;

        let is_mirrored = deck
            .cards
            .iter()
            .position(|e| *e == swapping_entity)
            .unwrap()
            < 2;
        log::info!("Is mirrored: {:?}", is_mirrored);

        let swapping_cardboard_bounds = deck.cardboards.get(&swapping_entity).unwrap().bounds;
        let neutral_cardboard_bounds = deck.cardboards.get(&neutral_entity).unwrap().bounds;

        // changing the bounds to make the appropriate checks when the card is moved from its place
        for (k, v) in deck.cardboards.iter_mut() {
            if *k == neutral_entity {
                v.bounds = swapping_cardboard_bounds;
                if is_mirrored {
                    v.card.is_mirrored = true;
                }
            } else if *k == swapping_entity {
                v.bounds = neutral_cardboard_bounds;
                if is_mirrored {
                    v.card.is_mirrored = false;
                }
            }
        }

        // reading the center card properties
        let (cen_transform, cen_card_index, cen_card_owner) = match transform_q.get(neutral_entity)
        {
            Ok(query) => query,
            Err(e) => {
                log::warn!("Error raised when swaping cards: {:?}", e);
                return;
            }
        };

        // preparing swapping card properties for the change
        let (mut sw_transform, mut sw_card_index, mut sw_card_owner) =
            match transform_q.get_mut(swapping_entity) {
                Ok(query) => query,
                Err(e) => {
                    log::warn!("Error raised when swaping cards: {:?}", e);
                    return;
                }
            };

        // swapping the cards in the global array
        deck.cards
            .swap(cen_card_index.0 as usize, sw_card_index.0 as usize);

        // saving temporary values for the swap
        let temp_transform = sw_transform.clone();
        let temp_card_index = sw_card_index.clone();
        let temp_cardowner = sw_card_owner.clone();

        *sw_transform = cen_transform.clone();
        *sw_card_index = cen_card_index.clone();
        *sw_card_owner = cen_card_owner.clone();

        // changing the center card
        let (mut cen_transform, mut cen_card_index, mut cen_card_owner) =
            match transform_q.get_mut(neutral_entity) {
                Ok(query) => query,
                Err(e) => {
                    log::warn!("Error raised when swaping cards: {:?}", e);
                    return;
                }
            };

        *cen_transform = temp_transform;
        *cen_card_index = temp_card_index;
        *cen_card_owner = temp_cardowner;
        if is_mirrored {
            mirror_card_ewr.send(MirrorCardEvent(neutral_entity));
            mirror_card_ewr.send(MirrorCardEvent(swapping_entity));
        }
    }
}

pub fn mirror_card(
    deck: Res<Deck<'static>>,
    board_assets: Res<BoardAssets>,
    tiles_q: Query<&Children, With<CardIndex>>,
    mut sprites_tiles_q: Query<
        (&Coordinates, &mut Sprite),
        (Without<Background>, Without<BoardTile>),
    >,
    mut mirror_card_rdr: EventReader<MirrorCardEvent>,
) {
    for event in mirror_card_rdr.iter() {
        let card = &deck.cardboards.get(&event.0).unwrap().card;

        let center = Coordinates { x: 2, y: 2 };
        let move_tiles = card
            .directions
            .iter()
            .map(|tuple| {
                if card.is_mirrored {
                    center + (tuple.0, -tuple.1)
                } else {
                    center + *tuple
                }
            })
            .collect::<Vec<_>>();

        if let Ok(children) = tiles_q.get(event.0) {
            for child in children.iter() {
                log::info!("got child: {:?}", child);
                match sprites_tiles_q.get_mut(*child) {
                    Ok((coordinates, mut sprite)) => {
                        let mut tile_color;

                        // highlight the center
                        if *coordinates == center {
                            tile_color = board_assets.deck_card_center_material.color;
                        } else {
                            tile_color = board_assets.tile_material.color;
                        }

                        // highlight possible moves
                        if move_tiles.contains(&coordinates) {
                            tile_color = board_assets.deck_card_allowed_move_material.color;
                        }

                        sprite.color = tile_color;
                    }
                    Err(e) => {
                        log::warn!("Incorrect query to the sprite while mirroring: {:?}", e);
                        continue;
                    }
                }
            }
        }
    }
}
