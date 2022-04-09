use bevy::{prelude::EventReader, log};

use crate::{events::ChangeGuideText, components::guide::GuideText, resources::board_assets::BoardAssets};

use bevy::prelude::*;

pub fn process_guide_text(
    mut change_guide_text_rdr: EventReader<ChangeGuideText>,
    // mut parent: Query<Entity, With<GuideText>>,
    // mut children: Query<&Children, With<Text>>,
    mut guide_text: Query<&mut Text>,
    board_assets: Res<BoardAssets>
) {
    for event in change_guide_text_rdr.iter() {
        match guide_text.get_single_mut() {
            Ok(mut text) => {
                text.sections = vec![TextSection {
                    value: event.0.clone(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font: board_assets.font.clone(),
                        font_size: board_assets.guide_text_size,
                    },
                }];
            },
            Err(e) => {
                log::error!("An error in changing guide text occured: {:?}", e);
                continue;
            }
        };
    }
}
