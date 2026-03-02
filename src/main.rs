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
        .run();
}
