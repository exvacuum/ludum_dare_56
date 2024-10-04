use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::AppState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSet;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_main_menu.in_set(MainMenuSet))
            .configure_sets(Update, MainMenuSet.run_if(in_state(AppState::MainMenu)));
    }
}

fn show_main_menu(mut egui: EguiContexts, mut next_state: ResMut<NextState<AppState>>) {
    egui::Window::new("Title").show(egui.ctx_mut(), |ui| {
        if ui.button("Play").clicked() {
            next_state.set(AppState::InGame);
        }
    });
}
