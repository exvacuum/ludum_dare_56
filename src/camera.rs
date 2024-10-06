use std::f32::consts::PI;

use bevy::prelude::*;
use leafwing_input_manager::{
    action_state::ActionState, input_map::InputMap, input_processing::{AxisProcessor, WithAxisProcessingPipelineExt}, plugin::InputManagerPlugin, user_input::{KeyboardVirtualAxis, MouseMoveAxis, MouseScrollAxis}, Actionlike, InputControlKind, InputManagerBundle
};

use crate::{AppState, GameObject, GameplaySet, Player};

pub const FOV_MIN: f32 = PI / 8.0;
pub const FOV_MAX: f32 = PI;
pub const ZOOM_SPEED: f32 = PI / 8.0;
pub const ROTATE_SPEED: f32 = PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(OnEnter(AppState::InGame), setup_camera)
            .add_systems(
                Update,
                (
                    initialize_camera_offset,
                    camera_follow,
                    apply_camera_controls,
                )
                    .in_set(GameplaySet),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum CameraAction {
    Zoom,
    Rotate,
}

impl Actionlike for CameraAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Zoom => InputControlKind::Axis,
            Self::Rotate => InputControlKind::Axis,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct GameCamera {
    offset: Option<Vec3>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        GameObject,
        GameCamera::default(),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        InputManagerBundle::with_map(
            InputMap::default()
                .with_axis(CameraAction::Zoom, KeyboardVirtualAxis::VERTICAL_ARROW_KEYS)
                .with_axis(CameraAction::Zoom, MouseScrollAxis::Y.with_processor(AxisProcessor::Sensitivity(5.0)))
                .with_axis(
                    CameraAction::Rotate,
                    KeyboardVirtualAxis::HORIZONTAL_ARROW_KEYS,
                )
                .with_axis(CameraAction::Rotate, MouseMoveAxis::X),
        ),
    ));
}

fn initialize_camera_offset(
    mut camera_query: Query<(Entity, &mut GameCamera)>,
    player_query: Query<Entity, With<Player>>,
    transform_query: Query<&Transform>,
) {
    for (camera_entity, mut camera) in camera_query.iter_mut() {
        if camera.offset.is_none() {
            let player_entity = player_query.single();
            let player_transform = transform_query.get(player_entity).unwrap();
            let camera_transform = transform_query.get(camera_entity).unwrap();
            camera.offset = Some(camera_transform.translation - player_transform.translation);
        }
    }
}

fn camera_follow(
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<(Entity, &GameCamera)>,
    mut transform_query: Query<&mut Transform>,
) {
    let (camera_entity, camera) = camera_query.single();
    if let Some(offset) = camera.offset {
        let player_entity = player_query.single();
        let [player_transform, mut camera_transform] =
            transform_query.many_mut([player_entity, camera_entity]);

        camera_transform.translation = player_transform.translation + offset;
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    }
}

fn apply_camera_controls(
    mut camera_query: Query<(
        Entity,
        &mut Projection,
        &ActionState<CameraAction>,
        &mut GameCamera,
    )>,
    player_query: Query<Entity, With<Player>>,
    mut transform_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    let (camera_entity, projection, action_state, mut camera) = camera_query.single_mut();
    let delta = time.delta_seconds();
    if let Some(rotation_input) = action_state.axis_data(&CameraAction::Rotate) {
        if rotation_input.value != 0.0 {
            let player_entity = player_query.single();
            let [player_transform, mut camera_transform] =
                transform_query.many_mut([player_entity, camera_entity]);

            camera_transform.rotate_around(
                player_transform.translation,
                Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    -rotation_input.value * ROTATE_SPEED * delta,
                    0.0,
                ),
            );
            camera.offset = Some(camera_transform.translation - player_transform.translation);
        }
    }
    if let Some(zoom_input) = action_state.axis_data(&CameraAction::Zoom) {
        if let Projection::Perspective(projection) = projection.into_inner() {
            projection.fov =
                (projection.fov + ZOOM_SPEED * delta * -zoom_input.value).clamp(FOV_MIN, FOV_MAX);
        }
    }
}
