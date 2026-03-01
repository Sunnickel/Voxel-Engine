use crate::config::{BlockRegistry, CHUNK_SIZE, MAX_HEIGHT};
use crate::world::chunks::ChunkData;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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

pub fn block_color(registry: &BlockRegistry, id: u16, face_index: usize) -> [f32; 4] {
    let def = &registry.defs[id as usize];

    let tint = match face_index {
        0 => 1.0,
        1 => 0.55,
        _ => 0.75,
    };

    [
        def.color.r * tint,
        def.color.g * tint,
        def.color.b * tint,
        def.color.a,
    ]
}

pub fn is_air(registry: &BlockRegistry, data: &ChunkData, x: i32, y: i32, z: i32) -> bool {
    let s = CHUNK_SIZE as i32;
    let h = MAX_HEIGHT as i32;

    if x < 0 || x >= s || y < 0 || y >= h || z < 0 || z >= s {
        return true;
    }

    data.blocks[x as usize][y as usize][z as usize] == registry.air
}
