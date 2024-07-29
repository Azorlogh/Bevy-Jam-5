mod fs;

use bevy::prelude::*;
use leafwing_input_manager::{axislike::DualAxis, input_map::InputMap};
use serde::{Deserialize, Serialize};

use crate::input::Action;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_settings)
            .add_systems(Update, save_settings);
    }
}

fn load_settings(mut cmds: Commands) {
    let settings = fs::load_settings();
    cmds.insert_resource(settings.input_map.clone());
}

fn save_settings(input_map: Res<InputMap<Action>>) {
    if input_map.is_changed() {
        fs::save_settings(&Settings {
            input_map: input_map.clone(),
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    input_map: InputMap<Action>,
}

impl Default for Settings {
    fn default() -> Self {
        let mut input_map = InputMap::default();
        input_map.insert(Action::Forward, KeyCode::KeyW);
        input_map.insert(Action::Backward, KeyCode::KeyS);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Jump, KeyCode::Space);
        input_map.insert(Action::Crouch, KeyCode::ControlLeft);
        input_map.insert(Action::Interact, KeyCode::KeyE);
        input_map.insert(Action::PlaceBeacon, KeyCode::KeyR);
        input_map.insert(
            Action::View,
            DualAxis::mouse_motion()
                .with_sensitivity(0.001, 0.001)
                .inverted(),
        );
        Self { input_map }
    }
}
