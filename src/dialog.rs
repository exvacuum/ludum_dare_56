use bevy::prelude::*;
use bevy_yarnspinner::{
    deferred_loading::LoadYarnProjectEvent,
    events::{DialogueCompleteEvent, DialogueStartEvent, ExecuteCommandEvent, PresentLineEvent, PresentOptionsEvent},
    prelude::{DialogueRunner, YarnFileSource, YarnSpinnerPlugin},
};

use crate::{dialog_box::DialogBoxContent, GameplaySet, PlayerState};

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YarnSpinnerPlugin::deferred())
            .add_systems(Startup, setup_yarnspinner)
            .add_systems(Update, handle_yarnspinner_events.in_set(GameplaySet));
    }
}

fn setup_yarnspinner(
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<LoadYarnProjectEvent>,
) {
    event_writer.send(LoadYarnProjectEvent::with_yarn_source(
        YarnFileSource::Handle(asset_server.load("embedded://ludum_dare_56/dialog/dialog.yarn")),
    ));
}

fn handle_yarnspinner_events(
    mut start_event_reader: EventReader<DialogueStartEvent>,
    mut line_event_reader: EventReader<PresentLineEvent>,
    mut options_event_reader: EventReader<PresentOptionsEvent>,
    mut end_event_reader: EventReader<DialogueCompleteEvent>,
    mut command_event_reader: EventReader<ExecuteCommandEvent>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut dialog_box_content: ResMut<DialogBoxContent>,
    mut commands: Commands,
    runner_query: Query<Entity, With<DialogueRunner>>,
) {
    for _ in start_event_reader.read() {
        next_player_state.set(PlayerState::Dialog);
    }

    for _ in end_event_reader.read() {
        let runner_entity = runner_query.single();
        commands.entity(runner_entity).despawn_recursive();
        next_player_state.set(PlayerState::Free);
    }

    for PresentLineEvent { line, .. } in line_event_reader.read() {
        dialog_box_content.character = line.character_name().map(str::to_string);
        dialog_box_content.line = line.text_without_character_name();
    }

    for ExecuteCommandEvent { command, .. } in command_event_reader.read() {
        match command.name.as_str() {
            "start_quest" => {
                // TODO: Start Quest
                info!("Quest Start Requested {:?}", command.parameters[0]);
            },
            _ => ()
        }
    }
}
