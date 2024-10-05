use std::f32::consts::PI;

use bevy::{ecs::query, prelude::*, render::mesh::PlaneMeshBuilder, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}, utils::warn};

mod ui;
use ui::*;

mod player;
use player::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    Paused,
    #[default]
    Running,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameplaySet;

#[derive(Component, Debug)]
struct Spin(pub f32);

#[derive(Component, Debug)]
struct GameObject;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin))
        .init_state::<AppState>()
        .init_state::<PausedState>()
        .add_systems(Startup, hello_world)
        .add_systems(OnEnter(AppState::InGame), setup_basic_scene)
        .add_systems(OnExit(AppState::InGame), clean_up_game)
        .add_systems(Update, (spin).in_set(GameplaySet))
        .configure_sets(
            Update,
            (GameplaySet
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PausedState::Running)),),
        )
        .run();
}

fn setup_basic_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameObject,
        Camera2dBundle::default(),
    ));
    commands.spawn((
        GameObject,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(asset_server.add(Rectangle::new(100.0, 100.0).mesh().build())),
            material: asset_server.add(ColorMaterial::from_color(Color::srgb(1.0, 0.0, 0.0))),
            ..Default::default()
        },
        Spin(PI),
    ));
}

fn clean_up_game(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn hello_world() {
    info!("Hello World!");
}

fn spin(mut spinner_query: Query<(&mut Transform, &Spin)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (mut spinner_transform, spin) in spinner_query.iter_mut() {
        spinner_transform.rotate_z(spin.0 * delta);
    }
}
