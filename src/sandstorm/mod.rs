pub mod post_process;
mod sound;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_kira_audio::{AudioInstance, AudioTween};
use post_process::PostProcessSettings;
use sound::SandstormAudioInstances;

use crate::camera::CameraShake;

pub struct SandstormPlugin;
impl Plugin for SandstormPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SandstormIntensity>()
            .insert_resource(SandstormIntensity(0.0))
            .add_plugins((post_process::PostProcessPlugin, sound::SandstormSoundPlugin))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_visuals, update_audio).run_if(resource_changed::<SandstormIntensity>),
            );
    }
}

// This is attached to a big sphere around the world.
// This is an unfortunate requirement to get fog to apply to the skybox at the moment.
#[derive(Component)]
pub struct SkyboxCover;

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        SkyboxCover,
        PbrBundle {
            mesh: meshes.add(Sphere::new(10000.0)),
            material: materials.add(StandardMaterial {
                unlit: true,
                cull_mode: None,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        },
        NotShadowCaster,
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SandstormIntensity(pub f32);

fn update_visuals(
    mut settings: Query<
        (&mut PostProcessSettings, &mut FogSettings, &mut CameraShake),
        With<Camera>,
    >,
    intensity: Res<SandstormIntensity>,
    q_skybox_cover: Query<&Handle<StandardMaterial>, With<SkyboxCover>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut setting, mut fog, mut shake) in &mut settings {
        setting.strength = intensity.0 * 0.95;
        fog.falloff = FogFalloff::from_visibility(4000.0 * (1.0 - intensity.0.powf(0.1)));
        let mat_handle = q_skybox_cover.single();
        let mat = materials.get_mut(mat_handle).unwrap();
        mat.base_color.set_alpha(intensity.0.powf(0.15));
        shake.0 = intensity.0.powf(2.0);
    }
}

const MAX_SANDSTORM_VOLUME: f32 = 0.7;

fn update_audio(
    intensity: Res<SandstormIntensity>,
    audio_instances: Res<SandstormAudioInstances>,
    mut instances: ResMut<Assets<AudioInstance>>,
) {
    let blend = intensity.0.powf(2.0);

    let weak = (1.0 - blend) * intensity.0 * MAX_SANDSTORM_VOLUME;
    let strong = blend * intensity.0 * MAX_SANDSTORM_VOLUME;
    instances
        .get_mut(audio_instances.weak.id())
        .map(|s| s.set_volume(weak as f64, AudioTween::default()));

    instances
        .get_mut(audio_instances.strong.id())
        .map(|s| s.set_volume(strong as f64, AudioTween::default()));
}
