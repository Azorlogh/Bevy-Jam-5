mod monolith;

use bevy::{prelude::*, time::Stopwatch};
use monolith::MonolithPlugin;

use crate::{sandstorm::SandstormIntensity, tower::RingBell};

#[allow(unused)]
const CYCLE_LENGTH: f32 = 10.0;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameTime>()
            .add_plugins(MonolithPlugin)
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::InGame), enter_game)
            .add_systems(
                Update,
                (update_game_time, ring_bell).run_if(in_state(GameState::InGame)),
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
    None,
    #[default]
    InGame,
    // CycleEnd,
}

fn enter_game(mut cmds: Commands) {
    cmds.insert_resource(GameTime::default());
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.prev_time = game_time.time;
    game_time.time += time.delta_seconds();
}

#[allow(unused)]
fn control_storm(mut storm_intensity: ResMut<SandstormIntensity>, time: Res<GameTime>) {}

fn ring_bell(time: Res<GameTime>, mut ev_ring: EventWriter<RingBell>) {
    if time.just_passed(30.0) {
        println!("ring 3!");
        ev_ring.send(RingBell(3));
    } else if time.just_passed(20.0) {
        println!("ring 2!");
        ev_ring.send(RingBell(2));
    } else if time.just_passed(10.0) {
        println!("ring 1!");
        ev_ring.send(RingBell(1));
    } else if time.prev_time == 0.0 {
        println!("ring 0!");
        ev_ring.send(RingBell(0));
    }
}
