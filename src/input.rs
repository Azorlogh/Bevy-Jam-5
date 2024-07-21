use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use leafwing_input_manager::{
    action_state::ActionState, axislike::DualAxis, input_map::InputMap, plugin::InputManagerPlugin,
    Actionlike,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct InputSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .init_resource::<ActionState<Action>>()
            .insert_resource(Inputs::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (update, cursor_grab));
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
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right,
    Move,
    View,
    Jump,
    Crouch,
}

fn setup(mut cmds: Commands) {
    let mut map = InputMap::default();
    map.insert(Action::Forward, KeyCode::KeyW);
    map.insert(Action::Backward, KeyCode::KeyS);
    map.insert(Action::Left, KeyCode::KeyA);
    map.insert(Action::Right, KeyCode::KeyD);
    map.insert(Action::Jump, KeyCode::Space);
    map.insert(Action::Crouch, KeyCode::ControlLeft);
    map.insert(
        Action::View,
        DualAxis::mouse_motion()
            .with_sensitivity(0.001, 0.001)
            .inverted(),
    );
    cmds.insert_resource(map);
}

fn update(
    action: Res<ActionState<Action>>,
    mut inputs: ResMut<Inputs>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    inputs.dir = Vec2::ZERO;
    inputs.view = Vec2::ZERO;

    if q_window
        .get_single()
        .is_ok_and(|win| !matches!(win.cursor.grab_mode, CursorGrabMode::Locked))
    {
        return;
    }

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
}

fn cursor_grab(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_window.single_mut();
    match window.cursor.grab_mode {
        CursorGrabMode::None if buttons.just_pressed(MouseButton::Left) => {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        _ if keys.just_pressed(KeyCode::Escape) => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
        _ => {}
    }
}
