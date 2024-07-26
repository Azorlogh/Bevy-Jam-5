use bevy::{math::DVec3, prelude::*};
use noise::NoiseFn;

use super::{CameraMode, CameraShake, MainCamera};
use crate::input::Inputs;

pub struct FollowCameraPlugin;
impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_camera, camera_follow_eyes)
                .run_if(|mode: Res<CameraMode>| matches!(*mode, CameraMode::Follow(_))),
        );
    }
}

/// This component is added as a child of the player entity, so that it follows the player's transform incl. scaling
#[derive(Component)]
pub struct Eyes;

#[derive(Component)]
pub struct IsControlled;

#[derive(Default, Component)]
pub struct CameraAngles {
    pub yaw: f32,
    pub pitch: f32,
}

pub fn player_camera(
    inputs: Res<Inputs>,
    mut q_camera: Query<(&mut CameraAngles, &mut Transform), With<MainCamera>>,
) {
    for (mut camera_angles, mut camera_tr) in &mut q_camera {
        camera_angles.yaw += inputs.view.x;
        camera_angles.pitch += inputs.view.y;
        camera_tr.rotation =
            Quat::from_rotation_y(camera_angles.yaw) * Quat::from_rotation_x(camera_angles.pitch);
    }
}

pub fn camera_follow_eyes(
    q_player_eyes: Query<&GlobalTransform, With<Eyes>>,
    mut q_camera: Query<(&mut Transform, &CameraShake), With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok(eyes_tr) = q_player_eyes.get_single() else {
        return;
    };
    let (mut camera_tr, shake) = q_camera.single_mut();

    let t = time.elapsed_seconds() as f64 * 10.0;
    let shake = DVec3::new(
        noise::Perlin::new(0).get([t, 0.0]),
        noise::Perlin::new(0).get([t, 10.0]),
        noise::Perlin::new(0).get([t, 20.0]),
    )
    .as_vec3()
        * 0.4
        * shake.0;

    camera_tr.translation = eyes_tr.translation() + shake;
}
