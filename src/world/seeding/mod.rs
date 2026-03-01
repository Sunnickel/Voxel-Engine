use crate::config::Seed;
use crate::states::GameState;
use crate::world::GenerationNoise;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{info, OnEnter, ResMut};
use std::hash::{DefaultHasher, Hasher};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), setup_seed);
    }
}

pub fn setup_seed(mut noise: ResMut<GenerationNoise>, seed: ResMut<Seed>) {
    info!("Starting seed generation");
    noise.temperature = seed_to_vec3(seed.0);
    noise.humidity = seed_to_vec3(seed.0.wrapping_add(1));
    noise.height = seed_to_vec3(seed.0.wrapping_add(2));
}

pub fn derive_seed(base: u64, index: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(base);
    hasher.write_u64(index);
    hasher.finish()
}

pub fn seed_to_vec3(base_seed: u64) -> Vec3 {
    let x_seed = derive_seed(base_seed, 0);
    let y_seed = derive_seed(base_seed, 1);
    let z_seed = derive_seed(base_seed, 2);

    let x = (x_seed as f64 / u64::MAX as f64).fract();
    let y = (y_seed as f64 / u64::MAX as f64).fract();
    let z = (z_seed as f64 / u64::MAX as f64).fract();

    Vec3::new(x as f32, y as f32, z as f32)
}
