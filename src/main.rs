use avian2d::prelude::{DistanceJoint, Gravity, Joint, RigidBody};
use bevy::{prelude::*, window::WindowResolution};

mod map;
mod player;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "WownDell".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(480., 720.),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        avian2d::PhysicsPlugins::default().with_length_unit(32.),
        leafwing_input_manager::prelude::InputManagerPlugin::<player::PlayerAction>::default(),
    ))
    .insert_resource(Gravity(Vec2::Y * -500.))
    .add_systems(Startup, spawn_camera)
    .add_plugins((player::plugin, map::plugin))
    .add_systems(Update, camera_follow);
    #[cfg(debug_assertions)]
    app.add_plugins((
        bevy_editor_pls::EditorPlugin::default(),
        avian2d::debug_render::PhysicsDebugPlugin::default(),
    ));
    app.run();
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((Camera2dBundle::default(), MainCamera, RigidBody::Dynamic))
        .with_children(|p| {
            p.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::X * 240.),
                    ..Default::default()
                },
                avian2d::prelude::Collider::rectangle(32., 720.),
                avian2d::prelude::RigidBody::Static,
            ));
            p.spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::X * -240.),
                    ..Default::default()
                },
                avian2d::prelude::Collider::rectangle(32., 720.),
                avian2d::prelude::RigidBody::Static,
            ));
        });
}

fn camera_follow(
    player: Query<&Transform, (With<player::Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();

    camera.translation = camera.translation.lerp(player.translation.y * Vec3::Y, 0.5);
}
