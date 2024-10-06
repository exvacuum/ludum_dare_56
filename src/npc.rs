use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

use crate::{GameplaySet, InteractEvent};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_npc_interactions.in_set(GameplaySet));
    }
}

#[derive(Component, Debug)]
pub struct Npc(pub String);

fn handle_npc_interactions(
    npc_query: Query<&Npc>,
    mut event_reader: EventReader<InteractEvent>,
    project: Res<YarnProject>,
    mut commands: Commands,
) {
    for InteractEvent(entity) in event_reader.read() {
        if let Ok(Npc(node)) = npc_query.get(*entity) {
            let mut dialog_runner = project.create_dialogue_runner();
            dialog_runner.start_node(node);
            commands.spawn(dialog_runner);
        }
    }
}
