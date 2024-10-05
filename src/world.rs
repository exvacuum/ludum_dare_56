use bevy::prelude::*;

use crate::{AppState, GameObject};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_world);
    }
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameObject,
        DirectionalLightBundle {
            transform: Transform::from_translation(Vec3::splat(1.0)).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));
    commands.spawn((
        GameObject,
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("embedded://ludum_dare_56/models/world.glb")),
            ..Default::default()
        }
    ));
}
