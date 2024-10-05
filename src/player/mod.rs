use avian3d::{collision::Collider, prelude::{LockedAxes, RigidBody}};
use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinWalk, controller::{TnuaController, TnuaControllerBundle, TnuaControllerPlugin}};
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, plugin::InputManagerPlugin, user_input::KeyboardVirtualDPad, Actionlike, InputControlKind, InputManagerBundle};

use crate::{AppState, GameObject, GameplaySet};

const PLAYER_WALK_SPEED: f32 = 5.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ))
        .add_systems(OnEnter(AppState::InGame), setup_player)
        .add_systems(Update, move_player.in_set(GameplaySet));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    Walk,
    Jump,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Walk => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button,
        }
    }
}

#[derive(Component, Debug)]
pub struct Player;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
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
        InputManagerBundle::with_map(InputMap::new([(
            PlayerAction::Jump, KeyCode::Space,
        )]).with_dual_axis(PlayerAction::Walk, KeyboardVirtualDPad::WASD)),    
    ));
}

fn move_player(mut player_query: Query<(&mut TnuaController, &ActionState<PlayerAction>), With<Player>>) {
    let (mut controller, action_state) = player_query.single_mut();

    if let Some(walk_input) = action_state.dual_axis_data(&PlayerAction::Walk) {
        let walk_dir = Vec3::new(walk_input.pair.x, 0.0, -walk_input.pair.y);

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: walk_dir.normalize_or_zero() * PLAYER_WALK_SPEED,
            float_height: 1.5,
            ..Default::default()
        });
    }
}
