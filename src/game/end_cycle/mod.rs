use bevy::prelude::*;

use crate::{
    menu::styling::{self, default_text, PADDING},
    sandstorm::SandstormIntensity,
};

use super::GameState;

const END_CYCLE_DURATION: f32 = 5.0;

pub struct EndCyclePlugin;
impl Plugin for EndCyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EndCycle), setup_intro)
            .add_systems(
                Update,
                (update_end_cycle, update_root_visibility).run_if(in_state(GameState::EndCycle)),
            );
    }
}

#[derive(Resource)]
pub struct EndCycleTime(f32);

#[derive(Component)]
pub struct EndCycleRoot;

fn setup_intro(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(EndCycleTime(0.0));
    cmds.insert_resource(SandstormIntensity(0.0));
    cmds.spawn((
        styling::opaque_root(),
        StateScoped(GameState::EndCycle),
        EndCycleRoot,
    ))
    .with_children(|cmds| {
        cmds.spawn(
            default_text("YOU SURVIVED THE STORM", 64.0, &asset_server).with_style(Style {
                padding: UiRect::all(Val::Px(PADDING)),
                ..default()
            }),
        );
    });
}

pub fn update_root_visibility(
    mut q_root: Query<&mut BackgroundColor, With<EndCycleRoot>>,
    time: Res<EndCycleTime>,
) {
    q_root.single_mut().0 = Color::BLACK.with_alpha(1.0 - time.0 / END_CYCLE_DURATION);
}

pub fn update_end_cycle(
    time: Res<Time>,
    mut intro_time: ResMut<EndCycleTime>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    intro_time.0 += time.delta_seconds();
    if intro_time.0 > END_CYCLE_DURATION {
        next_state.set(GameState::InCycle);
    }
}
