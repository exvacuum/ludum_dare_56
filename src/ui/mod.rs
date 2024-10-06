use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod main_menu;
use main_menu::*;
mod pause_menu;
use pause_menu::*;
pub mod dialog_box;
use dialog_box::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            DialogBoxPlugin,
        ));
    }
}
