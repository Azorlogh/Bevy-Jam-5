use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::game::{GameTime, CYCLE_LENGTH};

pub struct ClockPlugin;
impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClockHand>()
            .add_systems(Update, (set_hand_initial_rotation, set_clock_time).chain());
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ClockHand;

#[derive(Component)]
pub struct ClockHandInitialRotation(Quat);

fn set_hand_initial_rotation(
    mut cmds: Commands,
    q_added_hands: Query<(Entity, &Transform), Added<ClockHand>>,
) {
    for (e, tr) in &q_added_hands {
        cmds.entity(e).insert(ClockHandInitialRotation(tr.rotation));
    }
}

fn set_clock_time(
    mut q_hand: Query<(&mut Transform, &ClockHandInitialRotation), With<ClockHand>>,
    time: Res<GameTime>,
) {
    for (mut tr, initial_rotation) in &mut q_hand {
        tr.rotation = initial_rotation.0 * Quat::from_rotation_z(-time.time / CYCLE_LENGTH * TAU);
    }
}
