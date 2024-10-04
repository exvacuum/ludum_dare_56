use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, Actionlike,
    InputManagerBundle,
};

use crate::{AppState, PausedState};

#[derive(Actionlike, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct PauseMenuAction;

#[derive(Component, Debug)]
pub struct PauseMenu;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseMenuAction>::default())
            .add_systems(Startup, setup_pause_menu_input)
            .add_systems(
                Update,
                (
                    process_pause_input.run_if(in_state(AppState::InGame)),
                    show_pause_menu.run_if(in_state(PausedState::Paused)),
                ),
            );
    }
}

fn setup_pause_menu_input(mut commands: Commands) {
    commands.spawn((
        PauseMenu,
        InputManagerBundle::with_map(InputMap::new([(PauseMenuAction, KeyCode::Escape)])),
    ));
}

fn process_pause_input(
    pause_menu_query: Query<&ActionState<PauseMenuAction>, With<PauseMenu>>,
    state: Res<State<PausedState>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    let pause_action_state = pause_menu_query.single();
    if pause_action_state.just_pressed(&PauseMenuAction) {
        match state.get() {
            PausedState::Running => next_state.set(PausedState::Paused),
            PausedState::Paused => next_state.set(PausedState::Running),
        }
    }
}

fn show_pause_menu(
    mut egui: EguiContexts,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
) {
    egui::Window::new("Paused").show(egui.ctx_mut(), |ui| {
        if ui.button("Resume").clicked() {
            next_paused_state.set(PausedState::Running);
        }
        if ui.button("Quit").clicked() {
            next_app_state.set(AppState::MainMenu);
            next_paused_state.set(PausedState::Running);
        }
    });
}
