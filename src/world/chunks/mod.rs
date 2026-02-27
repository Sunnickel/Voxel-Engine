use crate::config::{Seed, CHUNK_SIZE};
use crate::world::{HeightMap, SpawnedChunks};
use avian3d::prelude::{Collider, ColliderConstructor, ColliderConstructorHierarchy, Friction, RigidBody};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Component, Cuboid, IVec2, Name, Query, ResMut, Transform, Visibility, With};
use noisy_bevy::simplex_noise_3d_seeded;
use crate::player::Player;
use crate::world::blocks::Block;
use crate::world::utils::{player_chunk_coords, world_to_chunk_coords};

#[derive(Component, Debug)]
pub struct Chunk;

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

pub fn generate_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    height_map: &mut HeightMap,
    seed: f32,
    coord: IVec2,
) {
    let cx = coord.x;
    let cz = coord.y;

    commands
        .spawn((
            Name::new("Chunk"),
            Chunk,
            ChunkCoord { x: cx, z: cz },
            Transform::from_xyz((cx * CHUNK_SIZE) as f32, 0.0, (cz * CHUNK_SIZE) as f32),
            Visibility::default(),
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
            RigidBody::Static,
            Collider::cuboid(0.5, 0.5, 0.5),
            Friction::new(0.2),
        ))
        .with_children(|parent| {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let wx = cx * CHUNK_SIZE + x;
                    let wz = cz * CHUNK_SIZE + z;

                    let noise = simplex_noise_3d_seeded(
                        Vec3::new(wx as f32 * 0.1, 0.0, wz as f32 * 0.1),
                        Vec3::new(seed * 0.1, seed * 0.1, seed * 0.1),
                    );

                    let height = (noise * 5.0).floor() as i32;

                    height_map.0.insert(IVec2::new(wx, wz), height);

                    parent.spawn((
                        Name::new("Block"),
                        Block { internal: 0 },
                        Mesh3d(meshes.add(Cuboid::default())),
                        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                        Transform::from_xyz(x as f32, height as f32, z as f32),
                    ));
                }
            }
        });
}


pub fn generate_neighbor_chunks(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut spawned: ResMut<SpawnedChunks>,
    mut height_map: ResMut<HeightMap>,
    seed: ResMut<Seed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_tf = player.single();

    let to_spawn = match player_tf {
        Ok(tf) => {
            let chunk = player_chunk_coords(tf);
            vec![chunk]
        }
        Err(_) => {
            vec![IVec2::ZERO]
        }
    };

    for coord in to_spawn {
        let neighbors: [IVec2; 8] = [
            IVec2::new(coord.x + 1, coord.y),
            IVec2::new(coord.x - 1, coord.y),
            IVec2::new(coord.x, coord.y + 1),
            IVec2::new(coord.x, coord.y - 1),
            IVec2::new(coord.x + 1, coord.y + 1),
            IVec2::new(coord.x - 1, coord.y - 1),
            IVec2::new(coord.x + 1, coord.y - 1),
            IVec2::new(coord.x - 1, coord.y + 1)
        ];

        for n in neighbors {
            if !spawned.0.contains(&n) {
                generate_chunk(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut height_map,
                    seed.0 as f32,
                    n,
                );
                spawned.0.insert(n);
            }
        }
    }
}