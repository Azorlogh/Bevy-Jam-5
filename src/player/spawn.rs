use avian3d::{
    collision::{Collider, CollidingEntities},
    prelude::{
        CoefficientCombine, Friction, GravityScale, LinearVelocity, LockedAxes, Restitution,
        RigidBody,
    },
};
use bevy::prelude::*;

use super::{beacon::BeaconCount, Player};
use crate::{
    camera::follow::Eyes,
    movement::{GroundSensorBundle, MovementInput, OnGround, Speed},
};

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 0.25;
const PLAYER_EYE_OFFSET: f32 = (PLAYER_HEIGHT * 0.92) / 2.0; // relative to center of body

#[derive(Event)]
pub struct SpawnPlayer(pub Vec3);

pub fn player_spawn(mut cmds: Commands, mut ev_spawn_player: EventReader<SpawnPlayer>) {
    for ev in ev_spawn_player.read() {
        cmds.spawn((
            Name::new("Player"),
            Player,
            SpatialBundle::from_transform(Transform::from_translation(ev.0)),
            (
                RigidBody::Dynamic,
                LinearVelocity::default(),
                Collider::capsule(PLAYER_RADIUS, PLAYER_HEIGHT - PLAYER_RADIUS * 2.0),
                LockedAxes::ROTATION_LOCKED,
                CollidingEntities::default(),
                GravityScale(1.0),
                Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                },
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombine::Min,
                },
            ),
            (OnGround(false), MovementInput::default(), Speed(10.0)),
            BeaconCount(10),
        ))
        .with_children(|cmds| {
            cmds.spawn(GroundSensorBundle::new(
                PLAYER_RADIUS * 0.8,
                -PLAYER_HEIGHT / 2.0,
            ));
            cmds.spawn((
                Eyes,
                TransformBundle::from_transform(Transform::from_xyz(0.0, PLAYER_EYE_OFFSET, 0.0)),
            ));
        });
    }
}
