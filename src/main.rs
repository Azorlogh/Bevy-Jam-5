mod audio;
mod beacon;
mod camera;
mod debug;
mod game;
mod input;
mod loddy;
mod menu;
mod movement;
mod player;
mod sandstorm;
mod settings;
mod terrain;

use avian3d::prelude::*;
use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};
use blenvy::BlenvyPlugin;

fn main() {
    App::new()
        // External plugins
        .add_plugins((
            DefaultPlugins,
            TemporalAntiAliasPlugin,
            PhysicsPlugins::default(),
            BlenvyPlugin::default(),
        ))
        // Game plugins
        .add_plugins((
            camera::CameraPlugin,
            settings::SettingsPlugin,
            input::InputPlugin,
            menu::MenuPlugin,
            movement::MovementPlugin,
            player::PlayerPlugin,
            debug::DebugPlugin,
            terrain::TerrainPlugin,
            game::GamePlugin,
            beacon::BeaconPlugin,
            sandstorm::SandstormPlugin,
            audio::AudioPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Sun;

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Static physics object with a collision shape
    // cmds.spawn((
    //     RigidBody::Static,
    //     Collider::cylinder(40.0, 0.1),
    //     PbrBundle {
    //         mesh: meshes.add(Cylinder::new(40.0, 0.1)),
    //         material: materials.add(StandardMaterial {
    //             base_color_texture: Some(asset_server.load("textures/test_texture.png")),
    //             uv_transform: Affine2::from_scale(Vec2::splat(5.0)),
    //             ..default()
    //         }),
    //         ..default()
    //     },
    // ));

    // Dynamic physics object with a collision shape and initial angular velocity
    cmds.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Light
    cmds.spawn((
        Sun,
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 13000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: {
                let pos = Quat::from_axis_angle(Vec3::Y, 35f32.to_radians())
                    * Quat::from_axis_angle(Vec3::Z, 25f32.to_radians())
                    * Vec3::X;
                Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Z)
            },
            ..default()
        },
    ));

    cmds.spawn((
        Name::new("Temple"),
        SceneBundle {
            scene: asset_server.load("levels/Scene.glb#Scene0"),
            ..default()
        },
    ));
}
