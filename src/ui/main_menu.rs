use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Align, Align2, Color32, Direction, Layout, Pos2},
    EguiContexts,
};

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

fn show_main_menu(
    mut egui: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_event_writer: EventWriter<AppExit>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = primary_window_query.single();
    let window_size = primary_window.size();
    egui::Window::new("Epic Game")
        .pivot(Align2::CENTER_CENTER)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .fixed_pos(Pos2::new(window_size.x / 2.0, window_size.y / 2.0))
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Play").clicked() {
                    next_state.set(AppState::InGame);
                }
                if ui.button("Quit").clicked() {
                    exit_event_writer.send_default();
                }
            });
        });
}
