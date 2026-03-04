use crate::config::{
    BiomRegistry, BlockRegistry, LastPlayerChunk, CHUNK_SIZE, MAX_HEIGHT,
};
use crate::player::Player;
use crate::states::GamePlugin;
use crate::world::bioms::{pick_biome, BiomDef, Climate};
use crate::world::blocks::{block_color, is_air, FACES};
use crate::world::utils::player_chunk_coords;
use crate::world::{GenerationNoise, HeightMap, SpawnedChunks};
use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::math::Vec3;
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d};
use bevy::prelude::{info, Color, Commands, Component, Entity, EntityCommands, IVec2, Name, Query, Res, ResMut, StandardMaterial, Transform, Visibility, With};
use noisy_bevy::simplex_noise_3d_seeded;

#[derive(Component, Debug)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkData {
    pub blocks: Box<[[[u16; CHUNK_SIZE as usize]; MAX_HEIGHT as usize]; CHUNK_SIZE as usize]>,
}

impl ChunkData {
    pub fn empty(air: u16) -> Self {
        ChunkData {
            blocks: Box::new(
                [[[air; CHUNK_SIZE as usize]; MAX_HEIGHT as usize]; CHUNK_SIZE as usize],
            ),
        }
    }
}

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}


fn terrain_block(
    wx: i32,
    wy: i32,
    surface_height: i32,
    noise: &GenerationNoise,
    block_registry: &BlockRegistry,
    biom_registry: &BiomRegistry,
    biome: &BiomDef,
) -> u16 {
    if wy > surface_height {
        if wy <= 0 {
            return block_registry.get_or_air("water");
        }
        return block_registry.air;
    }

    if wy == 0 {
        return block_registry.get_or_air("bedrock");
    }

    let cave_val = simplex_noise_3d_seeded(
        Vec3::new(wx as f32 * 0.05, wy as f32 * 0.1, 0.0),
        noise.height + 42.0,
    );

    if wy > 1 && wy < surface_height && cave_val > 0.65 {
        return block_registry.air;
    }

    if wy == surface_height {
        if surface_height <= 0 {
            biome.underwater
        } else {
            biome.surface
        }
    } else if wy >= surface_height - biome.top_layer_threshold {
        if cave_val > 0.6 {
            block_registry.get_or_air("gravel")
        } else {
            biome.underground
        }
    } else {
        block_registry.get_or_air("stone")
    }
}

fn generate_block_data(
    cx: i32,
    cz: i32,
    noise: &GenerationNoise,
    height_map: &mut HeightMap,
    _climate: &Climate,
    block_registry: &BlockRegistry,
    biom_registry: &BiomRegistry,
) -> ChunkData {
    let mut data = ChunkData::empty(block_registry.air);

    for x in 0..CHUNK_SIZE as usize {
        for z in 0..CHUNK_SIZE as usize {
            let wx = cx * CHUNK_SIZE + x as i32;
            let wz = cz * CHUNK_SIZE + z as i32;

            let n1 = simplex_noise_3d_seeded(
                Vec3::new(wx as f32 * 0.05, 0.0, wz as f32 * 0.05),
                noise.height,
            );
            let n2 = simplex_noise_3d_seeded(
                Vec3::new(wx as f32 * 0.02, 0.0, wz as f32 * 0.02),
                noise.height + 1.0,
            );

            let col_climate = Climate {
                temperature: noise.temperature(wx, wz),
                wetness: noise.wetness(wx, wz),
                height: noise.height(wx, wz),
                continentalness: noise.continentalness(wx, wz),
            };
            let biome = pick_biome(&col_climate, &biom_registry.defs);

            let surface = (8.0 + n1 * 4.0 + n2 * 6.0).floor() as i32;
            let surface = surface.clamp(2, MAX_HEIGHT - 1);

            height_map.0.insert(IVec2::new(wx, wz), surface);

            for y in 0..MAX_HEIGHT as usize {
                data.blocks[x][y][z] = terrain_block(
                    wx,
                    y as i32,
                    surface,
                    noise,
                    block_registry,
                    biom_registry,
                    biome,
                );
            }
        }
    }

    data
}

fn build_chunk_mesh(data: &ChunkData, block_registry: &BlockRegistry) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for x in 0..CHUNK_SIZE as usize {
        for y in 0..MAX_HEIGHT as usize {
            for z in 0..CHUNK_SIZE as usize {
                let id = data.blocks[x][y][z];
                if id == block_registry.air {
                    continue;
                }

                for (face_idx, (dir, normal, face_verts)) in FACES.iter().enumerate() {
                    let nx = x as i32 + dir[0];
                    let ny = y as i32 + dir[1];
                    let nz = z as i32 + dir[2];

                    if !is_air(block_registry, data, nx, ny, nz) {
                        continue;
                    }

                    let base = positions.len() as u32;
                    let color = block_color(block_registry, id, face_idx);

                    for v in face_verts.iter() {
                        positions.push([x as f32 + v[0], y as f32 + v[1], z as f32 + v[2]]);
                        normals.push(*normal);
                        colors.push(color);
                    }

                    indices.extend_from_slice(&[
                        base,
                        base + 1,
                        base + 2,
                        base,
                        base + 2,
                        base + 3,
                    ]);
                }
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn generate_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    height_map: &mut HeightMap,
    generation_noise: &mut GenerationNoise,
    block_register: &BlockRegistry,
    biom_registry: &BiomRegistry,
    coord: IVec2,
) -> Entity {
    let cx = coord.x;
    let cz = coord.y;

    let climate = Climate {
        temperature: generation_noise.temperature(cx, cz),
        wetness: generation_noise.wetness(cx, cz),
        height: generation_noise.height(cx, cz),
        continentalness: generation_noise.continentalness(cx, cz),
    };

    let data = generate_block_data(
        cx,
        cz,
        generation_noise,
        height_map,
        &climate,
        block_register,
        biom_registry,
    );
    let mesh = build_chunk_mesh(&data, block_register);

    let collider = build_collider_from_mesh(&mesh);

    let material = StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..Default::default()
    };

    let mut entity = commands.spawn((
        Name::new(format!("Chunk ({cx},{cz})")),
        Chunk,
        ChunkCoord { x: cx, z: cz },
        data,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(material)),
        Transform::from_xyz((cx * CHUNK_SIZE) as f32, 0.0, (cz * CHUNK_SIZE) as f32),
        Visibility::default(),
        RigidBody::Static,
        Friction::new(0.5),
        GamePlugin::tag(),
    ));

    if let Some(collider) = collider {
        entity.insert(collider);
    }

    entity.id()
}

fn build_collider_from_mesh(mesh: &Mesh) -> Option<Collider> {
    use bevy::mesh::VertexAttributeValues;

    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let vertices: Vec<avian3d::math::Vector> = match positions {
        VertexAttributeValues::Float32x3(verts) => verts
            .iter()
            .map(|v| avian3d::math::Vector::new(v[0] as _, v[1] as _, v[2] as _))
            .collect(),
        _ => return None,
    };

    if vertices.is_empty() {
        return None;
    }

    let indices = mesh.indices()?;
    let triangles: Vec<[u32; 3]> = match indices {
        Indices::U32(idx) => idx
            .chunks_exact(3)
            // Flip winding: swap index 1 and 2
            .map(|c| [c[0], c[2], c[1]])
            .collect(),
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|c| [c[0] as u32, c[2] as u32, c[1] as u32])
            .collect(),
    };

    if triangles.is_empty() {
        return None;
    }

    Some(Collider::trimesh(vertices, triangles))
}

pub fn generate_neighbor_chunks(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut last_chunk: ResMut<LastPlayerChunk>,
    mut spawned: ResMut<SpawnedChunks>,
    mut height_map: ResMut<HeightMap>,
    mut generation_noise: ResMut<GenerationNoise>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_registry: Res<BlockRegistry>,
    biom_registry: Res<BiomRegistry>,
) {
    let tf = match player.single() {
        Ok(tf) => tf,
        Err(_) => return,
    };

    let current_chunk = player_chunk_coords(tf);
    if last_chunk.0 == Some(current_chunk) {
        return;
    }
    last_chunk.0 = Some(current_chunk);

    let radius: i32 = 8;
    let unload_radius: i32 = radius + 2;

    spawned.0.retain(|&coord, &mut entity| {
        let dx = coord.x - current_chunk.x;
        let dy = coord.y - current_chunk.y;
        let outside = dx * dx + dy * dy > unload_radius * unload_radius;

        if outside {
            commands.entity(entity).despawn();
        }

        !outside
    });

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx * dx + dy * dy > radius * radius {
                continue;
            }

            let chunk_coord = IVec2::new(current_chunk.x + dx, current_chunk.y + dy);

            if !spawned.0.contains_key(&chunk_coord) {
                let entity = generate_chunk(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut height_map,
                    &mut generation_noise,
                    &block_registry,
                    &biom_registry,
                    chunk_coord,
                );
                spawned.0.insert(chunk_coord, entity);
            }
        }
    }
}