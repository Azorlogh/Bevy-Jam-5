use bevy::{color::palettes::css::BLUE, prelude::*};
use std::f32::consts::TAU;

use crate::{terrain::TerrainParams, util::poisson_disc_sampling};

pub struct PyramidPlugin;
impl Plugin for PyramidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, pyramid_light_beam);
    }
}

#[derive(Component)]
pub struct Pyramid;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>, terrain_params: Res<TerrainParams>) {
    let region = 5000.0;
    for p in poisson_disc_sampling(700.0, region, 5, vec![Vec2::splat(region / 2.0)]) {
        let p = p - region / 2.0;
        let height = terrain_params.get_height(p) + 3.0;
        cmds.spawn((
            Name::new("Pyramid"),
            Pyramid,
            SceneBundle {
                scene: asset_server.load("levels/Pyramid.glb#Scene0"),
                transform: Transform::from_translation(p.extend(height).xzy())
                    .with_rotation(Quat::from_rotation_y(rand::random::<f32>() * TAU)),
                ..default()
            },
        ));
    }
}

pub fn pyramid_light_beam(mut gizmos: Gizmos, q_pyramids: Query<&Transform, With<Pyramid>>) {
    for tr in &q_pyramids {
        gizmos.ray(tr.translation, Vec3::Y * 10000.0, BLUE);
    }
}
