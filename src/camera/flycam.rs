use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::input::Inputs;

/// Movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self { speed: 120. }
    }
}

/// Marks an entity as a fly camera controlled by user inputs.
#[derive(Component)]
pub struct FlyCam;

/// Handles movement
fn player_move(
    inputs: Res<Inputs>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    for mut transform in query.iter_mut() {
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        let mut velocity = (forward * inputs.dir.y + right * inputs.dir.x).normalize_or_zero();

        if inputs.jump {
            velocity.y += 1.0;
        }
        if inputs.crouch {
            velocity.y -= 1.0;
        }

        transform.translation += velocity * time.delta_seconds() * settings.speed;
    }
}

/// Handles looking around
fn player_look(
    inputs: Res<Inputs>,
    mut query: Query<&mut Transform, With<FlyCam>>,
    mut view: Local<Vec2>,
) {
    for mut transform in query.iter_mut() {
        *view = *view + inputs.view;
        view.y = view.y.clamp(-TAU / 4.0, TAU / 4.0);

        // Order is important to prevent unintended roll
        transform.rotation = Quat::from_rotation_y(view.x) * Quat::from_rotation_x(view.y);
    }
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct FlycamPlugin;
impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .add_systems(Update, (player_move, player_look));
    }
}
