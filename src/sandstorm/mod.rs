pub mod post_process;

use bevy::{pbr::NotShadowCaster, prelude::*};
use post_process::PostProcessSettings;

use crate::camera::CameraShake;

pub struct SandstormPlugin;
impl Plugin for SandstormPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SandstormIntensity>()
            .insert_resource(SandstormIntensity(0.0))
            .add_plugins(post_process::PostProcessPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                update_visuals.run_if(resource_changed::<SandstormIntensity>),
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
pub struct SandstormIntensity(f32);

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
        fog.falloff = FogFalloff::from_visibility(10000.0 * (1.0 - intensity.0.sqrt()));
        let mat_handle = q_skybox_cover.single();
        let mat = materials.get_mut(mat_handle).unwrap();
        mat.base_color.set_alpha(intensity.0);
        shake.0 = intensity.0.powf(2.0);
    }
}
