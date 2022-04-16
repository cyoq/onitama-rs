use bevy::{log, prelude::*};

use crate::{
    components::card_index::CardIndex,
    resources::{
        app_state::AppState,
        board_assets::BoardAssets,
        board_options::{BoardOptions, TileSize},
        card::CARDS,
        deck_options::DeckOptions,
        game_state::{PlayerColor, PlayerType},
    },
    BoardPlugin,
};

struct MainMenuData {
    camera_entity: Entity,
    title_root: Entity,
    description_root: Entity,
    player_list_root: Entity,
    cards: Vec<Entity>,
    button_root: Entity,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Loaded main menu plugin!");
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(setup_ui::<AppState>),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(button_system)
                .with_system(list_system)
                .with_system(button_press_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

pub struct MenuMaterials {
    pub root: Color,
    pub border: Color,
    pub menu: Color,
    pub button_normal: Color,
    pub button_hovered: Color,
    pub button_pressed: Color,
    pub button_text: Color,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
enum ButtonAction {
    NewGame,
    ClearSelectedCards,
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.into(),
            Interaction::Hovered => *material = materials.button_hovered.into(),
            Interaction::None => *material = materials.button_normal.into(),
        }
    }
}

fn list_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ListElement>)>,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.into(),
            Interaction::Hovered => *material = materials.button_hovered.into(),
            Interaction::None => *material = materials.button_normal.into(),
        }
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                ButtonAction::NewGame => log::info!("New Game"),
                ButtonAction::ClearSelectedCards => log::info!("Clear selected"),
                // MenuButton::Play => state
                //     .set(AppState::InProgress)
                //     .expect("Couldn't switch state to InGame"),
                // MenuButton::Quit => (),
            };
        }
    }
}

fn setup_ui<T>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    deck_options: Res<DeckOptions>,
    window: Res<WindowDescriptor>,
    board_assets: Res<BoardAssets>,
    board_options: Res<BoardOptions>,
) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let button_materials = MenuMaterials {
        root: Color::rgb(0.65, 0.65, 0.65),
        border: Color::rgb(0.15, 0.15, 0.15),
        menu: Color::rgb(0.15, 0.15, 0.15),
        button_normal: Color::rgb(0.25, 0.25, 0.25),
        button_hovered: Color::rgb(0.35, 0.35, 0.35),
        button_pressed: Color::rgb(0.35, 0.75, 0.35),
        button_text: Color::WHITE,
    };

    let mut tile_size = match deck_options.tile_size {
        TileSize::Fixed(size) => size,
        TileSize::Adaptive { min, max } => {
            // (5, 5) - board size
            BoardPlugin::<T>::adaptive_tile_size(&window, (min, max), (5, 5))
        }
    };

    tile_size /= 1.3;

    let board_size = Vec2::new(5. * tile_size, 5. * tile_size);

    let title_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(8.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new("Title"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");

            // spawning a title
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Percent(100.), Val::Percent(8.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Onitama".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 60.,
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
        })
        .id();

    let description_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(8.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(window.height * 0.08),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new("Description text"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Welcome to the game of Onitama! This game is created as a university project. The author is cyoq. ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
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
            }).id();

    let player_list_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(window.height * 0.16),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new("Player list"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");

            setup_single_list(
                parent,
                PlayerColor::Red,
                Color::RED,
                "Red Player: ",
                font.clone(),
            );
            setup_single_list(
                parent,
                PlayerColor::Blue,
                Color::BLUE,
                "Blue Player: ",
                font.clone(),
            );
        })
        .id();

    // generating the cards
    let mut cards: Vec<Entity> = Vec::with_capacity(CARDS.len());
    let offset = window.width / 18.;

    log::info!("board size: {:?}", board_size);

    // moving pivot from the center to the left bottom part of the screen from screen center
    let pivot = Vec2::new(-window.width / 2., -window.height / 2.);
    // 26% are taken by the UI
    let starting_y = pivot.y + window.height * 0.64 - board_size.y + offset;
    let starting_x = pivot.x + 1.5 * offset;

    let cards_in_row = 7;

    for (i, card) in CARDS.iter().enumerate() {
        let position = Vec2::new(
            starting_x + (offset + board_size.x) * (i % cards_in_row) as f32,
            starting_y - (offset + board_size.y) * (i / cards_in_row) as f32,
        );

        let card_entity = commands
            .spawn()
            .insert(Transform::from_translation(Vec3::new(
                position.x, position.y, 0.,
            )))
            .insert(GlobalTransform::default())
            .insert(CardIndex(i as u8))
            .insert(Name::new(card.name))
            .with_children(|builder| {
                BoardPlugin::<T>::spawn_deck_card_board(
                    builder,
                    board_size,
                    card,
                    &board_assets,
                    board_options.tile_padding,
                    tile_size,
                );
            })
            .id();
        cards.push(card_entity);
    }

    let button_root = commands
        // root
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                flex_wrap: FlexWrap::WrapReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new("Buttons"))
        .with_children(|parent| {
            let font = asset_server.load("fonts/pixeled.ttf");
            setup_single_button(
                parent,
                "CLEAR SELECTED CARDS",
                button_materials.button_normal.into(),
                font.clone(),
                ButtonAction::ClearSelectedCards,
            );

            setup_single_button(
                parent,
                "NEW GAME",
                button_materials.button_normal.into(),
                font,
                ButtonAction::NewGame,
            );
        })
        .id();

    commands.insert_resource(button_materials);

    commands.insert_resource(MainMenuData {
        camera_entity,
        title_root,
        description_root,
        player_list_root,
        cards,
        button_root,
    });
}

#[derive(Debug, Clone, Component)]
struct ListElement {
    color: PlayerColor,
    typ: PlayerType,
}

fn setup_single_list(
    parent: &mut ChildBuilder,
    player_color: PlayerColor,
    text_color: Color,
    text: &str,
    font: Handle<Font>,
) {
    let players = [
        ("Human", PlayerType::Human),
        ("Random", PlayerType::Random),
        ("AlphaBeta", PlayerType::AlphaBeta),
    ];

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.), Val::Auto),
                margin: Rect::all(Val::Px(10.)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..Default::default()
        })
        .insert(Name::new(text.to_string()))
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text.to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.,
                            color: text_color,
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

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                flex_grow: 1.0,
                max_size: Size::new(Val::Undefined, Val::Undefined),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // List items
            for (text, player) in players {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            flex_shrink: 0.,
                            size: Size::new(Val::Undefined, Val::Px(20.)),
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            format!("{}", text),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(ListElement {
                        color: player_color,
                        typ: player,
                    });
            }
        });
}

fn setup_single_button(
    parent: &mut ChildBuilder,
    text: &str,
    color: UiColor,
    font: Handle<Font>,
    action: ButtonAction,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(30.), Val::Auto),
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

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.title_root).despawn_recursive();
    commands
        .entity(menu_data.description_root)
        .despawn_recursive();
    commands
        .entity(menu_data.player_list_root)
        .despawn_recursive();
    commands.entity(menu_data.button_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();

    for entity in menu_data.cards.iter() {
        commands.entity(*entity).despawn_recursive();
    }

    commands.remove_resource::<MainMenuData>();
}
