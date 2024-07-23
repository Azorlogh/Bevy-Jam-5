mod monolith;

use bevy::{prelude::*, time::Stopwatch};
use monolith::MonolithPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MonolithPlugin)
            .add_systems(OnEnter(GameState::InGame), enter_game);
    }
}

#[allow(unused)]
#[derive(Resource)]
pub struct GameTime(pub Stopwatch);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    None,
    InGame,
    // CycleEnd,
}

fn enter_game(mut cmds: Commands) {
    cmds.insert_resource(GameTime(Stopwatch::new()));
}
