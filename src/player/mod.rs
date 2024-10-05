use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{AppState, GameObject};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_player);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameObject,
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));
}
