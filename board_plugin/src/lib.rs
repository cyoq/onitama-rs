pub mod components;
pub mod resources;
pub mod systems;

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use components::coordinates::Coordinates;
use components::pieces::{Piece, PieceColor, PieceKind};
use resources::board::Board;
use resources::board_assets::BoardAssets;
use resources::board_options::BoardOptions;
use resources::card::Card;
use resources::deck_options::DeckOptions;

use crate::resources::board_options::TileSize;
use crate::resources::deck::Deck;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;

use PieceColor::*;
use PieceKind::*;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        deck_options: Option<Res<DeckOptions>>,
        window: Res<WindowDescriptor>,
        board_assets: Res<BoardAssets>,
    ) {
        let options = match board_options {
            Some(opt) => opt.clone(),
            None => Default::default(),
        };

        let deck_options = match deck_options {
            Some(opt) => opt.clone(),
            None => Default::default(),
        };

        let board = Board::new();
        let deck = Deck::new();
        #[cfg(feature = "debug")]
        log::info!("{}", board.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(&window, (min, max), (board.width(), board.height()))
            }
        };

        let board_size = Vec2::new(
            board.width() as f32 * tile_size,
            board.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        commands
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
                    &board,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                );
            });

        // Spawn boards with cards movements

        let tile_size = match deck_options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(&window, (min, max), (board.width(), board.height()))
            }
        };

        log::info!("deck card tile size: {}", tile_size);

        let board_size = Vec2::new(
            board.width() as f32 * tile_size,
            board.height() as f32 * tile_size,
        );

        log::info!("one board size from the deck: {}", board_size);

        commands
            .spawn()
            .insert(Name::new("Deck"))
            .insert(Transform::from_translation(deck_options.position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                let offset = board_size / 4.;
                let deck_pos = Vec2::new(board_size.x - offset.x, board_size.y + offset.y);
                let positions = [
                    Vec2::new(0., 0.),
                    Vec2::new(-deck_pos.x, -deck_pos.y),
                    Vec2::new(-deck_pos.x, deck_pos.y),
                    Vec2::new(deck_pos.x, deck_pos.y),
                    Vec2::new(deck_pos.x, -deck_pos.y),
                ];
                for i in 0..5 {
                    Self::spawn_deck_board(
                        parent,
                        board_size,
                        positions[i],
                        &deck.cards[i],
                        &board_assets,
                        deck_options.tile_padding,
                        tile_size,
                    );
                }
            });
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
        board: &Board,
        tile_size: f32,
        padding: f32,
        board_assets: &BoardAssets,
    ) {
        for (y, line) in board.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u8,
                    y: y as u8,
                };

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
                .insert(coordinates);

                // Creating a pawn or a king square
                if let Some(piece) = tile.piece {
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
        }
    }

    fn spawn_deck_board(
        parent: &mut ChildBuilder,
        board_size: Vec2,
        position: Vec2,
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
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..Default::default()
            })
            .insert(Name::new(format!("Background {}", card.name)));
        // .insert_bundle(Text2dBundle {
        //     text: Text {
        //         sections: vec![TextSection {
        //             value: card.name.to_owned(),
        //             style: TextStyle {
        //                 color: Color::WHITE,
        //                 font: board_assets.font.clone(),
        //                 font_size: tile_size - padding,
        //             },
        //         }],
        //         alignment: TextAlignment {
        //             vertical: VerticalAlign::Bottom,
        //             horizontal: HorizontalAlign::Center,
        //         },
        //     },
        //     transform: Transform::from_xyz(
        //         board_size.x / 2. + position.x,
        //         board_size.y / 2. + position.y,
        //         1.,
        //     ),
        //     ..Default::default()
        // });

        let center = Coordinates { x: 2, y: 2 };
        let move_tiles = card
            .directions
            .iter()
            .map(|tuple| center + *tuple)
            .collect::<Vec<_>>();

        // spawn tiles for the card
        for x in 0..5u8 {
            for y in 0..5u8 {
                let coordinates = Coordinates { x, y };

                let mut tile_color;
                if (x, y) == (2, 2) {
                    tile_color = board_assets.deck_card_center_material.color;
                } else {
                    tile_color = board_assets.tile_material.color;
                }

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
                        (x as f32 * tile_size) + (tile_size / 2.)
                            - (board_size.x / 2. + position.x),
                        (y as f32 * tile_size) + (tile_size / 2.)
                            - (board_size.y / 2. + position.y),
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
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_board),
        );
        log::info!("Loaded Board Plugin");

        #[cfg(feature = "debug")]
        {
            let mut registry: Mut<InspectableRegistry> = app
                .world
                .get_resource_or_insert_with(InspectableRegistry::default);
            // registering custom component to be able to edit it in inspector
            registry.register::<Coordinates>();
            registry.register::<Piece>();
            registry.register::<PieceColor>();
            registry.register::<PieceKind>();
        }
    }
}
