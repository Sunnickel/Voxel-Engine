use crate::config::Seed;
use crate::states::GameState;
use crate::world::GenerationNoise;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{info, OnEnter, ResMut};

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), setup_seed);
    }
}

pub fn setup_seed(mut noise: ResMut<GenerationNoise>, seed: ResMut<Seed>) {
    info!("Starting seed generation");
    noise.temperature = seed_to_vec3(seed.0);
    noise.wetness = seed_to_vec3(seed.0.wrapping_add(1));
    noise.height = seed_to_vec3(seed.0.wrapping_add(2));
    noise.continentalness = seed_to_vec3(seed.0.wrapping_add(3));
    noise.erosion = seed_to_vec3(seed.0.wrapping_add(4));
}

pub fn derive_seed(base: u64, index: u64) -> u64 {
    let mut x = base ^ index.wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
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
