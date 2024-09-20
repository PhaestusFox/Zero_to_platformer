use avian2d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::MainCamera;

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
                custom_size: Some(Vec2::new(32., 32.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::rectangle(32., 32.),
        Friction {
            static_coefficient: 0.,
            combine_rule: CoefficientCombine::Min,
            ..Default::default()
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

fn player_move(
    mut players: Query<(&ActionState<PlayerAction>, &mut LinearVelocity)>,
    time: Res<Time>,
) {
    for (input, mut velocity) in &mut players {
        if input.pressed(&PlayerAction::MoveLeft) {
            velocity.x = -100.;
        } else if input.pressed(&PlayerAction::MoveRight) {
            velocity.x = 100.;
        } else {
            velocity.x = velocity.x.lerp(0., time.delta_seconds() * 4.);
        };
        if input.just_pressed(&PlayerAction::Jump) {
            velocity.0.y += 250.;
        }
    }
}

fn clamp_max_velocity(mut players: Query<&mut LinearVelocity, With<Player>>) {
    for mut player in &mut players {
        player.0.y = player.0.y.clamp(-250., 250.);
    }
}
