use crate::world::bioms::BiomDef;
use crate::world::blocks::BlockDef;
use bevy::asset::Handle;
use bevy::math::{IVec2, Vec3};
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::{Font, IVec3, Resource};
use noisy_bevy::simplex_noise_3d_seeded;

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
        self.name_to_id.get(name).copied().unwrap_or(self.air)
    }
}

#[derive(Resource, Default)]
pub struct BiomRegistry {
    pub defs: Vec<BiomDef>,
    pub name_to_id: HashMap<String, u16>,
    pub plains: u16,
}

impl BiomRegistry {
    pub fn get_or_plains(&self, name: &str) -> u16 {
        self.name_to_id.get(name).copied().unwrap_or(self.plains)
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
    pub(crate) wetness: Vec3,
    pub(crate) continentalness: Vec3,
    pub(crate) erosion: Vec3,
}

impl GenerationNoise {
    pub fn height(&self, cx: i32, cz: i32) -> f32 {
        simplex_noise_3d_seeded(
            Vec3::new(cx as f32 * 0.05, 0.0, cz as f32 * 0.05),
            self.height,
        )
    }

    pub fn temperature(&self, cx: i32, cz: i32) -> f32 {
        simplex_noise_3d_seeded(
            Vec3::new(cx as f32 * 0.05, 0.0, cz as f32 * 0.05),
            self.temperature,
        )
    }

    pub fn wetness(&self, cx: i32, cz: i32) -> f32 {
        simplex_noise_3d_seeded(
            Vec3::new(cx as f32 * 0.05, 0.0, cz as f32 * 0.05),
            self.wetness,
        )
    }

    pub fn continentalness(&self, cx: i32, cz: i32) -> f32 {
        simplex_noise_3d_seeded(
            Vec3::new(cx as f32 * 0.05, 0.0, cz as f32 * 0.05),
            self.continentalness,
        )
    }

    pub fn erosion(&self, cx: i32, cz: i32) {
        simplex_noise_3d_seeded(
            Vec3::new(cx as f32 * 0.05, 0.0, cz as f32 * 0.05),
            self.erosion,
        );
    }
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
