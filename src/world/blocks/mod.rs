use crate::config::{BlockRegistry, CHUNK_SIZE, MAX_HEIGHT};
use crate::world::chunks::ChunkData;
use crate::world::utils::Rgba;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Component, Debug, Clone)]
pub struct Block {
    pub id: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockDef {
    pub id: String,
    pub display_name: String,
    pub solid: bool,
    pub hardness: f32,
    pub color: Rgba,
}

pub fn load_blocks(commands: &mut Commands) {
    info!("Registering blocks");

    let mut registry = BlockRegistry::default();

    let dir = Path::new("assets/blocks");

    let entries = fs::read_dir(dir).expect("blocks folder missing");

    for entry in entries {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) != Some("ron") {
            continue;
        }

        let text = fs::read_to_string(&path).expect("failed reading block ron");

        let def: BlockDef = ron::from_str(&text).expect("invalid block ron");

        let id = registry.defs.len() as u16;

        registry.name_to_id.insert(def.id.clone(), id);
        registry.defs.push(def);
    }

    registry.air = registry.get_or_air("air");

    commands.insert_resource(registry);
}

pub const FACES: [([i32; 3], [f32; 3], [[f32; 3]; 4]); 6] = [
    // top
    (
        [0, 1, 0],
        [0.0, 1.0, 0.0],
        [
            [0.0, 1.0, 1.0], // was [0.0, 1.0, 0.0]
            [1.0, 1.0, 1.0], // was [1.0, 1.0, 0.0]
            [1.0, 1.0, 0.0], // was [1.0, 1.0, 1.0]
            [0.0, 1.0, 0.0], // was [0.0, 1.0, 1.0]
        ],
    ),
    // bottom
    (
        [0, -1, 0],
        [0.0, -1.0, 0.0],
        [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    ),
    // +x (right)
    (
        [1, 0, 0],
        [1.0, 0.0, 0.0],
        [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
    ),
    // -x (left)
    (
        [-1, 0, 0],
        [-1.0, 0.0, 0.0],
        [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    ),
    // +z (front)
    (
        [0, 0, 1],
        [0.0, 0.0, 1.0],
        [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
    ),
    // -z (back)
    (
        [0, 0, -1],
        [0.0, 0.0, -1.0],
        [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    ),
];

pub fn block_color(registry: &BlockRegistry, id: u16) -> [f32; 4] {
    let def = &registry.defs[id as usize];

    [def.color.r, def.color.g, def.color.b, def.color.a]
}

pub fn is_air(registry: &BlockRegistry, data: &ChunkData, x: i32, y: i32, z: i32) -> bool {
    let s = CHUNK_SIZE as i32;
    let h = MAX_HEIGHT as i32;

    if x < 0 || x >= s || y < 0 || y >= h || z < 0 || z >= s {
        return true;
    }

    data.blocks[x as usize][y as usize][z as usize] == registry.air
}
