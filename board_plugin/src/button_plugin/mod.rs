use bevy::{log, prelude::*};

use crate::resources::app_state::AppState;

struct ButtonPluginData {
    pub camera_entity: Entity,
    pub ui_root: Entity,
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InProgress).with_system(setup_ui))
            .add_system_set(SystemSet::on_update(AppState::InProgress).with_system(input_handler))
            .add_system_set(SystemSet::on_update(AppState::GameEnd).with_system(input_handler))
            .add_system_set(SystemSet::on_exit(AppState::GameEnd).with_system(cleanup));
        // .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(cleanup));
    }
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
                            log::info!("Creating a new setup during the game");
                            state.set(AppState::GameEnd).unwrap();
                        }

                        if state.current() == &AppState::GameEnd {
                            log::info!("Going to the main menu!");
                            state.set(AppState::MainMenu).unwrap();
                        }
                    }
                    ButtonAction::NewGame => {
                        if state.current() == &AppState::InProgress {
                            log::info!("Creating a new game during the game");
                            state.set(AppState::GameEnd).unwrap();
                        }

                        if state.current() == &AppState::GameEnd {
                            log::info!("Creating a new game after a game end!");
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
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let button_materials = ButtonColors {
        normal: Color::GRAY,
        hovered: Color::ANTIQUE_WHITE,
        pressed: Color::BLACK,
    };

    let root = commands
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
        })
        .id();

    commands.insert_resource(button_materials);

    commands.insert_resource(ButtonPluginData {
        camera_entity,
        ui_root: root,
    });
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

fn cleanup(mut commands: Commands, menu_data: Res<ButtonPluginData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
    commands.remove_resource::<ButtonPluginData>();
}
