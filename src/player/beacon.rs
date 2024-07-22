use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::beacon::Beacon;

#[derive(Component, Reflect)]
pub struct BeaconCount(pub usize);

pub fn place_beacon(
    mut cmds: Commands,
    mut q_player: Query<(Entity, &mut BeaconCount)>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
    spatial: SpatialQuery,
) {
    for (e, mut beacons) in &mut q_player {
        let Ok(cam_tr) = q_camera.get_single() else {
            continue;
        };
        let origin = cam_tr.translation();
        let dir = cam_tr.forward();
        if let Some(hit) = spatial.cast_ray(
            origin,
            dir,
            100.0,
            true,
            SpatialQueryFilter::from_excluded_entities([e]),
        ) {
            let p = origin + dir * hit.time_of_impact;
            beacons.0 += 1;
            cmds.spawn((
                Beacon,
                SpatialBundle::from_transform(Transform::from_translation(p)),
            ));
        }
    }
}
