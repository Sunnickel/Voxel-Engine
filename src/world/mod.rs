pub mod bioms;
pub mod blocks;
pub mod chunks;
pub mod seeding;
pub mod utils;

use crate::config::{BlockRegistry, GenerationNoise, HeightMap, SpawnChunkGenerated, SpawnPoint, SpawnedChunks, CHUNK_SIZE};
use crate::player::movement::movement;
use crate::states::{GamePlugin, GameState};
use crate::world::chunks::{generate_chunk, generate_neighbor_chunks};
use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::Commands;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), (setup_chunk, setup_sky))
            .add_systems(
                Update,
                (generate_neighbor_chunks
                    .after(movement)
                    .run_if(in_state(GameState::InGame)),),
            );
    }
}

pub fn setup_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.0)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 50.0,
            maximum_distance: 200.0,
            ..default()
        }
        .build(),
        GamePlugin::tag(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(8.0).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.6),
            emissive: LinearRgba::new(1.0, 0.9, 0.3, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(200.0, 300.0, -400.0),
    ));
}

pub fn setup_chunk(
    mut commands: Commands,
    mut spawned: ResMut<SpawnedChunks>,
    mut height_map: ResMut<HeightMap>,
    mut gen_noise: ResMut<GenerationNoise>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_point: ResMut<SpawnPoint>,
    mut generated: ResMut<SpawnChunkGenerated>,
    registry: Res<BlockRegistry>,
) {
    let coord = IVec2::new(0, 0);

    generate_chunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut height_map,
        &mut gen_noise,
        &registry,
        coord,
    );

    let center = IVec2::new(CHUNK_SIZE / 2, CHUNK_SIZE / 2);
    let height = height_map.0.get(&center).copied().unwrap_or(5);

    spawn_point.0 = IVec3::new(center.x, height + 5, center.y);

    generated.0 = true;
}
