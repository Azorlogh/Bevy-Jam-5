use bevy::prelude::*;

use crate::menu::styling::{button_bundle, central_panel, default_text, opaque_root, PADDING};

use super::GameState;

pub struct LostPlugin;
impl Plugin for LostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lost), setup_lose)
            .add_systems(Update, interact_restart);
    }
}

#[derive(Component)]
pub struct LoseMenu;

#[derive(Component)]
pub struct Restart;

fn setup_lose(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((opaque_root(), LoseMenu, StateScoped(GameState::Lost)))
        .with_children(|cmds| {
            cmds.spawn(central_panel()).with_children(|cmds| {
                cmds.spawn(
                    default_text("YOU WERE LOST TO THE STORM", 64.0, &asset_server).with_style(
                        Style {
                            padding: UiRect::all(Val::Px(PADDING)),
                            ..default()
                        },
                    ),
                );
                cmds.spawn((button_bundle(), Restart))
                    .with_children(|cmds| {
                        cmds.spawn(default_text(
                            "RESTART AT LAST CHECKPOINT",
                            32.0,
                            &asset_server,
                        ));
                    });
            });
        });
}

pub fn interact_restart(
    q_button: Query<&Interaction, (Changed<Interaction>, With<Restart>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Interaction::Pressed) = q_button.get_single() {
        next_state.set(GameState::InCycle);
    }
}
