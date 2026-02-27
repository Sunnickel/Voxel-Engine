use std::hash::{DefaultHasher, Hasher};
use bevy::math::{vec3, Vec3};
use bevy::prelude::ResMut;
use crate::config::Seed;
use crate::world::GenerationNoise;

pub fn setup_seed(mut noise: ResMut<GenerationNoise>, seed: ResMut<Seed>) {


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

    // Map seeds to [0,1) floats (or any range you like)
    let x = (x_seed as f64 / u64::MAX as f64).fract();
    let y = (y_seed as f64 / u64::MAX as f64).fract();
    let z = (z_seed as f64 / u64::MAX as f64).fract();

    Vec3::new(x as f32, y as f32, z as f32)
}

pub fn derive_vec3_seed(base_seed: u64) -> Vec3 {
    let mut s = base_seed as f32;
    vec3(s, s + 1.0, s + 2.0)
}