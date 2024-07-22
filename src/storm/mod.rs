use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

pub struct SandstormPlugin;

impl Plugin for SandstormPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sandstorm).add_systems(
            Update,
            (move_sandstorm_up, move_sandstorm_circle, move_center),
        );
    }
}

/// marker for all sandstorm particles
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct SandstormParticle;

/// height at which sandstorm particle cycles around center
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct StormHeight(f32);

/// center of sandstorm particle
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct StormCenter(Vec3);

/// the particles need to stay afloat and get a fixed velocity upwards after falling under their
/// height. This value determines this fixed velocity
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct StormUpVelocity(f32);

fn spawn_sandstorm(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    (0..2500).for_each(|_| {
        let height = rng.gen_range(1.0..=15.0);

        // position in radians
        let pos_rad = rng.gen_range(0.0..=std::f32::consts::TAU);

        let fixed_velocity = rng.gen_range(5.0..=15.0);

        let color_percent = rng.gen_range(0.0..=1.0);
        let color = Color::srgb_u8(
            (228.0 * color_percent + 150.0 * (1.0 - color_percent)) as u8,
            (214.0 * color_percent + 114.0 * (1.0 - color_percent)) as u8,
            (172.0 * color_percent + 22.0 * (1.0 - color_percent)) as u8,
        );
        cmds.spawn((
            SandstormParticle,
            StormHeight(height),
            StormCenter(Vec3::ZERO),
            StormUpVelocity(fixed_velocity),
            RigidBody::Dynamic,
            Collider::cuboid(0.1, 0.1, 0.1),
            AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    pos_rad.sin() * height,
                    height,
                    pos_rad.cos() * height,
                ),
                ..default()
            },
        ));
    });
}

fn move_sandstorm_up(
    mut sandstorm: Query<
        (
            &mut LinearVelocity,
            &GlobalTransform,
            &StormHeight,
            &StormUpVelocity,
        ),
        With<SandstormParticle>,
    >,
) {
    sandstorm
        .iter_mut()
        .filter(|(_, pos, StormHeight(height), _)| pos.translation().y < (*height as f32))
        .for_each(|(mut vel, _, _, StormUpVelocity(velocity_up))| {
            vel.y = *velocity_up;
        });
}

fn move_sandstorm_circle(
    mut sandstorm: Query<
        (&mut LinearVelocity, &StormCenter, &GlobalTransform),
        With<SandstormParticle>,
    >,
) {
    sandstorm.iter_mut().for_each(|(mut vel, center, pos)| {
        let current_position = pos.translation();
        let dir_to_center = center.0.xz() - current_position.xz();
        let draw_in_force = (dir_to_center.length() + 1.0) / (current_position.y + 1.0).powf(1.5);
        let dir_perp = dir_to_center.perp();
        let mixed_force =
            (dir_to_center.normalize() * draw_in_force * 0.1 + dir_perp.normalize()) * 10.0;

        vel.x = mixed_force.x;
        vel.z = mixed_force.y;
    });
}

fn move_center(mut storm_center: Query<&mut StormCenter>, keys: Res<ButtonInput<KeyCode>>) {
    let delta = [
        (Vec3::X, KeyCode::ArrowLeft),
        (Vec3::NEG_X, KeyCode::ArrowRight),
        (Vec3::Z, KeyCode::ArrowUp),
        (Vec3::NEG_Z, KeyCode::ArrowDown),
    ]
    .into_iter()
    .fold(Vec3::ZERO, |acc, (delta, key)| {
        keys.pressed(key).then_some(delta).unwrap_or_default() + acc
    });

    storm_center.iter_mut().for_each(|mut center| {
        center.0 += delta * 0.1;
    });
}
