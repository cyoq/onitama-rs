use bevy::{log, prelude::EventReader};

use crate::{
    components::guide::GuideText, events::ChangeGuideText, resources::board_assets::BoardAssets,
};

use bevy::prelude::*;

pub fn process_guide_text(
    mut change_guide_text_rdr: EventReader<ChangeGuideText>,
    // query all children of the parents with GuideText component(mut be only one parent)
    parents: Query<&Children, With<GuideText>>,
    // get children of that parent
    mut children: Query<&mut Text>,
    board_assets: Res<BoardAssets>,
) {
    for event in change_guide_text_rdr.iter() {
        // iterate through the parents children
        for children_components in parents.iter() {
            // get entities of the children
            for child_entity in children_components.iter() {
                // Get the needed component
                match children.get_mut(*child_entity) {
                    Ok(mut text) => {
                        text.sections = vec![TextSection {
                            value: event.0.clone(),
                            style: TextStyle {
                                color: Color::WHITE,
                                font: board_assets.font.clone(),
                                font_size: board_assets.guide_text_size,
                            },
                        }];
                        log::info!("Changed a guide text to {}", event.0);
                    }
                    Err(e) => {
                        log::error!("An error in changing guide text occured: {:?}", e);
                        continue;
                    }
                }
            }
        }
    }
}
