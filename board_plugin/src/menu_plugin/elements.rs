use bevy::prelude::*;

use crate::resources::board_assets::BoardAssets;

pub fn button(materials: &Res<BoardAssets>, position: Vec3) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        transform: Transform::from_translation(position),
        ..Default::default()
    }
}

pub fn button_text(materials: &Res<BoardAssets>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: materials.font.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    };
}