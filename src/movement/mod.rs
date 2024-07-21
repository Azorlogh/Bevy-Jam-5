use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

mod ground;
pub use ground::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<MovementInput>()
            .register_type::<OnGround>()
            .add_systems(Update, (movement, detect_ground));
    }
}

/// Movement speed of the agent
#[derive(Component, Reflect)]
pub struct Speed(pub f32);

/// Direction the agent wants to go in
#[derive(Default, Component, Reflect)]
pub struct MovementInput(pub Vec2);

fn movement(
    time: Res<Time>,
    mut q_agent: Query<(&mut LinearVelocity, &OnGround, &MovementInput, &Speed)>,
) {
    for (mut linvel, on_ground, input, speed) in &mut q_agent {
        let friction = match on_ground.0 {
            true => 64.0,
            false => 1.0,
        };

        let interp_t = 1.0 - (-friction * time.delta_seconds()).exp();

        let current_vel = linvel.xz();

        let dir = input.0;

        let lacking = speed.0 - current_vel.dot(dir);
        linvel.0 += dir.extend(0.0).xzy() * lacking * interp_t;

        let extra = current_vel.dot(dir.perp());
        linvel.0 -= dir.perp().extend(0.0).xzy() * extra * interp_t;

        if dir == Vec2::ZERO {
            let prev_linvel = linvel.0;
            linvel.0 += -prev_linvel.xz().extend(0.0).xzy() * interp_t;
        }
    }
}
