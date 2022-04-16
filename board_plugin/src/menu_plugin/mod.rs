pub mod systems;
pub mod elements;
pub mod menu_assets;

use bevy::prelude::*;

use crate::resources::app_state::AppState;

#[derive(Debug)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     SystemSet::on_enter(AppState::MainMenu)
        //         .with_system(cleanup)
        //         .with_system(setup),
        // )
        // .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}
