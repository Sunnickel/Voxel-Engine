use bevy::asset::Handle;
use bevy::math::{IVec2, Vec3};
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::{Font, IVec3, Resource};
use crate::world::blocks::BlockDef;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GlobalAssets {
    pub font: Handle<Font>,
}

#[derive(Resource, Default)]
pub struct BlockRegistry {
    pub defs: Vec<BlockDef>,
    pub name_to_id: HashMap<String, u16>,
    pub air: u16,
}

impl BlockRegistry {
    pub fn get_or_air(&self, name: &str) -> u16 {
        self.name_to_id
            .get(name)
            .copied()
            .unwrap_or(self.air)
    }
}

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub struct Seed(pub(crate) u64);

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub struct SpawnPoint(pub(crate) IVec3);

#[derive(Resource, Default)]
pub struct HeightMap(pub(crate) HashMap<IVec2, i32>);

#[derive(Resource, Default)]
pub struct SpawnedChunks(pub(crate) HashSet<IVec2>);

#[derive(Resource, Default)]
pub struct GenerationNoise {
    pub(crate) height: Vec3,
    pub(crate) temperature: Vec3,
    pub(crate) humidity: Vec3,
}

#[derive(Resource, Default, PartialEq)]
pub struct PlayerSpawned(pub(crate) bool);

#[derive(Resource, Default, PartialEq)]
pub struct WorldReady(pub(crate) bool);

#[derive(Resource, Default, PartialEq)]
pub struct SpawnChunkGenerated(pub(crate) bool);

#[derive(Resource, Default)]
pub struct LastPlayerChunk(pub Option<IVec2>);

pub const CHUNK_SIZE: i32 = 16;
pub const MAX_HEIGHT: i32 = 256;