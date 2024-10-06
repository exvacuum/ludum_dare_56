use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::PlayerState;

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            show_dialog_box.run_if(in_state(PlayerState::Dialog)),
        )
        .init_resource::<DialogBoxContent>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct DialogBoxContent {
    pub character: Option<String>,
    pub line: String,
}

fn show_dialog_box(dialog_box_content: Res<DialogBoxContent>, mut egui: EguiContexts) {
    egui::TopBottomPanel::bottom("Dialog Box")
        .resizable(false)
        .show(egui.ctx_mut(), |ui| {
            if let Some(character) = &dialog_box_content.character {
                ui.heading(character);
            }
            ui.label(&dialog_box_content.line);
        });
}
