use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::{AppState, GAME_TITLE};

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
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = primary_window_query.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;

    egui::CentralPanel::default().show(egui.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.add(egui::Label::new(egui::RichText::new(GAME_TITLE).strong().size(32.0)).wrap());
            ui.horizontal(|ui| {
                ui.label("A game by");
                if ui.link("Silas").clicked() {
                    let _ = webbrowser::open("https://github.com/exvacuum");
                }
                ui.label("and");
                if ui.link("Carter").clicked() {
                    let _ = webbrowser::open("https://github.com/hyperliskdev");
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    next_state.set(AppState::InGame);
                }
                if ui.button("Quit").clicked() {
                    exit_event_writer.send_default();
                }
            });
        });
    });
}
