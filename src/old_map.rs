use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng, SeedableRng};
use strum::IntoEnumIterator;

use crate::player::Player;

pub fn plugin(app: &mut App) {
    app.init_resource::<SpriteSheet>()
        .init_resource::<Seed>()
        .add_systems(Update, (spawn_walls, cull_walls, spawn_layer));
}

#[derive(Resource)]
struct SpriteSheet(Handle<Image>, Handle<TextureAtlasLayout>);

enum SpriteId {
    Box = 7,
    Wall = 12 * 13 + 2,
    Floor = 12 * 19 + 6,
}

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let image = world.resource::<AssetServer>().load("blocks.png");
        let atlas = TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            12,
            20,
            Some(UVec2::new(8, 8)),
            Some(UVec2::splat(16)),
        );
        let atlas = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(atlas);
        SpriteSheet(image, atlas)
    }
}

fn test_atlas(mut commands: Commands, atlas: Res<SpriteSheet>) {
    for x in 0..12 {
        for y in 0..20 {
            commands.spawn((
                SpriteBundle {
                    texture: atlas.0.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 20.,
                        y as f32 * 20.,
                        0.,
                    )),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: atlas.1.clone(),
                    index: y * 12 + x,
                },
            ));
        }
    }
}

#[derive(Component)]
struct Wall;

#[derive(Resource)]
struct Seed(u64);

impl FromWorld for Seed {
    fn from_world(world: &mut World) -> Self {
        #[cfg(not(debug_assertions))]
        return Seed(rand::thread_rng().gen());

        #[cfg(debug_assertions)]
        Seed(42069)
    }
}

fn cull_walls(
    mut commands: Commands,
    walls: Query<(Entity, &Transform), With<Wall>>,
    player: Query<&Transform, With<Player>>,
) {
    let player = player.single().translation;
    for (id, wall) in &walls {
        if wall.translation.distance_squared(player) > 1250000. {
            commands.entity(id).despawn_recursive();
        }
    }
}

fn spawn_walls(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut last: Local<isize>,
    sprite_sheet: Res<SpriteSheet>,
) {
    let player = player.single().translation.y;
    let cell = (player / 32.) as isize;
    if cell - 16 < *last {
        for layer in (cell - 32)..*last {
            commands.spawn((
                SpriteBundle {
                    texture: sprite_sheet.0.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        -240.,
                        layer as f32 * 32.,
                        0.,
                    )),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(32.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TextureAtlas {
                    layout: sprite_sheet.1.clone(),
                    index: SpriteId::Wall as usize,
                },
                Wall,
            ));
            commands.spawn((
                SpriteBundle {
                    texture: sprite_sheet.0.clone(),
                    transform: Transform::from_translation(Vec3::new(240., layer as f32 * 32., 0.)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(32.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TextureAtlas {
                    layout: sprite_sheet.1.clone(),
                    index: SpriteId::Wall as usize,
                },
                Wall,
            ));
        }
        *last = cell - 32;
    }
}

fn spawn_layer(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut last: Local<isize>,
    sprite_sheet: Res<SpriteSheet>,
    seed: Res<Seed>,
) {
    let player = player.single().translation.y;
    let layer = (player / 128.) as isize;
    if layer - 4 < *last {
        for layer in (layer - 8)..*last {
            let mut rng =
                rand::rngs::StdRng::seed_from_u64(seed.0.wrapping_add(layer as u64) ^ seed.0);
            if !rng.gen_bool(0.25) {
                continue;
            }
            LayerType::iter()
                .choose(&mut rng)
                .expect("At least one Layer Type To exist")
                .gen(&mut commands, &sprite_sheet, &mut rng, layer);
        }
        *last = layer - 8;
    }
}

#[derive(strum_macros::EnumIter, Debug)]
enum LayerType {
    Single,
    LineOpenEnd,
    LineOpenMiddle,
    Even,
    Odd,
}

impl LayerType {
    fn gen(
        &self,
        commands: &mut Commands,
        sprite_sheet: &SpriteSheet,
        rng: &mut impl Rng,
        layer: isize,
    ) {
        let base_layer = layer as f32 * 128.;
        info!("Spawning Layer {:?}", self);
        match self {
            LayerType::Single => {
                let pos = rng.gen_range(0..14);
                commands.spawn((
                    SpriteBundle {
                        texture: sprite_sheet.0.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            -208. + pos as f32 * 32.,
                            base_layer,
                            0.,
                        )),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(32.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextureAtlas {
                        layout: sprite_sheet.1.clone(),
                        index: SpriteId::Floor as usize,
                    },
                    Name::new(format!("Layer {}", layer)),
                    Wall,
                    Collider::rectangle(32., 32.),
                    RigidBody::Static,
                ));
            }
            LayerType::LineOpenEnd => {
                let pos = rng.gen_range(1..4);
                commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(Vec3::new(0., base_layer, 0.)),
                            ..Default::default()
                        },
                        Collider::rectangle((14 - pos * 2) as f32 * 32., 32.),
                        RigidBody::Static,
                        Name::new(format!("Layer {}", layer)),
                        Wall,
                    ))
                    .with_children(|commands| {
                        for i in pos..14 - pos {
                            commands.spawn((
                                SpriteBundle {
                                    texture: sprite_sheet.0.clone(),
                                    transform: Transform::from_translation(Vec3::new(
                                        -208. + i as f32 * 32.,
                                        0.,
                                        0.,
                                    )),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(32.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                TextureAtlas {
                                    layout: sprite_sheet.1.clone(),
                                    index: SpriteId::Floor as usize,
                                },
                            ));
                        }
                    });
            }
            LayerType::LineOpenMiddle => {
                let pos = rng.gen_range(1..4);
                commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(Vec3::new(0., base_layer, 0.)),
                            ..Default::default()
                        },
                        Name::new(format!("Layer {}", layer)),
                        Wall,
                    ))
                    .with_children(|commands| {
                        commands
                            .spawn((
                                Name::new("Left"),
                                SpatialBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        (-3. - pos as f32) * 32.,
                                        0.,
                                        0.,
                                    )),
                                    ..Default::default()
                                },
                                Collider::rectangle((7 - pos) as f32 * 32., 32.),
                                RigidBody::Static,
                            ))
                            .with_children(|commands| {
                                for i in 0..7 - pos {
                                    commands.spawn((
                                        SpriteBundle {
                                            texture: sprite_sheet.0.clone(),
                                            transform: Transform::from_translation(Vec3::new(
                                                -80. + i as f32 * 32.,
                                                0.,
                                                0.,
                                            )),
                                            sprite: Sprite {
                                                custom_size: Some(Vec2::splat(32.)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        TextureAtlas {
                                            layout: sprite_sheet.1.clone(),
                                            index: SpriteId::Floor as usize,
                                        },
                                    ));
                                }
                            });
                        commands
                            .spawn((
                                Name::new("Right"),
                                SpatialBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        (3. + pos as f32) * 32.,
                                        0.,
                                        0.,
                                    )),
                                    ..Default::default()
                                },
                                Collider::rectangle((7 - pos) as f32 * 32., 32.),
                                RigidBody::Static,
                            ))
                            .with_children(|commands| {
                                for i in 0..7 - pos {
                                    commands.spawn((
                                        SpriteBundle {
                                            texture: sprite_sheet.0.clone(),
                                            transform: Transform::from_translation(Vec3::new(
                                                80. - i as f32 * 32.,
                                                0.,
                                                0.,
                                            )),
                                            sprite: Sprite {
                                                custom_size: Some(Vec2::splat(32.)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        TextureAtlas {
                                            layout: sprite_sheet.1.clone(),
                                            index: SpriteId::Floor as usize,
                                        },
                                    ));
                                }
                            });
                    });
            }
            LayerType::Even => {
                for i in 0..14 {
                    if i % 2 == 1 {
                        continue;
                    }
                    commands.spawn((
                        SpriteBundle {
                            texture: sprite_sheet.0.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                -208. + i as f32 * 32.,
                                base_layer,
                                0.,
                            )),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(32.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        TextureAtlas {
                            layout: sprite_sheet.1.clone(),
                            index: SpriteId::Floor as usize,
                        },
                        Wall,
                        Collider::rectangle(32., 32.),
                        RigidBody::Static,
                    ));
                }
            }
            LayerType::Odd => {
                for i in 0..14 {
                    if i % 2 == 0 {
                        continue;
                    }
                    commands.spawn((
                        SpriteBundle {
                            texture: sprite_sheet.0.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                -208. + i as f32 * 32.,
                                base_layer,
                                0.,
                            )),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(32.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        TextureAtlas {
                            layout: sprite_sheet.1.clone(),
                            index: SpriteId::Floor as usize,
                        },
                        Wall,
                        Collider::rectangle(32., 32.),
                        RigidBody::Static,
                    ));
                }
            }
        }
    }
}
