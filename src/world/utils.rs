use crate::config::CHUNK_SIZE;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::Transform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rgba {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
    pub(crate) a: f32,
}

pub fn world_to_chunk_coords(pos: Vec2) -> IVec2 {
    IVec2::new(
        pos.x.div_euclid(CHUNK_SIZE as f32) as i32,
        pos.y.div_euclid(CHUNK_SIZE as f32) as i32,
    )
}

pub fn player_chunk_coords(transform: &Transform) -> IVec2 {
    world_to_chunk_coords(Vec2::new(transform.translation.x, transform.translation.z))
}
