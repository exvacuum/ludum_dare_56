use avian3d::{debug_render::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::{prelude::*, window::{Cursor, CursorGrabMode, WindowResolution}, winit::WinitPlugin};

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

mod embedded_assets;
use embedded_assets::*;

mod dialog;
use dialog::*;

mod interactable;
use interactable::*;

mod npc;
use npc::*;

mod billboard;
use billboard::*;

mod quests;
use quests::*;

const GAME_TITLE: &str = "The Many Beautiful Moments That We Struggle to Remember, and/or the Dull Yet Present Sense of Joy They Leave in Their Place, or The Pillbug's Quest";

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
        DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_TITLE.into(),
                resizable: false,
                resolution: WindowResolution::new(640.0, 480.0),
                ..Default::default()
            }),
            ..Default::default()
        }),
        PhysicsPlugins::default(),
        UiPlugin,
        WorldPlugin,
        PlayerPlugin,
        CameraPlugin,
        DialogPlugin,
        NpcPlugin,
        BillboardPlugin,
        QuestsPlugin,
    ))
    .init_state::<AppState>()
    .init_state::<PausedState>()
    .add_systems(OnExit(AppState::InGame), clean_up_game)
    .configure_sets(
        PreUpdate,
        (GameplaySet
            .run_if(in_state(AppState::InGame))
            .run_if(in_state(PausedState::Running)),),
    )
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
