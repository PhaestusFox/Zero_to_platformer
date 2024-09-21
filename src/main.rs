use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

mod camera;
mod map;
mod player;

#[cfg(debug_assertions)]
mod editor_window;

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
                    ..default()
                }),
                ..default()
            }),
        avian2d::PhysicsPlugins::default().with_length_unit(32.),
        leafwing_input_manager::prelude::InputManagerPlugin::<player::PlayerAction>::default(),
    ))
    .insert_resource(Gravity(Vec2::Y * -500.))
    .add_plugins((player::plugin, map::plugin, camera::plugin));

    #[cfg(debug_assertions)]
    app.add_plugins((
        bevy_editor_pls::EditorPlugin::default(),
        avian2d::debug_render::PhysicsDebugPlugin::default(),
        editor_window::setup,
    ));
    app.run();
}
