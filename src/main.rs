mod config;
mod player;
mod states;
mod world;

use crate::config::{
    BiomRegistry, BlockRegistry, GenerationNoise, GlobalAssets, HeightMap, LastPlayerChunk,
    PlayerSpawned, Seed, SpawnChunkGenerated, SpawnPoint, SpawnedChunks, WorldReady,
};
use crate::states::{AppLoadingPlugin, GameLoadingPlugin, GamePlugin, GameState, MenuPlugin};
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_lunex::UiLunexPlugins;
use image::{ImageBuffer, Rgb};
use crate::world::bioms::{pick_biome, Climate};
use crate::world::seeding::setup_seed;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Voxel Engine".into(),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::DX12),
                        ..default()
                    }),
                    synchronous_pipeline_compilation: false,
                    ..default()
                })
                .set(LogPlugin {
                    level: (Level::INFO),
                    ..default()
                }),
            UiLunexPlugins,
        ))
        .init_state::<GameState>()
        .init_resource::<GlobalAssets>()
        .init_resource::<SpawnPoint>()
        .init_resource::<HeightMap>()
        .init_resource::<SpawnedChunks>()
        .init_resource::<Seed>()
        .init_resource::<GenerationNoise>()
        .init_resource::<PlayerSpawned>()
        .init_resource::<SpawnChunkGenerated>()
        .init_resource::<LastPlayerChunk>()
        .init_resource::<WorldReady>()
        .init_resource::<BlockRegistry>()
        .init_resource::<BiomRegistry>()
        .add_plugins((AppLoadingPlugin, MenuPlugin, GameLoadingPlugin, GamePlugin))
        .add_systems(
            OnEnter(GameState::InGame),
            debug_noise_image,
        )
        .run();
}

pub fn debug_noise_image(noise: Res<GenerationNoise>, biom_registry: Res<BiomRegistry>) {
    let size = 512u32;
    let scale = 1; // 1 pixel = 1 block

    // Temperature
    let mut img = ImageBuffer::new(size, size);
    for (px, pz, pixel) in img.enumerate_pixels_mut() {
        let x = px as i32 * scale;
        let z = pz as i32 * scale;
        let val = noise.temperature(x, z);
        let t = (val + 1.0) * 0.5;
        let r = ((1.0 - t) * 255.0) as u8;
        let b = (t * 255.0) as u8;
        *pixel = Rgb([r, 0, b]);
    }
    img.save("debug_temperature.png").unwrap();

    // Wetness
    let mut img = ImageBuffer::new(size, size);
    for (px, pz, pixel) in img.enumerate_pixels_mut() {
        let val = noise.wetness(px as i32, pz as i32);
        let t = (val + 1.0) * 0.5;
        let rb = ((1.0 - t) * 255.0) as u8;
        *pixel = Rgb([rb, rb, 255]);
    }
    img.save("debug_wetness.png").unwrap();

    // Biome map (colorized)
    let mut img = ImageBuffer::new(size, size);
    for (px, pz, pixel) in img.enumerate_pixels_mut() {
        let x = px as i32 * scale;
        let z = pz as i32 * scale;
        let climate = Climate {
            temperature: noise.temperature(x, z),
            wetness: noise.wetness(x, z),
            height: noise.height(x, z),
            continentalness: noise.continentalness(x, z),
        };
        let biome = pick_biome(&climate, &biom_registry.defs);
        let tint = &biome.tint;
        *pixel = Rgb([
            tint.r as u8,
            tint.g as u8,
            tint.b as u8,
        ]);
    }
    img.save("debug_biomes.png").unwrap();

    info!("Noise debug images saved");
}
