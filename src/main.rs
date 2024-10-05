use avian3d::{debug_render::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::prelude::*;

mod ui;
use ui::*;

mod player;
use player::*;

mod camera;
use camera::*;

mod bugoid;
use bugoid::*;

mod world;
use world::*;

mod quests;
use quests::*;

mod embedded_assets;
use embedded_assets::*;

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
struct GameObject;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        UiPlugin,
        WorldPlugin,
        PlayerPlugin,
        CameraPlugin,
    ))
    .init_state::<AppState>()
    .init_state::<PausedState>()
    .add_systems(OnExit(AppState::InGame), clean_up_game)
    .configure_sets(
        Update,
        (GameplaySet
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(PausedState::Running)),),
    )
    .configure_sets(
        PostUpdate,
        (GameplaySet
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(PausedState::Running)),),
    );
    embed_assets(&mut app);
    app.run();
}

fn clean_up_game(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
