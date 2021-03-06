use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::button_plugin::ButtonPlugin;
use board_plugin::menu_plugin::MainMenuPlugin;
use board_plugin::resources::app_state::AppState;
use board_plugin::resources::board_assets::{BoardAssets, SpriteMaterial};
use board_plugin::resources::board_options::{BoardOptions, TileSize};
use board_plugin::resources::deck_options::DeckOptions;
use board_plugin::resources::depth::Depth;
use board_plugin::resources::physical_deck::PhysicalDeck;
use board_plugin::resources::selected::SelectedPlayers;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1320.,
        height: 940.,
        title: "Onitama in Rust".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app.insert_resource(PhysicalDeck::new());
    app.insert_resource(SelectedPlayers::default());
    app.insert_resource(Depth::default());

    app.add_state(AppState::MainMenu);
    app.add_plugin(MainMenuPlugin);

    app.add_plugin(BoardPlugin {
        running_state: AppState::InProgress,
        cleanup_state: AppState::GameEnd,
    })
    .add_startup_system(setup_board);

    app.add_plugin(ButtonPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        temple_tile_material: SpriteMaterial {
            color: Color::rgb(0.35, 0.45, 0.35),
            ..Default::default()
        },
        blue_pawn_material: SpriteMaterial {
            color: Color::BLUE,
            ..Default::default()
        },
        blue_king_material: SpriteMaterial {
            texture: asset_server.load("sprites/star_blue.png"),
            ..Default::default()
        },
        red_pawn_material: SpriteMaterial {
            color: Color::RED,
            ..Default::default()
        },
        red_king_material: SpriteMaterial {
            texture: asset_server.load("sprites/star_red.png"),
            ..Default::default()
        },
        deck_card_center_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        deck_card_allowed_move_material: SpriteMaterial {
            color: Color::OLIVE,
            ..Default::default()
        },
        selected_blue_card_material: SpriteMaterial {
            color: Color::CYAN,
            ..Default::default()
        },
        selected_red_card_material: SpriteMaterial {
            color: Color::RED,
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
    // state.set(AppState::InProgress).unwrap();
}
