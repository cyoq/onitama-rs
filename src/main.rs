use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::resources::board_assets::{BoardAssets, SpriteMaterial};
use board_plugin::resources::board_options::BoardOptions;
use board_plugin::BoardPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 700.,
        title: "Onitama in Rust".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    .add_state(AppState::Out)
    .add_startup_system(setup_board);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<AppState>>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        tile_padding: 3.0,
        position: Vec3::new(-300.0, -200.0, 0.),
        ..Default::default()
    });
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
            texture: asset_server.load("sprites/hat.png"),
            color: Color::BLUE,
        },
        red_pawn_material: SpriteMaterial {
            color: Color::RED,
            ..Default::default()
        },
        red_king_material: SpriteMaterial {
            texture: asset_server.load("sprites/hat.png"),
            color: Color::RED,
        },
        font: asset_server.load("fonts/pixeled.ttf"),
    });
    // Plugin activation
    state.set(AppState::InGame).unwrap();
}
