use crate::{
    components::{board_tile::BoardTile, coordinates::Coordinates, pieces::Piece},
    events::{
        BotMakeMoveEvent, CardSwapEvent, GenerateBotMoveEvent, NextTurnEvent,
        ProcessWinConditionEvent,
    },
    resources::{
        board::Board, board_assets::BoardAssets, deck::Deck, game_state::GameState,
        tile_map::MoveResult,
    },
    BoardPlugin,
};
use bevy::{log, prelude::*};

pub fn generate_bot_move(
    board: Res<Board>,
    game_state: Res<GameState>,
    deck: Res<Deck>,
    mut random_bot_move_rdr: EventReader<GenerateBotMoveEvent>,
    mut bot_make_move_ewr: EventWriter<BotMakeMoveEvent>,
) {
    for _ in random_bot_move_rdr.iter() {
        let current_player = game_state.get_current_player();
        let (card, mov) = current_player
            .agent
            .generate_move(&board, &game_state, &deck);

        log::info!(
            "Bot move is {:?} and used card is {:?}",
            mov,
            deck.cardboards.get(&card).unwrap().card.name
        );
        bot_make_move_ewr.send(BotMakeMoveEvent {
            mov: mov,
            card_used: card,
        });
    }
}

pub fn bot_make_move<T>(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    tiles_q: Query<(Entity, &Coordinates), With<BoardTile>>,
    children_q: Query<&Children, With<BoardTile>>,
    pieces_q: Query<&Piece>,
    mut bot_make_move_rdr: EventReader<BotMakeMoveEvent>,
    mut card_swap_ewr: EventWriter<CardSwapEvent>,
    mut next_turn_ewr: EventWriter<NextTurnEvent>,
    mut process_win_condition_ewr: EventWriter<ProcessWinConditionEvent>,
) {
    for event in bot_make_move_rdr.iter() {
        let move_result = board.tile_map.make_a_move(event.mov.from, event.mov.to);

        #[cfg(feature = "debug")]
        {
            log::info!("{}", board.tile_map.console_output());
            log::info!("Move result is: {:?}", move_result);
        }

        let mut from_tile_entity = None;
        for (parent, coords) in tiles_q.iter() {
            if *coords == event.mov.from {
                from_tile_entity = Some(parent);
                break;
            }
        }

        if from_tile_entity == None {
            log::warn!("Could not find a tile with coords: {:?}", event.mov);
            return;
        }

        // clear the sprite of old selected piece
        // we should have a selected piece for sure
        let mut piece = None;
        if let Ok((parent, _)) = tiles_q.get(from_tile_entity.unwrap()) {
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
                && *coords == event.mov.to
            {
                if let Ok(children) = children_q.get(parent) {
                    for child_entity in children.iter() {
                        commands.entity(*child_entity).despawn_recursive();
                        commands.entity(parent).remove::<Piece>();
                    }
                }
            }

            if *coords == event.mov.to {
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

        card_swap_ewr.send(CardSwapEvent(event.card_used));
        process_win_condition_ewr.send(ProcessWinConditionEvent(move_result));
        next_turn_ewr.send(NextTurnEvent);
    } // event loop
}
