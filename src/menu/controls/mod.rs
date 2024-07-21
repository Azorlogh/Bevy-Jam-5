use bevy::prelude::*;

mod interaction;
mod layout;

pub struct ControlsMenuPlugin;
impl Plugin for ControlsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ButtonState>()
            .insert_resource(ButtonState(None))
            .add_systems(
                Update,
                (
                    interaction::interact_back_button,
                    interaction::interact_action_button,
                    interaction::update_button_text,
                ),
            )
            .add_systems(OnEnter(MenuState::Controls), layout::spawn_menu)
            .add_systems(OnExit(MenuState::Controls), layout::despawn_menu);
    }
}

use crate::input::Action;

use super::MenuState;

#[derive(Component)]
pub struct ControlsMenu;

#[derive(Component)]
pub struct ActionButton(Action);

#[derive(Component)]
pub struct KeyText;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ButtonState(pub Option<Action>);

#[derive(Component)]
pub struct ControlsBack;
