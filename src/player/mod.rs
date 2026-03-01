pub mod camera;
pub mod movement;

use crate::config::WorldReady;
pub(crate) use crate::config::{PlayerSpawned, SpawnPoint};
use crate::player::camera::{mouse_look, LookAngles};
use crate::player::movement::{
    apply_gravity, apply_movement_damping, keyboard_input, kinematic_controller_collisions,
    movement, update_grounded, CharacterControllerBundle, MovementAction,
};
use crate::states::{GamePlugin, GameState};
use crate::world::setup_chunk;
use avian3d::math::{Scalar, Vector};
use avian3d::prelude::{Collider, NarrowPhaseSystems, PhysicsSchedule};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>()
            .add_systems(
                Update,
                spawn_player
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_equals(WorldReady(true)))
                    .run_if(resource_equals(PlayerSpawned(false))),
            )
            .add_systems(
                Update,
                (
                    keyboard_input,
                    update_grounded,
                    apply_gravity,
                    movement,
                    apply_movement_damping,
                    mouse_look,
                )
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_equals(WorldReady(true)))
                    .run_if(resource_equals(PlayerSpawned(true))),
            )
            .add_systems(
                PhysicsSchedule,
                kinematic_controller_collisions
                    .in_set(NarrowPhaseSystems::Last)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    spawn_point: ResMut<SpawnPoint>,
    mut player_spawned: ResMut<PlayerSpawned>,
) {
    let [x, y, z] = spawn_point.0.to_array();

    commands
        .spawn((
            Player,
            GamePlugin::tag(),
            LookAngles {
                yaw: 0.0,
                pitch: 0.0,
            },
            Transform::from_xyz(x as f32, y as f32, z as f32),
            Visibility::default(),
            CharacterControllerBundle::new(
                Collider::cuboid(1.0, 2.0, 1.0),
                Vec3::NEG_Y * 9.81 * 2.0,
            )
            .with_movement(50.0, 0.92, 7.0, 30f32.to_radians()),
        ))
        .with_children(|p| {
            p.spawn((
                Camera3d::default(),
                Transform::from_xyz(default(), 0.5, default()),
            ));
        });

    info!("Spawned at: x={:?} y={} z={}", x, y, z);

    player_spawned.0 = true;
    info!("Player spawned");
}
