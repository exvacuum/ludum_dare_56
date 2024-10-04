use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

#[derive(Component, Debug)]
struct Spin(pub f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (hello_world, setup_basic_scene))
        .add_systems(Update, spin)
        .run();
}

fn setup_basic_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::splat(5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(1.0, 1.0, -1.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: asset_server.add(Cuboid::default().mesh().build()),
            material: asset_server.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        },
        Spin(PI),
    ));

    commands.spawn(PbrBundle {
        mesh: asset_server.add(PlaneMeshBuilder::new(Dir3::Y, Vec2::splat(3.0)).build()),
        material: asset_server.add(StandardMaterial::default()),
        ..Default::default()
    });
}

fn hello_world() {
    info!("Hello World!");
}

fn spin(mut spinner_query: Query<(&mut Transform, &Spin)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (mut spinner_transform, spin) in spinner_query.iter_mut() {
        spinner_transform.rotate_y(spin.0 * delta);
    }
}
