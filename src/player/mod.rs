use avian3d::prelude::{
    AngularVelocity, ExternalForce, GravityScale, LinearVelocity, RigidBody, Sensor, SpatialQuery,
    SpatialQueryFilter,
};
use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use spawn::PLAYER_HEIGHT;

use crate::{
    camera::{follow::IsControlled, MainCamera},
    input::{Action, Inputs},
    movement::{MovementInput, OnGround},
};

mod beacon;
mod spawn;
pub use spawn::SpawnPlayer;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlayer>()
            .add_systems(Startup, |mut ev: EventWriter<SpawnPlayer>| {
                ev.send(SpawnPlayer(Vec3::Y * 2.0));
            })
            .add_systems(
                Update,
                (
                    spawn::player_spawn,
                    (reset_force, player_float, player_movement, player_jump).chain(),
                    beacon::place_beacon.run_if(action_just_pressed(Action::PlaceBeacon)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    pub batteries: Vec<Entity>,
}

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

fn reset_force(mut q_player: Query<&mut ExternalForce, With<Player>>) {
    for mut force in &mut q_player {
        force.clear();
    }
}

pub fn player_float(
    mut q_player: Query<(
        Entity,
        (&mut LinearVelocity, &AngularVelocity, &mut ExternalForce),
        &Children,
        &GlobalTransform,
    )>,
    spatial: SpatialQuery,
    q_sensor: Query<Entity, (Without<Sensor>, With<RigidBody>)>,
) {
    for (entity, (linvel, angvel, mut force), children, tr) in &mut q_player {
        if let Some(hit) = spatial.cast_ray_predicate(
            tr.translation(),
            -tr.up(),
            PLAYER_HEIGHT / 2.0,
            true,
            SpatialQueryFilter::from_excluded_entities([entity, children[0], children[1]]),
            &|e| q_sensor.contains(e),
        ) {
            let contact = tr.translation() - tr.up() * hit.time_of_impact;
            let vel = velocity_at_point(&linvel, &angvel, tr.translation(), contact);
            let leg_offset = (PLAYER_HEIGHT / 2.0 - hit.time_of_impact).max(0.0);
            let suspension_restitution = leg_offset * 20.0;
            let suspension_damping = -vel.dot(*tr.up()) * 10.0;

            let up_force = tr.up() * (suspension_restitution + suspension_damping);
            force.apply_force(up_force);
        }
    }
}

fn velocity_at_point(
    linvel: &LinearVelocity,
    angvel: &AngularVelocity,
    com: Vec3,
    pt: Vec3,
) -> Vec3 {
    linvel.0 + angvel.0.cross(pt - com)
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
