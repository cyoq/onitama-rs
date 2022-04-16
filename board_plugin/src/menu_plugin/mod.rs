use bevy::{log, prelude::*};

use crate::{
    bounds::Bounds2,
    components::{background::Background, card_index::CardIndex},
    resources::{
        app_state::AppState,
        board_assets::BoardAssets,
        board_options::{BoardOptions, TileSize},
        card::CARDS,
        deck_options::DeckOptions,
        game_state::{PlayerColor, PlayerType},
        physical_deck::PhysicalDeck,
        selected::SelectedPlayers,
    },
    BoardPlugin,
};

struct MainMenuData {
    camera_entity: Entity,
    title_root: Entity,
    description_root: Entity,
    player_list_root: Entity,
    // bounds are needed to color the background
    cards: Vec<(Entity, Bounds2)>,
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
                .with_system(color_selected_cards)
                .with_system(reset_selected_cards)
                .with_system(button_press_system)
                .with_system(list_press_system)
                .with_system(update_button_color),
        )
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));

        app.add_event::<ResetSelectedCardsEvent>();
        app.add_event::<UpdateButtonColorEvent>();
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

#[derive(Debug, Clone)]
pub struct SelectedCards(pub Vec<(Entity, u8)>);

impl Default for SelectedCards {
    fn default() -> Self {
        Self(vec![])
    }
}

struct CardColors(pub Vec<Color>);

impl Default for CardColors {
    fn default() -> Self {
        Self(vec![
            Color::RED,
            Color::RED,
            Color::LIME_GREEN,
            Color::CYAN,
            Color::CYAN,
        ])
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Component)]
pub struct ListElement {
    pub color: PlayerColor,
    pub typ: PlayerType,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
enum ButtonAction {
    StartGame,
    ClearSelectedCards,
}

#[derive(Component)]
struct SimpleButton;

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<SimpleButton>),
    >,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.into(),
            Interaction::Hovered => *material = materials.button_hovered.into(),
            Interaction::None => *material = materials.button_normal.into(),
        }
    }
}

fn color_selected_cards(
    windows: Res<Windows>,
    menu_data: Res<MainMenuData>,
    mut colors: ResMut<CardColors>,
    mut selected_cards: ResMut<SelectedCards>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    cards_q: Query<(&CardIndex, &Children)>,
    mut sprites: Query<&mut Sprite, With<Background>>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if colors.0.is_empty() {
        log::info!("All cards are colored!");
        return;
    }

    let window = windows.get_primary().unwrap();
    let position = window.cursor_position();

    for (entity, bounds) in menu_data.cards.iter() {
        if let Some(pos) = position {
            if !bounds.in_bounds_window(&window, pos) {
                continue;
            }

            if let Ok((card_index, children)) = cards_q.get(*entity) {
                'child: for child in children.iter() {
                    match sprites.get_mut(*child) {
                        Ok(mut sprite) => {
                            let color = colors.0.remove(0);
                            sprite.color = color;
                            log::info!(
                                "Selectd a card with index {} and given the color {:?}",
                                card_index.0,
                                color
                            );

                            selected_cards.0.push((*entity, card_index.0));
                            break 'child;
                        }
                        Err(_) => log::warn!("Sprite for background was not found"),
                    }
                }
            }
        }
    }
}

struct ResetSelectedCardsEvent;

fn reset_selected_cards(
    mut colors: ResMut<CardColors>,
    mut selected_cards: ResMut<SelectedCards>,
    cards_q: Query<&Children, With<CardIndex>>,
    mut sprites: Query<&mut Sprite, With<Background>>,
    mut reset_selected_cards_rdr: EventReader<ResetSelectedCardsEvent>,
) {
    for _ in reset_selected_cards_rdr.iter() {
        *colors = CardColors::default();
        for (entity, _) in selected_cards.0.iter() {
            if let Ok(children) = cards_q.get(*entity) {
                'child: for child in children.iter() {
                    match sprites.get_mut(*child) {
                        Ok(mut sprite) => {
                            sprite.color = Color::WHITE;
                            break 'child;
                        }
                        Err(_) => log::warn!("Error while resetting card background!"),
                    }
                }
            }
        }
        selected_cards.0.clear();
        log::info!("Card colors after reset: {:?}", colors.0);
        log::info!("Card colors after reset: {:?}", selected_cards);
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut physical_deck: ResMut<PhysicalDeck>,
    selected_cards: Res<SelectedCards>,
    mut state: ResMut<State<AppState>>,
    mut reset_selected_cards_ewr: EventWriter<ResetSelectedCardsEvent>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                ButtonAction::StartGame => {
                    log::info!("New Game");
                    if selected_cards.0.len() == 5 {
                        let res = selected_cards.0.iter().map(|v| v.1).collect::<Vec<_>>();
                        physical_deck.take_cards_from_indices(&res);
                    } else if selected_cards.0.len() == 0 {
                        physical_deck.take_random_cards();
                    } else {
                        let res = selected_cards.0.iter().map(|v| v.1).collect::<Vec<_>>();
                        physical_deck.take_some_random_cards(&res);
                    }
                    state.set(AppState::InProgress).unwrap();
                },
                ButtonAction::ClearSelectedCards => {
                    log::info!("Clear selected");
                    reset_selected_cards_ewr.send(ResetSelectedCardsEvent);
                }
            };
        }
    }
}

struct UpdateButtonColorEvent;

fn update_button_color(
    selected_players: Res<SelectedPlayers>,
    materials: Res<MenuMaterials>,
    mut buttons: Query<(&ListElement, &mut UiColor), With<Button>>,
    mut update_button_color_rdr: EventReader<UpdateButtonColorEvent>,
) {
    for _ in update_button_color_rdr.iter() {
        for (list_element, mut material) in buttons.iter_mut() {
            match list_element.color {
                PlayerColor::Red => {
                    if selected_players.red_player == list_element.typ {
                        *material = Color::RED.into();
                    } else {
                        *material = materials.button_normal.into()
                    }
                }
                PlayerColor::Blue => {
                    if selected_players.blue_player == list_element.typ {
                        *material = Color::BLUE.into();
                    } else {
                        *material = materials.button_normal.into()
                    }
                }
            }
        }
    }
}

fn list_press_system(
    mut selected_players: ResMut<SelectedPlayers>,
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &ListElement, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut update_button_color_ewr: EventWriter<UpdateButtonColorEvent>,
) {
    for (interaction, list_element, mut material) in buttons.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                match list_element.color {
                    PlayerColor::Red => {
                        *material = Color::RED.into();
                        selected_players.red_player = list_element.typ;
                    }
                    PlayerColor::Blue => {
                        *material = Color::BLUE.into();
                        selected_players.blue_player = list_element.typ;
                    }
                }
                update_button_color_ewr.send(UpdateButtonColorEvent);
                log::info!("Changed selected players: {:?}", selected_players);
            }
            Interaction::Hovered => {
                *material = materials.button_hovered.into();
            }
            Interaction::None => match list_element.color {
                PlayerColor::Red => {
                    if selected_players.red_player == list_element.typ {
                        *material = Color::RED.into();
                    } else {
                        *material = materials.button_normal.into()
                    }
                }
                PlayerColor::Blue => {
                    if selected_players.blue_player == list_element.typ {
                        *material = Color::BLUE.into();
                    } else {
                        *material = materials.button_normal.into()
                    }
                }
            },
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
    mut physical_deck: ResMut<PhysicalDeck>,
) {
    physical_deck.clear();
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
    let mut cards: Vec<(Entity, Bounds2)> = Vec::with_capacity(CARDS.len());
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
        let bounds = Bounds2 {
            position: position - board_size / 2.,
            size: board_size,
        };
        cards.push((card_entity, bounds));
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
                "START A GAME",
                button_materials.button_normal.into(),
                font,
                ButtonAction::StartGame,
            );
        })
        .id();

    commands.insert_resource(button_materials);

    commands.insert_resource(SelectedCards::default());
    commands.insert_resource(CardColors::default());

    commands.insert_resource(MainMenuData {
        camera_entity,
        title_root,
        description_root,
        player_list_root,
        cards,
        button_root,
    });
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
                let color = if player == PlayerType::Human {
                    if player_color == PlayerColor::Red {
                        Color::RED
                    } else {
                        Color::BLUE
                    }
                } else {
                    Color::NONE
                };
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            flex_shrink: 0.,
                            size: Size::new(Val::Percent(50.), Val::Px(20.)),
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        color: color.into(),
                        ..Default::default()
                    })
                    .insert(ListElement {
                        color: player_color,
                        typ: player,
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
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
                        });
                    });
            } // for cycle
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
        .insert(SimpleButton)
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

    for (entity, _) in menu_data.cards.iter() {
        commands.entity(*entity).despawn_recursive();
    }

    commands.remove_resource::<MainMenuData>();
    commands.remove_resource::<SelectedCards>();
    commands.remove_resource::<CardColors>();
}
