use bevy::prelude::{Commands, Component, Entity, Query, With, World};
use log::info;

use bevy::prelude::*;
use perlin_noise::PerlinNoise;

pub fn setup_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = PerlinNoise::new();
    commands
        .spawn((
            Chunk,
            Visibility::default(),
            Name::new("Chunk"),
            Transform::default(),
        ))
        .with_children(|parent| {
            for x in 0..3 {
                for z in 0..3 {
                    let scale = 0.1;
                    let amplitude = 5.0;

                    let noise = perlin.get3d([
                        x as f64 * scale,
                        z as f64 * scale,
                        0.0,
                    ]);

                    let height = (noise * amplitude).floor();

                    parent.spawn((
                        Name::new("Block"),
                        Block { internal: 0 },
                        Mesh3d(meshes.add(Cuboid::default())),
                        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                        Transform::from_xyz(
                            x as f32,
                            height as f32,
                            z as f32,
                        ),
                    ));
                }
            }
        });
}

#[derive(Component, Debug)]
pub struct Chunk;

#[derive(Component, Debug)]
pub struct Block {
    pub internal: u8,
}

pub(crate) fn spawned_chunks(query: Query<&Children, With<Chunk>>) {
    for children in query.iter() {
        info!("Chunk has {} children blocks", children.len());
    }
}
