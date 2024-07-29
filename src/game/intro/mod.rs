use bevy::prelude::*;

use crate::{
    camera::CameraMode,
    menu::styling::{bottom_root, default_text},
    player::Player,
    terrain::TerrainParams,
};

use super::{GameState, SpawnPoint};

const INTRO_TIME: f32 = 10.0;

pub struct IntroPlugin;
impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IntroViewpoint>()
            .add_systems(OnEnter(GameState::Intro), setup_intro)
            .add_systems(OnExit(GameState::Intro), exit_intro)
            .add_systems(Update, update_intro.run_if(in_state(GameState::Intro)));
    }
}

#[derive(Component)]
pub struct IntroScreen;

#[derive(Resource)]
pub struct IntroTime(f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IntroViewpoint;

fn setup_intro(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    q_viewpoint: Query<Entity, With<IntroViewpoint>>,
    mut camera_mode: ResMut<CameraMode>,
) {
    *camera_mode = CameraMode::Follow(q_viewpoint.single());
    cmds.insert_resource(IntroTime(0.0));
    cmds.spawn((bottom_root(), IntroScreen, StateScoped(GameState::Intro)))
        .with_children(|cmds| {
            cmds.spawn(default_text(
                "Find the batteries to power the weather inhibitor.
Once the clock's hand reaches the second quadrant, the wind will rise.
Find shelter before it reaches the top, or you will not survive the storm.",
                64.0,
                &asset_server,
            ));
        });
}

pub fn update_intro(
    time: Res<Time>,
    mut intro_time: ResMut<IntroTime>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    intro_time.0 += time.delta_seconds();
    if intro_time.0 > INTRO_TIME {
        next_state.set(GameState::InCycle);
    }
}

pub fn exit_intro(
    mut camera_mode: ResMut<CameraMode>,
    mut q_player: Query<(Entity, &mut Transform), With<Player>>,
    q_spawn_point: Query<&Transform, (With<SpawnPoint>, Without<Player>)>,
    terrain_params: Res<TerrainParams>,
) {
    let (player_e, mut player_tr) = q_player.single_mut();
    *camera_mode = CameraMode::Control(player_e);

    let xz = q_spawn_point.single().translation.xz();
    let height = terrain_params.get_height(xz) + 4.0;
    player_tr.translation = xz.extend(height).xzy();
}
