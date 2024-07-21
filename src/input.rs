use bevy::{
    input::{mouse::MouseMotion, InputSystem},
    prelude::*,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct InputSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Inputs>()
            .insert_resource(Inputs::default())
            .add_systems(
                PreUpdate,
                (reset_input, handle_mouse_input, handle_keyboard_input)
                    .chain()
                    .in_set(InputSet)
                    .after(InputSystem),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Resource, Reflect)]
#[reflect(Resource)]
pub struct Inputs {
    pub dir: Vec2,
    pub yaw: f32,
    pub pitch: f32,
    pub jump: bool,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            dir: Vec2::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            jump: false,
        }
    }
}

pub fn reset_input(mut inputs: ResMut<Inputs>) {
    *inputs = Inputs::default();
}

fn handle_mouse_input(
    mut inputs: ResMut<Inputs>,
    _buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let delta = mouse_motion.read().fold(Vec2::ZERO, |acc, x| acc + x.delta);
    inputs.pitch += delta.y * -5e-4;
    inputs.yaw += -delta.x * 5e-4;
}

pub fn handle_keyboard_input(mut inputs: ResMut<Inputs>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyA) {
        inputs.dir += Vec2::new(-1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        inputs.dir += Vec2::new(1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyW) {
        inputs.dir += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        inputs.dir += Vec2::new(0.0, -1.0);
    }

    inputs.dir = inputs.dir.normalize_or_zero();

    if keys.pressed(KeyCode::Space) {
        inputs.jump = true;
    }
}
