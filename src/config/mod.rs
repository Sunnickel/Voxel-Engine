use bevy::prelude::{IVec3, Resource};

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub struct Seed(pub(crate) u64);

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub struct SpawnPoint(pub(crate) IVec3);

pub const CHUNK_SIZE: i32 = 16;
