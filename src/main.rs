mod audio;
mod battery;
mod beacon;
mod camera;
#[cfg(feature = "dev")]
mod debug;
mod game;
mod input;
mod materials;
mod menu;
mod movement;
mod player;
mod pyramids;
mod sandstorm;
mod settings;
mod shelter;
mod terrain;
mod tower;
mod util;

use avian3d::prelude::*;
use bevy::{
    asset::AssetMetaCheck, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*,
};
use blenvy::BlenvyPlugin;

fn main() {
    let mut app = App::new();
    app
        // External plugins
        .add_plugins((
            DefaultPlugins.build().set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default(),
            BlenvyPlugin::default(),
        ))
        // Game plugins
        .add_plugins((
            (
                camera::CameraPlugin,
                settings::SettingsPlugin,
                input::InputPlugin,
                menu::MenuPlugin,
                movement::MovementPlugin,
                player::PlayerPlugin,
                #[cfg(feature = "dev")]
                debug::DebugPlugin,
                terrain::TerrainPlugin,
                game::GamePlugin,
                beacon::BeaconPlugin,
                sandstorm::SandstormPlugin,
                tower::TowerPlugin,
                audio::AudioPlugin,
                shelter::ShelterPlugin,
                battery::BatteryPlugin,
            ),
            (materials::BuiltinMaterialsPlugin, pyramids::PyramidPlugin),
        ))
        .add_systems(Startup, setup);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(TemporalAntiAliasPlugin);

    app.run();
}

#[derive(Component)]
pub struct Sun;

fn setup(mut cmds: Commands) {
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
}
