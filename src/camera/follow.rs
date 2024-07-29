use bevy::{math::DVec3, prelude::*};
use noise::NoiseFn;

use super::{CameraMode, CameraShake, MainCamera};
use crate::input::Inputs;

pub struct FollowCameraPlugin;
impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_control_camera
                    .run_if(|mode: Res<CameraMode>| matches!(*mode, CameraMode::Control(_))),
                camera_follow_position,
                camera_follow_rotation,
                update_post_processing_settings,
            ),
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

pub fn player_control_camera(
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

pub fn update_post_processing_settings(
    mut q_camera: Query<
        (&mut PostProcessSettings, &GlobalTransform),
        (With<MainCamera>, Changed<Transform>),
    >,
) {
    q_camera.iter_mut().for_each(|(mut settings, transform)| {
        const WIND_DIRECTION: Vec3 = Vec3::new(1.0, 0.0, 1.0);
        let forward = transform.forward().as_vec3();
        let wind = WIND_DIRECTION.normalize();
        let xspd = {
            let align = forward.dot(wind);
            let align_factor = align * 3.0;
            if align_factor.abs() < 1.0 {
                align_factor.signum() * 0.5
            } else {
                align_factor
            }
        };
        settings.xspd = xspd;
        settings.yspd = (-1.0 / xspd.abs()).max(-3.0);
    })
}

pub fn camera_follow_position(
    q_target: Query<&GlobalTransform>,
    mut q_camera: Query<(&mut Transform, &CameraShake), With<MainCamera>>,
    time: Res<Time>,
    camera_mode: Res<CameraMode>,
) {
    let target_e = match *camera_mode {
        CameraMode::Control(e) | CameraMode::Follow(e) => e,
        _ => return,
    };
    let Ok(target_tr) = q_target.get(target_e) else {
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

    camera_tr.translation = target_tr.translation() + shake;
}

pub fn camera_follow_rotation(
    q_target: Query<&GlobalTransform>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    camera_mode: Res<CameraMode>,
) {
    let CameraMode::Follow(target_e) = *camera_mode else {
        return;
    };
    let Ok(target_tr) = q_target.get(target_e) else {
        return;
    };
    let mut camera_tr = q_camera.single_mut();

    camera_tr.rotation = target_tr.to_scale_rotation_translation().1;
}
