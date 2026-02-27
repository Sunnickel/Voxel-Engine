pub mod config;
mod player;
mod world;

use std::hash::{DefaultHasher, Hash, Hasher};
use crate::config::SpawnPoint;
use crate::player::PlayerPlugin;
use crate::world::{Seed, WorldPlugin};
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::{CursorGrabMode, CursorOptions};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Voxel Engine".into(),
                        ..default()
                    }),
                    primary_cursor_options: Option::from(CursorOptions {
                        visible: false,
                        grab_mode: CursorGrabMode::Confined,
                        hit_test: true,
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            WorldPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(_commands: Commands, mut seed: ResMut<Seed>, mut spawn_point: ResMut<SpawnPoint>) {
    seed.0 = string_to_seed("254");
    spawn_point.0 = IVec3::new(8, 0, 8);
}

fn string_to_seed(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(input.as_bytes().as_ptr() as u64);
    hasher.finish()
}