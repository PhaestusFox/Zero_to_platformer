use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_follow);
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((Camera2dBundle::default(), MainCamera, RigidBody::Dynamic))
        .with_children(|p| {
            p.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::X * 240.),
                    ..default()
                },
                avian2d::prelude::Collider::rectangle(32., 720.),
                avian2d::prelude::RigidBody::Static,
            ));
            p.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::X * -240.),
                    ..default()
                },
                avian2d::prelude::Collider::rectangle(32., 720.),
                avian2d::prelude::RigidBody::Static,
            ));
        });
}

fn camera_follow(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();

    camera.translation = camera.translation.lerp(player.translation.y * Vec3::Y, 0.5);
}
