use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadedFolder},
    prelude::*,
    utils::HashMap,
};
use rand::{seq::IteratorRandom, SeedableRng};
use strum::IntoEnumIterator;

pub fn plugin(app: &mut App) {
    app
        .init_resource::<Tiles>()
        .init_asset::<TileDescriptor>()
        .register_type::<TileSprite>()
        .register_asset_loader(MapLoader)
        .register_asset_loader(TileDescriptorLoader)
        .init_state::<MapState>()
        .init_asset::<MapData>()
        .register_type::<MapData>()
        .init_resource::<CurrentMap>()
        .init_resource::<LoadMap>()
        .init_resource::<SpriteSheet>()
        .add_systems(
          Update,
          (
            update_tile,
            set_tile
          ).chain()
        )
        .add_systems(
            Update,
            (
                check_loaded.run_if(in_state(MapState::Loading)),
                start_loading.run_if(resource_changed::<LoadMap>),
                detect_changes.run_if(in_state(MapState::Done)),
            ),
        )
        .add_systems(
          OnEnter(MapState::Spawning),
          spawn_map
        )
        .add_systems(
          Last,
          set_done.run_if(in_state(MapState::Spawning))
        );
}

#[derive(Resource)]
#[allow(dead_code)] // TODO: reason as to why, I wasn't paying full attention at this point to say exactly why - Skylark
struct Tiles(Handle<LoadedFolder>);
impl FromWorld for Tiles {
    fn from_world(world: &mut World) -> Self {
        Tiles(world.resource::<AssetServer>().load_folder("tiles"))
    }
}

#[derive(Resource)]
struct LoadMap(String);

impl FromWorld for LoadMap {
    fn from_world(_: &mut World) -> Self {
        LoadMap("maps/lobby.map".to_string())
    }
}

#[derive(Resource)]
struct SpriteSheet(Handle<Image>, Handle<TextureAtlasLayout>);

impl SpriteSheet {
    fn image(&self) -> Handle<Image> {
        self.0.clone()
    }

    fn atlas(&self) -> Handle<TextureAtlasLayout> {
        self.1.clone()
    }
}

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let image = world.resource::<AssetServer>().load("tilemap.png");
        let atlas = TextureAtlasLayout::from_grid(UVec2::splat(8), 15, 10, None, None);
        let atlas = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(atlas);
        SpriteSheet(image, atlas)
    }
}

#[derive(Component)]
struct TileId(IVec3);

#[derive(
    Component,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::EnumIter,
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Reflect,
)]
enum TileSprite {
    Air,
    TreeTopYellow,
    DirtEmpty,
    DirtLowLeftYellow,
    DirtLowYellow,
    DirtLowRightYellow,
    DirtTopSlopeUpYellow,
    DirtTopSlopeDownYellow,
    TreeTopPink,
    DirtBones,
    DirtLowLeftPink,
    DirtLowPink,
    DirtLowRightPink,
    DirtTopSlopeUpPink,
    DirtTopSlopeDownPink,
    SpringDown,
    TreeMiddleYellow,
    DirtSpots,
    DirtRightYellow,
    DirtFullYellow,
    DirtLeftYellow,
    DirtRightDotYellow,
    DirtLeftDotYellow,
    TreeMiddlePink,
    DirtBone,
    DirtRightPink,
    DirtFullPink,
    DirtLeftPink,
    DirtRightDotPink,
    DirtLeftDotPink,
    SpringUp,
    TreeBottomYellow,
    DirtCross,
    DirtTopRightYellow,
    DirtTopYellow,
    DirtTopLeftYellow,
    DirtBottomRightDotYellow,
    DirtBottomLeftDotYellow,
    TreeBottomPink,
    DirtSkull,
    DirtTopRightPink,
    DirtTopPink,
    DirtTopLeftPink,
    DirtBottomRightDotPink,
    DirtBottomLeftDotPink,
    PlatformOff,
    TreeStemYellow,
    AllBarBottomYellow,
    AllBarRightYellow,
    TopBottomYellow,
    AllBarLeftYellow,
    DirtBottomSlopeDownYellow,
    DirtBottomSlopeUpYellow,
    TreeStemPink,
    AllBarBottomPink,
    AllBarRightPink,
    TopBottomPink,
    AllBarLeftPink,
    DirtBottomSlopeDownPink,
    DirtBottomSlopeUpPink,
    PlatformOn,
    TreeTrunkYellow,
    LeftRightYellow,
    TopLeftYellow,
    TopRightYellow,
    GrassYellow,
    Key,
    Lock,
    TreeTrunkPink,
    LeftRightPink,
    TopLeftPink,
    TopRightPink,
    GrassPink,
    Flag,
    Pole,
    Spike,
    Void,
    AllBarTopYellow,
    BottomLeftYellow,
    BottomRightYellow,
    HeartFlowerYellow,
    Cloud,
    Smoke,
    Steam,
    AllBarTopPink,
    BottomLeftPink,
    BottomRightPink,
    HeartFlowerPink,
    Coin,
    Magnet,
    PlayerRedStand,
    PlayerRedWalk,
    PlayerRedJump,
    PlayerRedDie,
    RedCubeFace,
    RedCircle,
    RedDot,
    BlueCircleFace,
    BlueDiamond,
    BlueDots,
}

impl TileSprite {
    fn is_sold(&self) -> bool {
        match self {
            TileSprite::Air => false,
            TileSprite::TreeTopYellow => true,
            TileSprite::DirtEmpty => true,
            TileSprite::DirtLowLeftYellow => true,
            TileSprite::DirtLowYellow => true,
            TileSprite::DirtLowRightYellow => true,
            TileSprite::DirtTopSlopeUpYellow => true,
            TileSprite::DirtTopSlopeDownYellow => true,
            TileSprite::TreeTopPink => true,
            TileSprite::DirtBones => true,
            TileSprite::DirtLowLeftPink => true,
            TileSprite::DirtLowPink => true,
            TileSprite::DirtLowRightPink => true,
            TileSprite::DirtTopSlopeUpPink => true,
            TileSprite::DirtTopSlopeDownPink => true,
            TileSprite::SpringDown => false,
            TileSprite::TreeMiddleYellow => true,
            TileSprite::DirtSpots => true,
            TileSprite::DirtRightYellow => true,
            TileSprite::DirtFullYellow => true,
            TileSprite::DirtLeftYellow => true,
            TileSprite::DirtRightDotYellow => true,
            TileSprite::DirtLeftDotYellow => true,
            TileSprite::TreeMiddlePink => true,
            TileSprite::DirtBone => true,
            TileSprite::DirtRightPink => true,
            TileSprite::DirtFullPink => true,
            TileSprite::DirtLeftPink => true,
            TileSprite::DirtRightDotPink => true,
            TileSprite::DirtLeftDotPink => true,
            TileSprite::SpringUp => false,
            TileSprite::TreeBottomYellow => true,
            TileSprite::DirtCross => true,
            TileSprite::DirtTopRightYellow => true,
            TileSprite::DirtTopYellow => true,
            TileSprite::DirtTopLeftYellow => true,
            TileSprite::DirtBottomRightDotYellow => true,
            TileSprite::DirtBottomLeftDotYellow => true,
            TileSprite::TreeBottomPink => true,
            TileSprite::DirtSkull => true,
            TileSprite::DirtTopRightPink => true,
            TileSprite::DirtTopPink => true,
            TileSprite::DirtTopLeftPink => true,
            TileSprite::DirtBottomRightDotPink => true,
            TileSprite::DirtBottomLeftDotPink => true,
            TileSprite::PlatformOff => false,
            TileSprite::TreeStemYellow => false,
            TileSprite::AllBarBottomYellow => true,
            TileSprite::AllBarRightYellow => true,
            TileSprite::TopBottomYellow => true,
            TileSprite::AllBarLeftYellow => true,
            TileSprite::DirtBottomSlopeDownYellow => true,
            TileSprite::DirtBottomSlopeUpYellow => true,
            TileSprite::TreeStemPink => false,
            TileSprite::AllBarBottomPink => true,
            TileSprite::AllBarRightPink => true,
            TileSprite::TopBottomPink => true,
            TileSprite::AllBarLeftPink => true,
            TileSprite::DirtBottomSlopeDownPink => true,
            TileSprite::DirtBottomSlopeUpPink => true,
            TileSprite::PlatformOn => false,
            TileSprite::TreeTrunkYellow => false,
            TileSprite::LeftRightYellow => true,
            TileSprite::TopLeftYellow => true,
            TileSprite::TopRightYellow => true,
            TileSprite::GrassYellow => false,
            TileSprite::Key => false,
            TileSprite::Lock => false,
            TileSprite::TreeTrunkPink => false,
            TileSprite::LeftRightPink => true,
            TileSprite::TopLeftPink => true,
            TileSprite::TopRightPink => true,
            TileSprite::GrassPink => false,
            TileSprite::Flag => false,
            TileSprite::Pole => false,
            TileSprite::Spike => false,
            TileSprite::Void => false,
            TileSprite::AllBarTopYellow => true,
            TileSprite::BottomLeftYellow => true,
            TileSprite::BottomRightYellow => true,
            TileSprite::HeartFlowerYellow => true,
            TileSprite::Cloud => false,
            TileSprite::Smoke => false,
            TileSprite::Steam => false,
            TileSprite::AllBarTopPink => true,
            TileSprite::BottomLeftPink => true,
            TileSprite::BottomRightPink => true,
            TileSprite::HeartFlowerPink => false,
            TileSprite::Coin => false,
            TileSprite::Magnet => false,
            TileSprite::PlayerRedStand => false,
            TileSprite::PlayerRedWalk => false,
            TileSprite::PlayerRedJump => false,
            TileSprite::PlayerRedDie => false,
            TileSprite::RedCubeFace => false,
            TileSprite::RedCircle => false,
            TileSprite::RedDot => false,
            TileSprite::BlueCircleFace => false,
            TileSprite::BlueDiamond => false,
            TileSprite::BlueDots => false,
        }
    }
}

fn check_loaded(
    mut load_map: EventReader<AssetEvent<MapData>>,
    mut next: ResMut<NextState<MapState>>,
    current: Res<CurrentMap>,
) {
    for event in load_map.read() {
        match event {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id } => {
                if *id == current.0.id() {
                    next.set(MapState::Spawning);
                }
            }
            _ => {}
        }
    }
}

fn set_tile(mut tiles: Query<(&TileSprite, &mut TextureAtlas), Changed<TileSprite>>) {
    for (tile, mut atlas) in &mut tiles {
        atlas.index = *tile as usize;
    }
}

#[derive(Resource, Default)]
struct CurrentMap(Handle<MapData>);

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum MapState {
    #[default]
    Loading,
    Spawning,
    Done,
}

fn start_loading(
    to_load: Res<LoadMap>,
    asset_server: Res<AssetServer>,
    mut current: ResMut<CurrentMap>,
    mut next: ResMut<NextState<MapState>>,
) {
    current.0 = asset_server.load(&to_load.0);
    next.set(MapState::Loading);
}

fn set_done(mut next: ResMut<NextState<MapState>>) {
    next.set(MapState::Done);
}

fn spawn_map(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    map_data: Res<Assets<MapData>>,
    target: Res<CurrentMap>,
) {
    let Some(map_data) = map_data.get(target.0.id()) else {
        error!("Map Not Loaded");
        return;
    };
    let mut map_entities = MapEntities::new();
    commands
        .spawn(SpatialBundle::default())
        .with_children(|map| {
            for block in map_data.blocks.iter() {
                let id = block.translation;
                if map_entities.empty(id) {
                    map_entities.add(
                        id,
                        map.spawn((
                            SpriteBundle {
                                transform: Transform::from_translation(
                                    block.translation.as_vec3() * 32.,
                                ),
                                texture: sprite_sheet.image(),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(32.)),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: sprite_sheet.atlas(),
                                index: 0,
                            },
                            TileSprite::Air,
                            block.color,
                            block.tile,
                            block.variant,
                            TileId(id),
                        ))
                        .id(),
                    );
                }
            }
        })
        .insert(map_entities);
}

#[derive(Component)]
struct MapEntities(HashMap<IVec3, Entity>);

impl MapEntities {
    fn new() -> MapEntities {
        MapEntities(HashMap::default())
    }

    fn add(&mut self, pos: IVec3, entity: Entity) -> bool {
        if self.0.contains_key(&pos) {
            false
        } else {
            self.0.insert(pos, entity);
            true
        }
    }

    fn empty(&self, pos: IVec3) -> bool {
        !self.0.contains_key(&pos)
    }

    fn get(&self, id: IVec3) -> Option<Entity> {
        self.0.get(&id).copied()
    }
}

#[derive(Asset, Reflect)]
struct MapData {
    blocks: Vec<Block>,
}

#[derive(
    Reflect, Component, Clone, Copy, serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug,
)]
enum Tile {
    Dirt,
    Tree,
    Spring,
    Platform,
    Collectable,
}

impl Tile {
    fn is_solid(&self) -> bool {
        match self {
            Tile::Dirt => true,
            Tile::Tree => false,
            Tile::Spring => false,
            Tile::Platform => false,
            Tile::Collectable => false,
        }
    }

    fn parse<'a, T: Iterator<Item = &'a str>>(self, mut words: T) -> Result<Block, &'static str> {
        match self {
            Tile::Dirt => {
                let mut block = Block {
                    tile: self,
                    color: Team::None,
                    variant: Variant::Default,
                    translation: IVec3::default(),
                };
                while let Some(word) = words.next() {
                    match word.trim().to_lowercase().as_str() {
                        "x" => {
                            let Some(num) = words.next() else {
                                return Err("No number after 'X'");
                            };
                            let Ok(num) = num.parse() else {
                                return Err("Word after 'X' is not a int");
                            };
                            block.translation.x = num;
                        }
                        "y" => {
                            let Some(num) = words.next() else {
                                return Err("No number after 'Y'");
                            };
                            let Ok(num) = num.parse() else {
                                return Err("Word after 'Y' is not a int");
                            };
                            block.translation.y = num;
                        }
                        "z" => {
                            let Some(num) = words.next() else {
                                return Err("No number after 'Z'");
                            };
                            let Ok(num) = num.parse() else {
                                return Err("Word after 'Z' is not a int");
                            };
                            block.translation.z = num;
                        }
                        "rand" | "random" => block.variant = Variant::Random,
                        "variant" => {
                            let Some(num) = words.next() else {
                                return Err("No number after 'variant'");
                            };
                            let Ok(num) = num.parse() else {
                                return Err("Word after 'variant' is not a u8");
                            };
                            block.variant = Variant::Fixed(num)
                        }
                        "yellow" => block.color = Team::Yellow,
                        "pink" => block.color = Team::Pink,
                        e => {
                            error!("Unknown word {e}");
                        }
                    }
                }
                Ok(block)
            }
            Tile::Tree => todo!(),
            Tile::Spring => todo!(),
            Tile::Platform => todo!(),
            Tile::Collectable => todo!(),
        }
    }
}

#[derive(
    Reflect, Component, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug,
)]
enum Team {
    Yellow,
    Pink,
    Any,
    None,
}

#[derive(Reflect, Component, Clone, Copy)]
enum Variant {
    Default,
    Random,
    Fixed(u8),
}

#[derive(Reflect)]
struct Block {
    tile: Tile,
    color: Team,
    translation: IVec3,
    variant: Variant,
}

#[derive(strum_macros::EnumIter, Clone, Copy, PartialEq, Eq)]
enum Adjacencies {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

impl std::ops::Add<Adjacencies> for IVec3 {
    type Output = IVec3;
    fn add(mut self, rhs: Adjacencies) -> Self::Output {
        match rhs {
            Adjacencies::UpLeft => {
                self.x -= 1;
                self.y += 1;
            }
            Adjacencies::Up => {
                self.y += 1;
            }
            Adjacencies::UpRight => {
                self.x += 1;
                self.y += 1;
            }
            Adjacencies::Right => {
                self.x += 1;
            }
            Adjacencies::DownRight => {
                self.x += 1;
                self.y -= 1;
            }
            Adjacencies::Down => {
                self.y -= 1;
            }
            Adjacencies::DownLeft => {
                self.y -= 1;
                self.x -= 1;
            }
            Adjacencies::Left => {
                self.x -= 1;
            }
        }
        self
    }
}

fn update_tile(
    mut sprites: Query<
        (&mut TileSprite, &TileId, &Tile, &Team, &Variant),
        Or<(Changed<Tile>, Changed<Team>, Changed<Variant>)>,
    >,
    tiles: Query<(&Tile, &Team, &Variant)>,
    map: Query<&MapEntities>,
    tile_descriptors: Res<Assets<TileDescriptor>>,
) {
    let Ok(map) = map.get_single() else {
        return;
    };
    let tile_descriptors = tile_descriptors
        .iter()
        .map(|d| d.1)
        .cloned()
        .collect::<Vec<_>>();
    for (mut sprite, id, tile, team, variant) in &mut sprites {
        let mut tile_builder = TileSpriteBuilder::new(tile_descriptors.clone());
        tile_builder.set_team(*team);
        tile_builder.set_variant(*variant);
        tile_builder.set_tile(*tile);
        for adjacent in Adjacencies::iter() {
            if let Some(id) = map.get(id.0 + adjacent) {
                if let Ok(to) = tiles.get(id) {
                    tile_builder.set_adjacent(adjacent, to.0.is_solid());
                }
            }
        }
        tile_builder
            .set_seed((((id.0.x as u64) << 32) ^ id.0.y as u64).wrapping_add(id.0.z as u64));
        tile_builder.resolve();

        if let Some(to) = tile_builder.result() {
            *sprite = to;
        } else {
            error!("tile builder failed to resolve");
        }
    }
}

struct TileSpriteBuilder {
    seed: u64,
    tiles: Vec<TileDescriptor>,
    tile: Option<Tile>,
    team: Option<Team>,
    variant: Variant,
    adjacent_solid: [bool; 8],
}

impl TileSpriteBuilder {
    fn new(tiles: Vec<TileDescriptor>) -> TileSpriteBuilder {
        TileSpriteBuilder {
            seed: 0,
            tiles,
            variant: Variant::Default,
            tile: None,
            team: None,
            adjacent_solid: [false; 8],
        }
    }

    fn set_adjacent(&mut self, place: Adjacencies, to: bool) {
        self.adjacent_solid[place as usize] = to;
    }

    fn set_tile(&mut self, tile: Tile) {
        self.tile = Some(tile)
    }

    fn set_variant(&mut self, variant: Variant) {
        self.variant = variant
    }

    fn set_team(&mut self, team: Team) {
        self.team = Some(team)
    }

    fn resolve(&mut self) -> bool {
        if let Some(team) = self.team {
            self.tiles.retain(|sprite| sprite.is_team(team));
        }
        if let Some(tile) = self.tile {
            self.tiles.retain(|sprite| sprite.is_tile(tile));
        }
        self.tiles
            .retain(|tiles| tiles.test_solid(self.adjacent_solid));

        self.tiles.sort_by(|a, b| b.priority.cmp(&a.priority));
        true
    }

    fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    fn result(&self) -> Option<TileSprite> {
        let tile = self.tiles.first()?;
        let tile = match self.variant {
            Variant::Default => tile.variants.first().copied(),
            Variant::Random => tile
                .variants
                .iter()
                .choose(&mut rand::rngs::StdRng::seed_from_u64(self.seed))
                .copied(),
            Variant::Fixed(index) => tile.variants.get(index as usize).cloned(),
        };
        // info!("picked tile {:?}", tile);
        tile
    }
}

#[derive(serde::Deserialize, serde::Serialize, Asset, Reflect, Clone, Debug)]
struct TileDescriptor {
    name: String,
    priority: i8,
    tile: Tile,
    is_sold: bool,
    team: Team,
    can_be_solid: [bool; 8],
    must_be_solid: [bool; 8],
    variants: Vec<TileSprite>,
}

#[test]
fn ron_out() {
    let all = TileDescriptor::all();
    for tile in all {
        println!(
            "{}",
            ron::ser::to_string_pretty(&tile, ron::ser::PrettyConfig::default()).unwrap()
        );
    }
}

impl TileDescriptor {
    fn all() -> Vec<TileDescriptor> {
        vec![
            TileDescriptor {
                name: "Grass Yellow Up".to_string(),
                priority: 1,
                team: Team::Yellow,
                tile: Tile::Dirt,
                is_sold: true,
                can_be_solid: [true, false, true, true, true, true, true, true],
                must_be_solid: [false, false, false, true, false, false, false, true],
                variants: vec![
                    TileSprite::DirtEmpty,
                    TileSprite::DirtBone,
                    TileSprite::DirtBones,
                ],
            },
            TileDescriptor {
                name: "Dirt".to_string(),
                priority: -1,
                tile: Tile::Dirt,
                team: Team::Any,
                is_sold: true,
                can_be_solid: [true; 8],
                must_be_solid: [false; 8],
                variants: vec![
                    TileSprite::DirtEmpty,
                    TileSprite::DirtBone,
                    TileSprite::DirtBones,
                ],
            },
        ]
    }

    fn is_team(&self, team: Team) -> bool {
        if let Team::Any = self.team {
            return true;
        }
        self.team == team
    }

    fn is_tile(&self, tile: Tile) -> bool {
        self.tile == tile
    }

    fn test_solid(&self, solid: [bool; 8]) -> bool {
        for i in 0..8 {
            if solid[i] {
                if !self.can_be_solid[i] {
                    return false;
                }
            } else if self.must_be_solid[i] {
                return false;
            }
        }
        true
    }
}

struct MapLoader;

impl AssetLoader for MapLoader {
    type Asset = MapData;
    type Error = &'static str;
    type Settings = ();
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        load_map(reader, load_context)
    }
}

async fn load_map<'a>(
    reader: &'a mut bevy::asset::io::Reader<'_>,
    _load_context: &'a mut bevy::asset::LoadContext<'_>,
) -> Result<MapData, &'static str> {
    let mut data = String::new();
    if reader.read_to_string(&mut data).await.is_err() {
        return Err("Failed to read to string");
    };

    let mut blocks = vec![];
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut words = line.split_whitespace();
        match words
            .next()
            .expect("at least one word")
            .to_lowercase()
            .as_str()
        {
            "dirt" => blocks.push(Tile::Dirt.parse(words)?),
            e => {
                error!("{e} is not a valise tile type");
            }
        }
    }

    Ok(MapData { blocks })
}

fn detect_changes(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<MapData>>,
    mut next: ResMut<NextState<MapState>>,
    maps: Query<Entity, With<MapEntities>>,
    current: Res<CurrentMap>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id } = event {
            if *id == current.0.id() {
                next.set(MapState::Spawning);
                for map in &maps {
                    commands.entity(map).despawn_recursive();
                }
            }
        }
    }
}

struct TileDescriptorLoader;

impl AssetLoader for TileDescriptorLoader {
    type Asset = TileDescriptor;
    type Settings = ();
    type Error = &'static str;
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        load_tile_descriptor(reader)
    }
    fn extensions(&self) -> &[&str] {
        &["tile"]
    }
}

async fn load_tile_descriptor<'a>(
    reader: &'a mut bevy::asset::io::Reader<'_>,
) -> Result<TileDescriptor, &'static str> {
    let mut data = String::new();
    if reader.read_to_string(&mut data).await.is_err() {
        return Err("Failed to read to string");
    }
    match ron::from_str(&data) {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("{e}");
            Err("Ron Failed")
        }
    }
}
