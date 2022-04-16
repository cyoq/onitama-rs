use bevy::prelude::*;

use crate::resources::app_state::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system)
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct MenuMaterials {
    pub root: Color,
    pub border: Color,
    pub menu: Color,
    pub button: Color,
    pub button_hovered: Color,
    pub button_pressed: Color,
    pub button_text: Color,
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.into(),
            Interaction::Hovered => *material = materials.button_hovered.into(),
            Interaction::None => *material = materials.button.into(),
        }
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::InProgress)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::Quit => (),
            };
        }
    }
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.root.into(),
        ..Default::default()
    }
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: materials.border.into(),
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: materials.menu.into(),
        ..Default::default()
    }
}

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button.into(),
        ..Default::default()
    }
}

fn button_text(
    asset_server: &Res<AssetServer>,
    materials: &Res<MenuMaterials>,
    label: &str,
) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/pixeled.ttf"),
                font_size: 30.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: Res<MenuMaterials>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "New Game",
                                    ));
                                })
                                .insert(MenuButton::Play);
                            parent
                                .spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(
                                        &asset_server,
                                        &materials,
                                        "Quit",
                                    ));
                                })
                                .insert(MenuButton::Quit);
                        });
                });
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
    commands.remove_resource::<MainMenuData>();
}
