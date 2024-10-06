use bevy::{prelude::*, app::Plugin, prelude::{Component, Query}, transform::components::Transform};

use crate::{GameCamera, GameplaySet};


pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_billboards.in_set(GameplaySet));
    }
}

#[derive(Component, Debug)]
pub struct Billboard;

fn update_billboards(
    camera_query: Query<&Transform, With<GameCamera>>,
    mut billboard_query: Query<&mut Transform, (With<Billboard>, Without<GameCamera>)>,
) {
    let camera_transform = camera_query.single();
    for mut billboard_transform in billboard_query.iter_mut() {
        billboard_transform.look_to(*camera_transform.back(), Vec3::Y);
    }
}
