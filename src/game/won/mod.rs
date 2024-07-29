use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_kira_audio::prelude::*;

use crate::{
    menu::styling::{central_panel, default_text, opaque_root, PADDING},
    sandstorm::SandstormIntensity,
};

use super::GameState;

pub struct WonPlugin;
impl Plugin for WonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Won), setup_win)
            .add_systems(Update, interact_restart);
    }
}

#[derive(Component)]
pub struct WonMenu;

#[derive(Component)]
pub struct Restart;

fn setup_win(mut cmds: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    cmds.insert_resource(SandstormIntensity(0.0));
    audio.play(asset_server.load("audio/sfx/cheer.mp3"));
    cmds.spawn((opaque_root(), WonMenu, StateScoped(GameState::Won)))
        .with_children(|cmds| {
            cmds.spawn(central_panel()).with_children(|cmds| {
                cmds.spawn(
                    default_text(
                        "Good job!\nYou have fixed the weather machine!\nThe desert is peaceful once again.\n\n\nThanks for playing! <3",
                        64.0,
                        &asset_server,
                    )
                    .with_style(Style {
                        padding: UiRect::all(Val::Px(PADDING)),
                        ..default()
                    }),
                );
            });
        });
}

pub fn interact_restart(
    mut cmds: Commands,
    q_button: Query<&Interaction, (Changed<Interaction>, With<Restart>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Interaction::Pressed) = q_button.get_single() {
        cmds.add(super::checkpoint::load_checkpoint);
        next_state.set(GameState::InCycle);
    }
}
