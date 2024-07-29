use bevy::prelude::*;
use leafwing_input_manager::{input_map::InputMap, user_input::UserInput};

use crate::input::Action;

use super::{ActionButton, ButtonState, ControlsBack, KeyText, MenuState};

pub fn interact_action_button(
    mut q_button: Query<(&Interaction, &ActionButton), Changed<Interaction>>,
    mut map: ResMut<InputMap<Action>>,
    mut button_state: ResMut<ButtonState>,

    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    match &mut button_state.0 {
        Some(action) => {
            let mut bind: Option<UserInput> = None;
            for k in keys.get_just_pressed() {
                bind = Some((*k).into());
            }

            for b in buttons.get_just_pressed() {
                bind = Some((*b).into());
            }

            if let Some(bind) = bind {
                map.clear_action(action);
                map.insert(*action, bind);
                button_state.0 = None;
            }
        }
        _ => {
            for (interaction, action) in &mut q_button {
                match *interaction {
                    Interaction::Pressed => {
                        // change user action
                        button_state.0 = Some(action.0);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn update_button_text(
    q_action_button: Query<&ActionButton>,
    mut q_text: Query<(&mut Text, &Parent), With<KeyText>>,
    button_state: Res<ButtonState>,
    map: Res<InputMap<Action>>,
) {
    for (mut text, parent) in &mut q_text {
        let action_btn = q_action_button.get(parent.get()).unwrap();
        match button_state.0 {
            Some(action) if action_btn.0 == action => {
                text.sections[0].value = String::from("???");
            }
            _ => {
                let binding = map
                    .get(&action_btn.0)
                    .map(|inputs| format!("{}", inputs[0]))
                    .unwrap_or_default();
                text.sections[0].value = binding;

                text.sections[0].style.color = Color::srgb(0.91, 0.83, 0.49);
            }
        }
    }
}

pub fn interact_back_button(
    mut q_button: Query<&Interaction, (Changed<Interaction>, With<ControlsBack>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if let Ok(interaction) = q_button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => menu_state.set(MenuState::None),
            _ => {}
        }
    }
}
