use std::f32::consts::PI;

use avian3d::{
    collision::Collider,
    prelude::{LockedAxes, RigidBody},
};
use bevy::{pbr::NotShadowCaster, prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_tnua::{
    builtins::TnuaBuiltinWalk,
    controller::{TnuaController, TnuaControllerBundle, TnuaControllerPlugin},
};
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};
use bevy_yarnspinner::prelude::DialogueRunner;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin,
    user_input::KeyboardVirtualDPad, Actionlike, InputControlKind, InputManagerBundle,
};

use crate::{
    dialog, AppState, Billboard, GameCamera, GameObject, GameplaySet, InteractEvent, Interactable,
};

const PLAYER_WALK_SPEED: f32 = 5.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ))
        .insert_state(PlayerState::Free)
        .add_event::<InteractEvent>()
        .add_systems(OnEnter(AppState::InGame), setup_player)
        .add_systems(
            Update,
            (
                (move_player)
                    .in_set(GameplaySet)
                    .run_if(in_state(PlayerState::Free)),
                (handle_player_interaction, update_player_look_direction).in_set(GameplaySet),
            ),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Walk,
    Jump,
    Interact,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Free,
    Dialog,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Walk => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button,
            Self::Interact => InputControlKind::Button,
        }
    }
}

#[derive(Component, Debug)]
pub struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pillbug_texture = asset_server.load("embedded://ludum_dare_56/textures/pillbug.png");
    commands
        .spawn((
            GameObject,
            Player,
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            TnuaControllerBundle::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            LockedAxes::ROTATION_LOCKED,
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
                ..Default::default()
            },
            InputManagerBundle::with_map(
                InputMap::new([
                    (PlayerAction::Jump, KeyCode::Space),
                    (PlayerAction::Interact, KeyCode::KeyE),
                ])
                .with_dual_axis(PlayerAction::Walk, KeyboardVirtualDPad::WASD),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: asset_server.add(
                        PlaneMeshBuilder::new(Dir3::NEG_Z, Vec2::splat(2.0))
                            .build()
                            .rotated_by(Quat::from_axis_angle(Vec3::NEG_Z, PI)),
                    ),
                    material: asset_server.add(StandardMaterial {
                        base_color_texture: Some(pillbug_texture),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)).with_translation(Vec3::new(0.0, -0.5, 0.0)),
                    ..Default::default()
                },
                NotShadowCaster,
            ));
        });
}

fn move_player(
    mut player_query: Query<
        (&Transform, &mut TnuaController, &ActionState<PlayerAction>),
        With<Player>,
    >,
) {
    let (transform, mut controller, action_state) = player_query.single_mut();

    if let Some(walk_input) = action_state.dual_axis_data(&PlayerAction::Walk) {
        let walk_dir =
            walk_input.pair.x * transform.right() + walk_input.pair.y * transform.forward();

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: walk_dir.normalize_or_zero() * PLAYER_WALK_SPEED,
            float_height: 1.5,
            ..Default::default()
        });
    }
}

fn update_player_look_direction(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<GameCamera>>,
    mut transform_query: Query<&mut Transform>,
) {
    let player_entity = player_query.single();
    let camera_entity = camera_query.single();
    let [mut player_transform, camera_transform] =
        transform_query.many_mut([player_entity, camera_entity]);
    let camera_offset = camera_transform.translation - player_transform.translation;
    let look_direction = Vec3::new(-camera_offset.x, 0.0, -camera_offset.z).normalize_or_zero();
    player_transform.look_to(look_direction, Vec3::Y);
}

fn handle_player_interaction(
    state: Res<State<PlayerState>>,
    player_query: Query<(&Transform, &ActionState<PlayerAction>), With<Player>>,
    mut dialog_runner_query: Query<&mut DialogueRunner>,
    interactable_query: Query<(Entity, &Transform), (With<Interactable>, Without<Player>)>,
    mut event_writer: EventWriter<InteractEvent>,
) {
    let (player_transform, action_state) = player_query.single();
    if action_state.just_pressed(&PlayerAction::Interact) {
        match state.get() {
            PlayerState::Free => {
                let mut closest = (None, f32::INFINITY);
                for (interactable_entity, interactable_transform) in interactable_query.iter() {
                    if player_transform
                        .translation
                        .distance_squared(interactable_transform.translation)
                        < (2.0 * 2.0)
                    {
                        let interactable_arccosine = f32::acos(
                            player_transform.forward().dot(
                                (interactable_transform.translation - player_transform.translation)
                                    .normalize(),
                            ),
                        );
                        if interactable_arccosine < closest.1 {
                            closest = (Some(interactable_entity), interactable_arccosine);
                        }
                    }
                }
                if let Some(entity) = closest.0 {
                    event_writer.send(InteractEvent(entity));
                }
            }
            PlayerState::Dialog => {
                let mut dialog_runner = dialog_runner_query.single_mut();
                dialog_runner.continue_in_next_update();
            }
        }
    }
}
