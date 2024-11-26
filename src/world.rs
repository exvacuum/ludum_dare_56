use std::f32::consts::PI;

use avian3d::{
    collision::{Collider, ColliderConstructor, ColliderConstructorHierarchy},
    prelude::{LockedAxes, RigidBody},
};
use bevy::{pbr::NotShadowCaster, prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::{
    input_map::InputMap, input_processing::{AxisProcessor, WithAxisProcessingPipelineExt}, user_input::{KeyboardVirtualAxis, KeyboardVirtualDPad, MouseMoveAxis, MouseScrollAxis}, InputManagerBundle
};

use crate::{
    AppState, Billboard, CameraAction, GameCamera, GameObject, GameplaySet, Interactable, Npc, Player, PlayerAction
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_world)
            .add_systems(Update, handle_world_load.in_set(GameplaySet));
    }
}

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameObject,
        DirectionalLightBundle {
            transform: Transform::from_translation(Vec3::splat(1.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        GameObject,
        SceneBundle {
            scene: asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("embedded://ludum_dare_56/models/world.glb"),
            ),
            ..Default::default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));
}

fn handle_world_load(
    mut commands: Commands,
    new_world_object_query: Query<(&Transform, &Name), Added<Name>>,
    asset_server: Res<AssetServer>,
) {
    for (transform, name) in new_world_object_query.iter() {
        match name.as_str() {
            "Ant_Spawn" => {
                let ant_texture = asset_server.load("embedded://ludum_dare_56/textures/ant.png");

                commands.spawn((
                    Billboard,
                    PbrBundle {
                        mesh: asset_server.add(
                            PlaneMeshBuilder::new(Dir3::NEG_Z, Vec2::splat(2.0))
                                .build()
                                .rotated_by(Quat::from_axis_angle(Vec3::NEG_Z, PI)),
                        ),
                        material: asset_server.add(StandardMaterial {
                            base_color_texture: Some(ant_texture),
                            alpha_mode: AlphaMode::Blend,
                            unlit: true,
                            ..Default::default()
                        }),
                        transform: *transform * Transform::from_translation(Vec3::Y),
                        ..Default::default()
                    },
                    Npc("Ant_Start".to_string()),
                    Interactable,
                    NotShadowCaster,
                ));
            }
            "Player_Spawn" => {
                commands.spawn((
                    GameObject,
                    GameCamera::default(),
                    Camera3dBundle {
                        transform: (*transform * Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)))
                            .looking_at(transform.translation, Vec3::Y),
                        ..Default::default()
                    },
                    InputManagerBundle::with_map(
                        InputMap::default()
                            .with_axis(CameraAction::Zoom, KeyboardVirtualAxis::VERTICAL_ARROW_KEYS)
                            .with_axis(
                                CameraAction::Zoom,
                                MouseScrollAxis::Y.with_processor(AxisProcessor::Sensitivity(5.0)),
                            )
                            .with_axis(
                                CameraAction::Rotate,
                                KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS,
                            )
                            .with_axis(CameraAction::Rotate, MouseMoveAxis::X),
                    ),
                ));
                let pillbug_texture =
                    asset_server.load("embedded://ludum_dare_56/textures/pillbug.png");
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
                            transform: *transform,
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
                                transform: Transform::from_rotation(Quat::from_axis_angle(
                                    Vec3::Y,
                                    PI,
                                ))
                                .with_translation(Vec3::new(0.0, -0.5, 0.0)),
                                ..Default::default()
                            },
                            NotShadowCaster,
                        ));
                    });
            }
            _ => (),
        }
    }
}
