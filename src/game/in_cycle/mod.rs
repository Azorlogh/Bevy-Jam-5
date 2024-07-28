use bevy::prelude::*;

use crate::{sandstorm::SandstormIntensity, shelter::PlayerIsSafe, tower::RingBell};

use super::{GameState, GameTime, CYCLE_LENGTH};

pub struct InCyclePlugin;
impl Plugin for InCyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InCycle), enter_cycle)
            .add_systems(
                Update,
                (update_game_time, ring_bell, control_storm, end_cycle)
                    .run_if(in_state(GameState::InCycle)),
            );
    }
}

fn enter_cycle(mut cmds: Commands) {
    cmds.insert_resource(GameTime::default());
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.prev_time = game_time.time;
    game_time.time += time.delta_seconds();
}

fn control_storm(mut storm_intensity: ResMut<SandstormIntensity>, time: Res<GameTime>) {
    storm_intensity.0 = (time.time - CYCLE_LENGTH * 0.5).max(0.0) / (CYCLE_LENGTH / 2.0);
}

fn end_cycle(
    time: Res<GameTime>,
    safe: Res<PlayerIsSafe>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if time.just_passed(CYCLE_LENGTH) {
        if safe.0 {
            next_state.set(GameState::EndCycle);
        } else {
            next_state.set(GameState::Lost);
        }
    }
}

fn ring_bell(time: Res<GameTime>, mut ev_ring: EventWriter<RingBell>) {
    if time.just_passed(CYCLE_LENGTH) {
        // ev_ring.send(RingBell(3));
    } else if time.just_passed(CYCLE_LENGTH * 0.75) {
        ev_ring.send(RingBell(3));
    } else if time.just_passed(CYCLE_LENGTH * 0.50) {
        ev_ring.send(RingBell(1));
    } else if time.just_passed(CYCLE_LENGTH * 0.25) {
        ev_ring.send(RingBell(0));
    }
}
