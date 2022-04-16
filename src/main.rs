use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::ai::alpha_beta::AlphaBetaAgent;
use board_plugin::ai::human::Human;
use board_plugin::ai::random_agent::RandomAgent;
use board_plugin::resources::app_state::AppState;
use board_plugin::resources::board_assets::{BoardAssets, SpriteMaterial};
use board_plugin::resources::board_options::{BoardOptions, TileSize};
use board_plugin::resources::deck_options::DeckOptions;
use board_plugin::resources::game_state::{GameState, Player, PlayerType};
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: "Onitama in Rust".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.add_state(AppState::Out);
    app.add_plugin(BoardPlugin {
        running_state: AppState::InProgress,
        cleanup_state: AppState::GameEnd
    })
    .add_startup_system(setup_board)
    .add_startup_system(setup_ui)
    .add_system(input_handler);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        tile_padding: 3.0,
        position: Vec3::new(-550.0, -230.0, 0.),
        ..Default::default()
    });

    commands.insert_resource(DeckOptions {
        tile_padding: 3.0,
        position: Vec3::new(350.0, 30., 0.),
        tile_size: TileSize::Adaptive {
            min: 10.0,
            max: 30.0,
        },
    });

    let first_player = Player {
        agent: Box::new(Human),
        player_type: PlayerType::Human,
    };

    let second_player = Player {
        agent: Box::new(AlphaBetaAgent { max_depth: 5 }),
        player_type: PlayerType::AlphaBeta,
    };

    commands.insert_resource(GameState::new(first_player, second_player));

    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        blue_pawn_material: SpriteMaterial {
            color: Color::BLUE,
            ..Default::default()
        },
        blue_king_material: SpriteMaterial {
            texture: asset_server.load("sprites/star.png"),
            color: Color::BLUE,
        },
        red_pawn_material: SpriteMaterial {
            color: Color::RED,
            ..Default::default()
        },
        red_king_material: SpriteMaterial {
            texture: asset_server.load("sprites/star.png"),
            color: Color::RED,
        },
        deck_card_center_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        deck_card_allowed_move_material: SpriteMaterial {
            color: Color::OLIVE,
            ..Default::default()
        },
        selected_card_material: SpriteMaterial {
            color: Color::rgb_u8(255, 154, 154),
            ..Default::default()
        },
        selected_piece_material: SpriteMaterial {
            color: Color::AZURE,
            ..Default::default()
        },
        allowed_move_tile_material: SpriteMaterial {
            color: Color::CYAN,
            ..Default::default()
        },
        guide_text_size: 80.,
        turn_text_size: 40.,
        font: asset_server.load("fonts/pixeled.ttf"),
    });
    // Plugin activation
    state.set(AppState::InProgress).unwrap();
}

// Almost everything about UI setup was taken from here: https://gitlab.com/qonfucius/minesweeper-tutorial/-/blob/master/src/main.rs
/// Button action type
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub enum ButtonAction {
    NewSetup,
    NewGame,
}

#[derive(Debug)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn input_handler(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = button_colors.pressed.into();
                match action {
                    ButtonAction::NewSetup => {
                        if state.current() == &AppState::InProgress {
                            println!("cleared!");
                            state.set(AppState::Out).unwrap();
                        }
                    }
                    ButtonAction::NewGame => {
                        if state.current() == &AppState::GameEnd {
                            state.set(AppState::InProgress).unwrap();
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_materials = ButtonColors {
        normal: Color::GRAY,
        hovered: Color::DARK_GRAY,
        pressed: Color::BLACK,
    };
    commands
        // root
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(50.)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new("UI"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");
            setup_single_menu(
                parent,
                "NEW GAME",
                button_materials.normal.into(),
                font.clone(),
                ButtonAction::NewGame,
            );
            setup_single_menu(
                parent,
                "NEW SETUP",
                button_materials.normal.into(),
                font,
                ButtonAction::NewSetup,
            );
        });
    commands.insert_resource(button_materials);
}

fn setup_single_menu(
    parent: &mut ChildBuilder,
    text: &str,
    color: UiColor,
    font: Handle<Font>,
    action: ButtonAction,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.), Val::Auto),
                margin: Rect::all(Val::Px(10.)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color,
            ..Default::default()
        })
        .insert(action)
        .insert(Name::new(text.to_string()))
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text.to_string(),
                        style: TextStyle {
                            font,
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                ..Default::default()
            });
        });
}
