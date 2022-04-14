use crate::{
    components::{board_tile::BoardTile, coordinates::Coordinates, pieces::Piece},
    events::{
        BotMakeMoveEvent, CardSwapEvent, NextTurnEvent, ProcessWinConditionEvent,
        RandomBotMoveEvent,
    },
    resources::{
        board::Board, board_assets::BoardAssets, deck::Deck, game_state::GameState,
        tile_map::MoveResult,
    },
    BoardPlugin,
};
use bevy::{log, prelude::*};
use rand::Rng;

pub fn generate_random_bot_move(
    board: Res<Board>,
    game_state: Res<GameState<'static>>,
    deck: Res<Deck<'static>>,
    mut random_bot_move_rdr: EventReader<RandomBotMoveEvent>,
    mut bot_make_move_ewr: EventWriter<BotMakeMoveEvent>,
) {
    for _ in random_bot_move_rdr.iter() {
        let all_moves = board
            .tile_map
            .generate_all_possible_moves(&game_state, &deck);
        let card_idx: usize = rand::thread_rng().gen_range(0..2);

        log::info!("All possible moves: ");
        for pmov in all_moves.iter() {
            println!(
                "card: {:?}",
                deck.cardboards.get(&pmov.card).unwrap().card.name,
            );
            for mov in pmov.moves.iter() {
                println!("Move: {:?}", mov);
            }
        }

        let moves = &all_moves[card_idx];
        let size = moves.moves.len();
        let mov_idx: usize = rand::thread_rng().gen_range(0..size);

        let mov = &moves.moves[mov_idx];
        log::info!("Random bot chose a move {:?}", mov);

        bot_make_move_ewr.send(BotMakeMoveEvent {
            mov: *mov,
            card_used: moves.card,
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
