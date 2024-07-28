use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use controls::ControlsMenuPlugin;
use styling::MenuStylingPlugin;

use crate::{input::cursor_is_grabbed, util::switch_to_state};

mod controls;

#[allow(unused)]
pub mod styling;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuStylingPlugin, ControlsMenuPlugin))
            .add_systems(
                Update,
                (
                    switch_to_state(MenuState::Controls).run_if(in_state(MenuState::None)),
                    switch_to_state(MenuState::None).run_if(in_state(MenuState::Controls)),
                )
                    .run_if(input_just_pressed(KeyCode::Escape).and_then(not(cursor_is_grabbed))),
            )
            .init_state::<MenuState>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum MenuState {
    #[default]
    None,
    Controls,
}
