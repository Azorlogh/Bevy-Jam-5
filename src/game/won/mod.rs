use bevy::prelude::*;

use crate::{
    menu::styling::{central_panel, default_text, opaque_root, PADDING},
    sandstorm::SandstormIntensity,
};

use super::GameState;

pub struct WonPlugin;
impl Plugin for WonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Won), setup_lose)
            .add_systems(Update, interact_restart);
    }
}

#[derive(Component)]
pub struct WonMenu;

#[derive(Component)]
pub struct Restart;

fn setup_lose(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(SandstormIntensity(0.0));
    cmds.spawn((opaque_root(), WonMenu, StateScoped(GameState::Won)))
        .with_children(|cmds| {
            cmds.spawn(central_panel()).with_children(|cmds| {
                cmds.spawn(
                    default_text(
                        "Good job!\nYou have fixed the weather machine!\nThe desert is peaceful once again.",
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
