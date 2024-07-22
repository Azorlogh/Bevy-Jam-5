use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use leafwing_input_manager::{
    action_state::ActionState,
    plugin::{InputManagerPlugin, ToggleActions},
    Actionlike,
};
use serde::{Deserialize, Serialize};

use crate::menu::MenuState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct InputSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(ToggleActions::<Action>::ENABLED)
            .insert_resource(Inputs::default())
            .add_systems(Update, update.run_if(in_state(MenuState::None)))
            .add_systems(PostUpdate, cursor_grab.run_if(in_state(MenuState::None)));
    }
}

#[derive(Resource, Default)]
pub struct Inputs {
    /// Direction the player is moving in
    pub dir: Vec2,
    /// Controls the camera rotation
    pub view: Vec2,
    pub jump: bool,
    pub crouch: bool,
    pub place: bool,
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Move,
    View,
    Jump,
    Crouch,
    Place,
}

fn update(action: Res<ActionState<Action>>, mut inputs: ResMut<Inputs>) {
    inputs.dir = Vec2::ZERO;
    inputs.view = Vec2::ZERO;

    if action.pressed(&Action::Forward) {
        inputs.dir += Vec2::Y;
    }
    if action.pressed(&Action::Backward) {
        inputs.dir += -Vec2::Y;
    }
    if action.pressed(&Action::Left) {
        inputs.dir -= Vec2::X;
    }
    if action.pressed(&Action::Right) {
        inputs.dir += Vec2::X;
    }

    inputs.view = action.clamped_axis_pair(&Action::View).unwrap().xy();

    inputs.jump = action.pressed(&Action::Jump);
    inputs.crouch = action.pressed(&Action::Crouch);
    inputs.place = action.just_pressed(&Action::Place);
}

fn cursor_grab(
    mut ctx: EguiContexts,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle_actions: ResMut<ToggleActions<Action>>,
) {
    let mut window = q_window.single_mut();
    match window.cursor.grab_mode {
        CursorGrabMode::None
            if buttons.just_pressed(MouseButton::Left) && !ctx.ctx_mut().is_pointer_over_area() =>
        {
            toggle_actions.enabled = true;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        _ if keys.just_pressed(KeyCode::Escape) => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            toggle_actions.enabled = false;
        }
        _ => {}
    }
}

pub fn cursor_is_grabbed(q_window: Query<&Window, With<PrimaryWindow>>) -> bool {
    q_window
        .get_single()
        .is_ok_and(|window| matches!(window.cursor.grab_mode, CursorGrabMode::Locked))
}
