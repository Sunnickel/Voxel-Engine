use crate::player::Player;
use bevy::camera::Camera3d;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{Children, Component, MessageReader, Query, Transform, With, Without};

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
