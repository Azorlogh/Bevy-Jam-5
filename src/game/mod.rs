mod checkpoint;
mod end_cycle;
mod in_cycle;
mod intro;
mod lost;
mod monolith;

use bevy::prelude::*;
use checkpoint::CheckpointPlugin;
use end_cycle::EndCyclePlugin;
use in_cycle::InCyclePlugin;
use intro::{IntroPlugin, IntroViewpoint};
use monolith::MonolithPlugin;

use crate::util::switch_to_state;

pub const CYCLE_LENGTH: f32 = 60.0 * 5.0;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameTime>()
            .register_type::<SpawnPoint>()
            .add_plugins((
                IntroPlugin,
                InCyclePlugin,
                EndCyclePlugin,
                CheckpointPlugin,
                MonolithPlugin,
            ))
            .init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            // switch to intro when the scene is loaded (there's definitely a better way to do this)
            .add_systems(
                Update,
                switch_to_state(GameState::Intro).run_if(in_state(GameState::None).and_then(
                    |q_added_intro: Query<(), Added<IntroViewpoint>>| !q_added_intro.is_empty(),
                )),
            );
    }
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameTime {
    pub time: f32,
    pub prev_time: f32,
}

impl GameTime {
    pub fn just_passed(&self, t: f32) -> bool {
        self.time >= t && self.prev_time < t
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    None,
    Intro,
    InCycle,
    EndCycle,
    Lost,
    _Won,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpawnPoint;
