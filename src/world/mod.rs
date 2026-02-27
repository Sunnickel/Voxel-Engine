pub mod bioms;
pub mod blocks;
pub mod chunks;
pub mod utils;
pub mod seeding;

use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::Commands;
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) use crate::config::Seed;
use crate::config::SpawnPoint;
use crate::player::movement::movement;
use crate::world::chunks::{generate_chunk, generate_neighbor_chunks};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Seed>()
            .init_resource::<SpawnPoint>()
            .init_resource::<HeightMap>()
            .init_resource::<SpawnedChunks>()
            .add_systems(
                PreStartup,
                (setup_chunk, generate_neighbor_chunks.after(setup_chunk)),
            )
            .add_systems(Update, generate_neighbor_chunks.after(movement));
    }
}

#[derive(Resource, Default)]
pub struct HeightMap(HashMap<IVec2, i32>);

#[derive(Resource, Default)]
pub struct SpawnedChunks(HashSet<IVec2>);

#[derive(Resource, Default)]
pub struct GenerationNoise {
    height: Vec3,
    temperature: Vec3,
    humidity: Vec3,
}



pub fn setup_chunk(
    mut commands: Commands,
    mut spawned: ResMut<SpawnedChunks>,
    mut height_map: ResMut<HeightMap>,
    seed: ResMut<Seed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_point: ResMut<SpawnPoint>,
) {
    let coord = IVec2::new(0, 0);

    spawned.0.insert(coord);

    generate_chunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut height_map,
        seed.0 as f32,
        coord,
    );

    let height = height_map.0.get(&coord).unwrap_or(&0);

    spawn_point.0 = IVec3::new(coord.x, *height, coord.y);
}
