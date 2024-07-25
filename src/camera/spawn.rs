use bevy::{
    core_pipeline::{bloom::BloomSettings, experimental::taa::TemporalAntiAliasBundle, Skybox},
    pbr::{ScreenSpaceAmbientOcclusionBundle, VolumetricFogSettings},
    prelude::*,
};

use super::{flycam::FlyCam, follow::CameraAngles, MainCamera};
use crate::sandstorm;

pub fn setup_normal(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((
        MainCamera,
        CameraAngles::default(),
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        FlyCam,
        Skybox {
            brightness: 2500.0,
            // image: asset_server.load("environment_maps/main_specular_rgb9e5_zstd.ktx2"),
            image: asset_server.load("environment_maps/night_02_specular_rgb9e5_zstd.ktx2"),
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/night_02_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/night_02_specular_rgb9e5_zstd.ktx2"),
            intensity: 500.0,
        },
        sandstorm::PostProcessSettings {
            strength: 0.5,
            blowout_factor: 2.5,
            distort_strength: 0.04,
            xspd: -1.2,
            yspd: 0.7,
            ..default()
        },
    ))
    .insert(ScreenSpaceAmbientOcclusionBundle::default())
    .insert(TemporalAntiAliasBundle::default())
    .insert(VolumetricFogSettings {
        ambient_intensity: 1.0,
        ..default()
    })
    .insert(BloomSettings::NATURAL);
}
