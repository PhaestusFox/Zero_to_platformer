use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::camera::MainCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player)
        .add_systems(PostStartup, add_camera_joint)
        .add_systems(Update, (player_move, clamp_max_velocity));
}

#[derive(Component)]
pub struct Player;

fn add_camera_joint(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<MainCamera>>,
) {
    let player = player.single();
    let camera = camera.single();
    commands
        .entity(player)
        .insert(DistanceJoint::new(player, camera));
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        RigidBody::Static,
        LockedAxes::new().lock_rotation(),
        InputManagerBundle {
            action_state: ActionState::default(),
            input_map: default_keybindings(),
        },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.)),
                ..default()
            },
            ..default()
        },
        Collider::rectangle(32., 32.),
        Friction {
            static_coefficient: 0.,
            combine_rule: CoefficientCombine::Min,
            ..default()
        },
    ));
}

#[derive(Actionlike, Reflect, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
}

fn default_keybindings() -> InputMap<PlayerAction> {
    InputMap::new([
        (PlayerAction::MoveLeft, KeyCode::KeyA),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::Jump, KeyCode::KeyW),
        (PlayerAction::MoveLeft, KeyCode::ArrowLeft),
        (PlayerAction::MoveRight, KeyCode::ArrowRight),
        (PlayerAction::Jump, KeyCode::ArrowUp),
    ])
}

const PLAYER_HORIZONTAL_SPEED: f32 = 100.;
const PLAYER_JUMP_SPEED: f32 = 250.;
const PLAYER_SPEED_LIMIT: f32 = 250.;

fn player_move(
    mut players: Query<(&ActionState<PlayerAction>, &mut LinearVelocity)>,
    time: Res<Time>,
) {
    for (input, mut velocity) in &mut players {
        if input.pressed(&PlayerAction::MoveLeft) {
            velocity.x = -PLAYER_HORIZONTAL_SPEED;
        } else if input.pressed(&PlayerAction::MoveRight) {
            velocity.x = PLAYER_HORIZONTAL_SPEED;
        } else {
            velocity.x = velocity.x.lerp(0., time.delta_seconds() * 4.);
        };
        if input.just_pressed(&PlayerAction::Jump) {
            velocity.0.y += PLAYER_JUMP_SPEED;
        }
    }
}

fn clamp_max_velocity(
    mut players: Query<&mut LinearVelocity, With<Player>>
) {
    for mut player in &mut players {
        player.0.y = player.0.y.clamp(-PLAYER_SPEED_LIMIT, PLAYER_SPEED_LIMIT);
    }
}
