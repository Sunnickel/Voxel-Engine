use crate::config::{CHUNK_SIZE, MAX_HEIGHT};
use crate::player::Player;
use crate::world::chunks::ChunkCoord;
use bevy::camera::primitives::{Aabb, Frustum};
use bevy::camera::Camera3d;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    Children, Component, MessageReader, Query, Transform, Vec3, Visibility, With, Without,
};

#[derive(Component)]
pub struct LookAngles {
    pub yaw: f32,
    pub pitch: f32,
}

pub fn mouse_look(
    mut motion: MessageReader<MouseMotion>,
    mut players: Query<(&mut Transform, &mut LookAngles, &Children), With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut delta = Vec2::ZERO;

    for ev in motion.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.002;

    for (mut player_tf, mut angles, children) in &mut players {
        angles.yaw -= delta.x * sensitivity;
        angles.pitch -= delta.y * sensitivity;

        angles.pitch = angles.pitch.clamp(-1.54, 1.54);

        player_tf.rotation = Quat::from_rotation_y(angles.yaw);

        for child in children {
            if let Ok(mut cam_tf) = cameras.get_mut(*child) {
                cam_tf.rotation = Quat::from_rotation_x(angles.pitch);
            }
        }
    }
}

pub fn occlusion_culling(
    camera_query: Query<(&Transform, &Frustum), With<Camera3d>>,
    mut chunks: Query<(&ChunkCoord, &mut Visibility)>,
) {
    let Ok((cam_tf, frustum)) = camera_query.single() else {
        return;
    };

    for (coord, mut visibility) in &mut chunks {
        let world_pos = Vec3::new(
            coord.x as f32 * CHUNK_SIZE as f32 + CHUNK_SIZE as f32 / 2.0,
            MAX_HEIGHT as f32 / 2.0,
            coord.z as f32 * CHUNK_SIZE as f32 + CHUNK_SIZE as f32 / 2.0,
        );

        let aabb = Aabb {
            center: world_pos.into(),
            half_extents: Vec3::new(
                CHUNK_SIZE as f32 / 2.0,
                MAX_HEIGHT as f32 / 2.0,
                CHUNK_SIZE as f32 / 2.0,
            )
            .into(),
        };

        let in_frustum = frustum.intersects_obb(&aabb, &bevy::math::Affine3A::IDENTITY, true, true);

        *visibility = if in_frustum {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
