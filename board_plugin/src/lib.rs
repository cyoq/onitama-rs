pub mod ai;
pub mod bounds;
pub mod button_plugin;
pub mod components;
pub mod events;
pub mod menu_plugin;
pub mod resources;
pub mod systems;

use bevy::ecs::schedule::StateData;
use bevy::ecs::system::EntityCommands;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::utils::{AHashExt, HashMap};
use components::background::Background;
use components::board_tile::BoardTile;
use components::coordinates::Coordinates;
use components::pieces::{Piece, PieceKind};
use events::TurnProcessEvent;
use resources::board::Board;
use resources::board_assets::BoardAssets;
use resources::board_options::BoardOptions;
use resources::card::Card;
use resources::deck_options::DeckOptions;
use resources::game_state::{GameState, PlayerColor};
use resources::physical_deck::PhysicalDeck;
use resources::selected::SelectedPlayers;

use crate::ai::agent::Agent;
use crate::ai::alpha_beta::AlphaBetaAgent;
use crate::ai::human::Human;
use crate::ai::random_agent::RandomAgent;
use crate::bounds::Bounds2;
use crate::components::card_board::{CardBoard, CardOwner};
use crate::components::card_index::CardIndex;
use crate::components::texts::{EvaluationText, GuideText, TurnText};
use crate::events::{
    BotMakeMoveEvent, CardSwapEvent, ChangeGuideTextEvent, ColorSelectedCardEvent,
    ColorSelectedPieceEvent, GenerateAllowedMovesEvent, GenerateBotMoveEvent, MirrorCardEvent,
    MovePieceEvent, NextTurnEvent, NoCardSelectedEvent, PieceSelectEvent, ProcessWinConditionEvent,
    ResetAllowedMovesEvent, ResetSelectedCardColorEvent, ResetSelectedPieceColorEvent,
};
use crate::menu_plugin::ListElement;
use crate::resources::board_options::TileSize;
use crate::resources::card::CARDS;
use crate::resources::deck::Deck;
use crate::resources::game_state::{Player, PlayerType};
use crate::resources::selected::{SelectedCard, SelectedPiece};
use crate::resources::text_handler::{EvaluationResult, TextHandler};
use crate::resources::tile_map::TileMap;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;

use PieceKind::*;
use PlayerColor::*;

pub struct BoardPlugin<T> {
    pub running_state: T,
    pub cleanup_state: T,
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        deck_options: Option<Res<DeckOptions>>,
        window: Res<WindowDescriptor>,
        selected_players: Res<SelectedPlayers>,
        board_assets: Res<BoardAssets>,
        physical_deck: Res<PhysicalDeck>,
    ) {
        let options = match board_options {
            Some(opt) => opt.clone(),
            None => Default::default(),
        };

        let deck_options = match deck_options {
            Some(opt) => opt.clone(),
            None => Default::default(),
        };

        let tile_map = TileMap::new();
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(&window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let board_entity = commands
            .spawn()
            .insert(Name::new("GameBoard"))
            .insert(Transform::from_translation(options.position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_board(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                );
            })
            .id();

        commands.insert_resource(Board {
            bounds: Bounds2 {
                position: options.position.xy(),
                size: board_size,
            },
            tile_size,
            padding: options.tile_padding,
            tile_map,
            entity: board_entity,
        });

        // Spawn boards with cards movements
        let tile_size = match deck_options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(&window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        log::info!("deck card tile size: {}", tile_size);

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );

        log::info!("one board size from the deck: {}", board_size);

        let offset = board_size / 4.;
        let deck_pos = Vec2::new(-board_size.x / 2. - offset.x, board_size.y + offset.y);

        let positions = [
            Vec2::new(deck_pos.x, deck_pos.y),
            Vec2::new(-deck_pos.x, deck_pos.y),
            Vec2::new(0., 0.),
            Vec2::new(deck_pos.x, -deck_pos.y),
            Vec2::new(-deck_pos.x, -deck_pos.y),
        ];

        // let mut cards = [
        //     CARDS[0].clone(),
        //     CARDS[1].clone(),
        //     CARDS[2].clone(),
        //     CARDS[3].clone(),
        //     CARDS[4].clone(),
        // ];

        let mut cards = physical_deck.cards.clone();

        cards[0].is_mirrored = true;
        cards[1].is_mirrored = true;

        let mut deck_container = HashMap::with_capacity(5);
        let mut card_entities = Vec::with_capacity(5);

        for i in 0..5 {
            let position = deck_options.position.xy() + positions[i];
            let card_board_entity = commands
                .spawn()
                .insert(Name::new(format!("Card {}", cards[i].name)))
                .insert(CardIndex(i as u8))
                .insert(Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    deck_options.position.z,
                )))
                .insert(GlobalTransform::default())
                .with_children(|parent| {
                    Self::spawn_deck_card_board(
                        parent,
                        board_size,
                        &cards[i],
                        &board_assets,
                        deck_options.tile_padding,
                        tile_size,
                    );
                })
                .id();

            if i == 0 || i == 1 {
                commands.entity(card_board_entity).insert(CardOwner::Blue);
            } else if i == 3 || i == 4 {
                commands.entity(card_board_entity).insert(CardOwner::Red);
            } else {
                commands
                    .entity(card_board_entity)
                    .insert(CardOwner::Neutral);
            }

            deck_container.insert(
                card_board_entity,
                CardBoard {
                    card: cards[i].clone(),
                    bounds: Bounds2 {
                        size: board_size,
                        position: positions[i] + deck_options.position.xy() - board_size / 2.,
                    },
                },
            );

            card_entities.push(card_board_entity);
        }

        commands.insert_resource(Deck {
            cardboards: deck_container,
            cards: card_entities,
        });

        commands.insert_resource(SelectedCard::default());
        commands.insert_resource(SelectedPiece::default());

        // Spawn a guide text
        let guide_text = commands
            .spawn()
            .insert(Name::new("Guide text"))
            .insert(GuideText)
            .insert(Transform::from_translation(Vec3::new(
                0.0,
                window.height / 2.4,
                1.,
            )))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                Self::spawn_text(
                    parent,
                    "Red to move. Select a card".to_owned(),
                    &board_assets,
                    board_assets.guide_text_size,
                    Vec2::new(0., 0.),
                    Color::WHITE,
                );
            })
            .id();

        // Spawn a turn text
        let turn_text = commands
            .spawn()
            .insert(Name::new("Turn text"))
            .insert(TurnText)
            .insert(Transform::from_translation(Vec3::new(
                -window.width / 2. + window.width / 8.,
                window.height / 2.4,
                1.,
            )))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                Self::spawn_text(
                    parent,
                    "Red turn: 0".to_owned(),
                    &board_assets,
                    board_assets.turn_text_size,
                    Vec2::new(0., 0.),
                    Color::RED,
                );
            })
            .id();

        let evaluation_result = EvaluationResult::default();

        // create evaluation text
        let evaluation_text = commands
            .spawn()
            .insert(Name::new("Evaluation text"))
            .insert(EvaluationText)
            .insert(Transform::from_translation(Vec3::new(
                -window.width / 2. + window.width / 8. + 10.,
                window.height / 2.4 - board_assets.turn_text_size,
                1.,
            )))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                Self::spawn_text(
                    parent,
                    evaluation_result.to_string(),
                    &board_assets,
                    board_assets.turn_text_size,
                    Vec2::new(0., 0.),
                    Color::WHITE,
                );
            })
            .id();

        commands.insert_resource(evaluation_result);

        commands.insert_resource(TextHandler {
            turn_text,
            guide_text,
            evaluation_text,
        });

        let red_agent: Box<dyn Agent> = match selected_players.red_player {
            PlayerType::Human => Box::new(Human),
            PlayerType::Random => Box::new(RandomAgent),
            PlayerType::AlphaBeta => Box::new(AlphaBetaAgent { max_depth: 7 }),
        };

        let blue_agent: Box<dyn Agent> = match selected_players.blue_player {
            PlayerType::Human => Box::new(Human),
            PlayerType::Random => Box::new(RandomAgent),
            PlayerType::AlphaBeta => Box::new(AlphaBetaAgent { max_depth: 7 }),
        };

        let red_player = Player {
            agent: red_agent,
            player_type: selected_players.red_player,
        };

        let blue_player = Player {
            agent: blue_agent,
            player_type: selected_players.blue_player,
        };

        commands.insert_resource(GameState::new(red_player, blue_player));
    }

    pub fn adaptive_tile_size(
        window: &WindowDescriptor,
        (min, max): (f32, f32),
        (width, height): (u8, u8),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    fn spawn_board(
        parent: &mut ChildBuilder,
        board: &TileMap,
        tile_size: f32,
        padding: f32,
        board_assets: &BoardAssets,
    ) {
        // reversing here, because bevy starts (0, 0) from the left bottom corner
        for (y, line) in board.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u8,
                    y: y as u8,
                };

                // print letters to the left of the board
                if coordinates.x == 0 {
                    Self::spawn_text(
                        parent,
                        String::from((101 - y) as u8 as char),
                        board_assets,
                        tile_size,
                        Vec2::new(
                            (x as f32 * tile_size) - (tile_size / 4.),
                            (y as f32 * tile_size) + (tile_size / 8.),
                        ),
                        Color::WHITE,
                    );
                }

                // print numbers below the board
                if coordinates.y == 0 {
                    Self::spawn_text(
                        parent,
                        (x + 1).to_string(),
                        board_assets,
                        tile_size,
                        Vec2::new(
                            (x as f32 * tile_size) + (tile_size / 2.),
                            (y as f32 * tile_size) - (tile_size / 1.5) - 10.,
                        ),
                        Color::WHITE,
                    );
                }

                // Creating a tile on the board
                let mut cmd = parent.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(tile_size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                // We add the `Coordinates` component to our tile entity
                .insert(coordinates)
                .insert(BoardTile);

                // Creating a pawn or a king square
                Self::spawn_a_piece(tile.piece, &mut cmd, board_assets, tile_size, padding);
            }
        }
    }

    pub fn spawn_a_piece(
        piece: Option<Piece>,
        cmd: &mut EntityCommands,
        board_assets: &BoardAssets,
        tile_size: f32,
        padding: f32,
    ) {
        if let Some(piece) = piece {
            match (piece.color, piece.kind) {
                (Blue, King) => {
                    cmd.insert(piece);
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.blue_king_material.color.clone(),
                                custom_size: Some(Vec2::splat(tile_size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: board_assets.blue_king_material.texture.clone(),
                            ..Default::default()
                        });
                    });
                }
                (Blue, Pawn) => {
                    cmd.insert(piece);
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.blue_pawn_material.color.clone(),
                                custom_size: Some(Vec2::splat(tile_size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: board_assets.blue_pawn_material.texture.clone(),
                            ..Default::default()
                        });
                    });
                }
                (Red, King) => {
                    cmd.insert(piece);
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.red_king_material.color.clone(),
                                custom_size: Some(Vec2::splat(tile_size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: board_assets.red_king_material.texture.clone(),
                            ..Default::default()
                        });
                    });
                }
                (Red, Pawn) => {
                    cmd.insert(piece);
                    cmd.with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.red_pawn_material.color.clone(),
                                custom_size: Some(Vec2::splat(tile_size - padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 1.),
                            texture: board_assets.red_pawn_material.texture.clone(),
                            ..Default::default()
                        });
                    });
                }
            };
        }; // if let ends
    }

    fn spawn_text(
        parent: &mut ChildBuilder,
        text: String,
        board_assets: &BoardAssets,
        tile_size: f32,
        position: Vec2,
        color: Color,
    ) {
        parent.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color: color,
                        font: board_assets.font.clone(),
                        font_size: tile_size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(position.x, position.y, 1.),
            ..Default::default()
        });
    }

    fn spawn_deck_card_board(
        parent: &mut ChildBuilder,
        board_size: Vec2,
        card: &Card,
        board_assets: &BoardAssets,
        padding: f32,
        tile_size: f32,
    ) {
        // spawn background for the card
        parent
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(board_size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            })
            .insert(Name::new(format!("Background {}", card.name)))
            .insert(Background);

        // spawn card name below the board
        parent
            .spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: card.name.to_owned(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font: board_assets.font.clone(),
                            font_size: tile_size - padding,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                transform: Transform::from_xyz(0.0, -board_size.y / 1.5, 1.),
                ..Default::default()
            })
            .insert(Name::new(format!("Card title: {}", card.name)));

        // Calculate the coordinates for the possible moves
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

        // spawn tiles for the card board
        for x in 0..5u8 {
            for y in 0..5u8 {
                let coordinates = Coordinates { x, y };

                let mut tile_color;
                // highlight the center
                if (x, y) == (2, 2) {
                    tile_color = board_assets.deck_card_center_material.color;
                } else {
                    tile_color = board_assets.tile_material.color;
                }

                // highlight possible moves
                if move_tiles.contains(&coordinates) {
                    tile_color = board_assets.deck_card_allowed_move_material.color;
                }

                let mut cmd = parent.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: tile_color,
                        custom_size: Some(Vec2::splat(tile_size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.) - (board_size.x / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.) - (board_size.y / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!(
                    "Tile in card {} ({}, {})",
                    card.name, x, y
                )))
                // We add the `Coordinates` component to our tile entity
                .insert(coordinates);
            }
        } // end of for loop
    }

    pub fn start_game(mut turn_process_ewr: EventWriter<TurnProcessEvent>) {
        turn_process_ewr.send(TurnProcessEvent);
    }

    pub fn cleanup_game(
        mut commands: Commands,
        board: Res<Board>,
        deck: Res<Deck>,
        text_handler: Res<TextHandler>,
    ) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();

        for (entity, _) in deck.cardboards.iter() {
            commands.entity(*entity).despawn_recursive();
        }
        commands.remove_resource::<Deck>();

        commands.entity(text_handler.turn_text).despawn_recursive();
        commands.entity(text_handler.guide_text).despawn_recursive();
        commands
            .entity(text_handler.evaluation_text)
            .despawn_recursive();
        commands.remove_resource::<TextHandler>();

        commands.remove_resource::<SelectedPiece>();
        commands.remove_resource::<SelectedCard>();

        commands.remove_resource::<GameState>();
        commands.remove_resource::<EvaluationResult>();
    }
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone())
                .with_system(Self::create_board)
                .with_system(Self::start_game),
        );
        app.add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(
                    systems::game_state_process::process_win_condition
                        .label("process_win_condition"),
                )
                .with_system(
                    systems::text_change::process_guide_text
                        .label("process_guide_text")
                        .after("process_win_condition"),
                )
                .with_system(
                    systems::game_state_process::next_turn_event
                        .label("next_turn_event")
                        .after("process_win_condition")
                        .after("process_guide_text"),
                )
                .with_system(
                    systems::game_state_process::turn_process.before("process_win_condition"),
                )
                .with_system(
                    systems::ai_input::generate_bot_move
                        .label("bot_generate_move")
                        .after("next_turn_event"),
                )
                .with_system(systems::text_change::change_evaluation_text)
                .with_system(
                    systems::ai_input::bot_make_move::<T>
                        .label("bot_make_move")
                        .after("bot_generate_move"),
                )
                .with_system(
                    systems::board_input::input_handling
                        .label("input_handling")
                        .after("next_turn_event"),
                )
                .with_system(systems::card_input::card_selection_handling.after("input_handling"))
                .with_system(
                    systems::board_input::process_selected_tile.label("color_selected_tile"),
                )
                .with_system(systems::text_change::change_turn_text)
                .with_system(systems::card_input::color_selected_card.label("color_selected_card"))
                .with_system(
                    systems::card_input::reset_selected_card_color
                        .label("reset_selected_card_color")
                        .after("color_selected_card")
                        .after("color_selected_tile"),
                )
                .with_system(systems::text_change::process_guide_text_change_timer)
                .with_system(systems::board_input::color_selected_piece) // .with_system(systems::card_input::blink_non_selected_card),
                .with_system(systems::board_input::reset_selected_piece_color)
                .with_system(
                    systems::board_input::reset_allowed_moves.before("generate_allowed_moves"),
                )
                .with_system(
                    systems::board_input::generate_allowed_moves.label("generate_allowed_moves"),
                )
                .with_system(systems::board_input::move_piece::<T>)
                .with_system(systems::card_input::card_swap)
                .with_system(systems::card_input::mirror_card),
        );
        app.add_system_set(
            SystemSet::on_exit(self.cleanup_state.clone()).with_system(Self::cleanup_game),
        );
        app.add_event::<PieceSelectEvent>();
        app.add_event::<ColorSelectedCardEvent>();
        app.add_event::<ResetSelectedCardColorEvent>();
        app.add_event::<ChangeGuideTextEvent>();
        app.add_event::<NoCardSelectedEvent>();
        app.add_event::<ColorSelectedPieceEvent>();
        app.add_event::<ResetSelectedPieceColorEvent>();
        app.add_event::<GenerateAllowedMovesEvent>();
        app.add_event::<ResetAllowedMovesEvent>();
        app.add_event::<TurnProcessEvent>();
        app.add_event::<NextTurnEvent>();
        app.add_event::<GenerateBotMoveEvent>();
        app.add_event::<MovePieceEvent>();
        app.add_event::<CardSwapEvent>();
        app.add_event::<MirrorCardEvent>();
        app.add_event::<ProcessWinConditionEvent>();
        app.add_event::<BotMakeMoveEvent>();

        log::info!("Loaded Board Plugin");

        #[cfg(feature = "debug")]
        {
            let mut registry: Mut<InspectableRegistry> = app
                .world
                .get_resource_or_insert_with(InspectableRegistry::default);
            // registering custom component to be able to edit it in inspector
            registry.register::<Coordinates>();
            registry.register::<Piece>();
            registry.register::<PlayerColor>();
            registry.register::<PieceKind>();
            registry.register::<CardIndex>();
            registry.register::<CardBoard>();
            registry.register::<CardOwner>();
            registry.register::<Entity>();
            registry.register::<ListElement>();
        }
    }
}
