use avian3d::prelude::{GravityScale, LinearVelocity};
use bevy::prelude::*;

use crate::{
    camera::{follow::IsControlled, MainCamera},
    input::Inputs,
    movement::{MovementInput, OnGround},
};

mod spawn;
pub use spawn::SpawnPlayer;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlayer>()
            .add_systems(Startup, |mut ev: EventWriter<SpawnPlayer>| {
                ev.send(SpawnPlayer(Vec3::Y * 2.0));
            })
            .add_systems(Update, (spawn::player_spawn, player_movement, player_jump));
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_movement(
    inputs: Res<Inputs>,
    mut q_player: Query<&mut MovementInput, (With<Player>, With<IsControlled>)>,
    q_camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    for mut movement_input in &mut q_player {
        let camera_tr = q_camera.single();

        let camera_forward = (*camera_tr.forward() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
        let camera_right = (*camera_tr.right() * Vec3::new(1.0, 0.0, 1.0)).normalize_or_zero();
        let dir = (camera_forward * inputs.dir.y + camera_right * inputs.dir.x).xz();

        movement_input.0 = dir;
    }
}

pub fn player_jump(
    inputs: Res<Inputs>,
    mut q_player: Query<
        (&mut LinearVelocity, &mut GravityScale, &OnGround),
        (With<Player>, With<IsControlled>),
    >,
    mut falling: Local<bool>,
) {
    for (mut linvel, mut gravity, on_ground) in &mut q_player {
        if on_ground.0 && inputs.jump {
            linvel.y = 2.7;
            gravity.0 = 0.5;
            *falling = false;
        } else if !on_ground.0 && !*falling && !inputs.jump {
            gravity.0 = 1.0;
            *falling = true;
        }
    }
}
