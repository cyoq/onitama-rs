pub mod components;
pub mod resources;
pub mod systems;

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use components::coordinates::Coordinates;
use components::pieces::{PieceColor::*, PieceKind::*};
use resources::board::Board;
use resources::board_assets::BoardAssets;
use resources::board_options::BoardOptions;

use crate::components::pieces::Piece;
use crate::resources::board_options::TileSize;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
        board_assets: Res<BoardAssets>,
    ) {
        let options = match board_options {
            Some(opt) => opt.clone(),
            None => Default::default(),
        };

        let board = Board::new();
        #[cfg(feature = "debug")]
        log::info!("{}", board.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (board.width(), board.height()))
            }
        };

        let board_size = Vec2::new(
            board.width() as f32 * tile_size,
            board.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);

        let board_position = options.position;

        commands
            .spawn()
            .insert(Name::new("GameBoard"))
            .insert(Transform::from_translation(board_position))
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

                Self::spawn_tiles(
                    parent,
                    &board,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                );
            });
    }

    pub fn adaptive_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u8, u8),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    fn spawn_tiles(
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
        }
    }
}
