use bevy::{
    self,
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use flycam::{FlyCam, FlycamPlugin, MovementSettings};
use follow::{FollowCameraPlugin, IsControlled};

use crate::player::Player;

pub mod flycam;
pub mod follow;
mod spawn;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlycamPlugin, FollowCameraPlugin))
            .register_type::<MovementSettings>()
            .insert_resource(MovementSettings {
                speed: 200.0, // default: 12.0
            })
            .insert_resource(CameraMode::Free)
            .add_systems(Startup, spawn::setup_normal)
            .add_systems(Update, apply_mode)
            .add_systems(
                Update,
                (|mut mode: ResMut<CameraMode>, mut q_player: Query<(Entity, &mut Transform), With<Player>>, q_camera: Query<&Transform, (Without<Player>, With<MainCamera>)>| {
                    println!("mode switch!");
                    *mode = match (*mode, q_player.get_single_mut()) {
                        (CameraMode::Follow(_), _) => CameraMode::Free,
                        (CameraMode::Free, Ok((player_e, mut player_tr))) => {
                            let cam_pos = q_camera.get_single().unwrap().translation;
                            player_tr.translation = cam_pos;
                            CameraMode::Follow(player_e)
                        },
                        (m, _) => m,
                    }
                })
                .run_if(
                    input_pressed(KeyCode::ControlLeft).and_then(input_just_pressed(KeyCode::KeyM)),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, Resource, Reflect)]
pub enum CameraMode {
    Follow(Entity),
    Free,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraRange(pub f32);

#[derive(Component)]
pub struct CameraShake(pub f32);

pub fn apply_mode(
    mut cmds: Commands,
    cam_mode: Res<CameraMode>,
    mut q_camera: Query<Entity, With<MainCamera>>,
    q_controlled: Query<Entity, With<IsControlled>>,
) {
    if cam_mode.is_changed() {
        let Ok(cam_e) = q_camera.get_single_mut() else {
            return;
        };
        match *cam_mode {
            CameraMode::Follow(followed_e) => {
                cmds.entity(followed_e).insert(IsControlled);
                cmds.entity(cam_e)
                    .remove::<FlyCam>()
                    .insert(CameraRange(5.0));
            }
            CameraMode::Free => {
                for e in &q_controlled {
                    cmds.entity(e).remove::<IsControlled>();
                }
                cmds.entity(cam_e).insert(FlyCam).insert(CameraRange(100.0));
            }
        }
    }
}
