use bevy::prelude::*;

pub struct BeaconPlugin;
impl Plugin for BeaconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_models)
            .add_systems(Update, (beacon_spawn, deploy));
    }
}

#[derive(Component)]
pub struct Beacon;

#[derive(Component, Reflect)]
pub struct BeaconParts {
    segments: Vec<Entity>,
    top_off: Entity,
    top_on: Entity,
}

#[derive(Component)]
pub struct BeaconTimestamp(f32);

const SEGMENT_HEIGHT: f32 = 2.19;

#[derive(Resource)]
pub struct BeaconModels {
    bottom: Handle<Scene>,
    segment: Handle<Scene>,
    top_off: Handle<Scene>,
    top_on: Handle<Scene>,
}

fn setup_models(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(BeaconModels {
        bottom: asset_server.load("models/beacon/bottom.glb#Scene0"),
        segment: asset_server.load("models/beacon/segment.glb#Scene0"),
        top_off: asset_server.load("models/beacon/top_off.glb#Scene0"),
        top_on: asset_server.load("models/beacon/top_on.glb#Scene0"),
    });
}

fn beacon_spawn(
    mut cmds: Commands,
    time: Res<Time>,
    q_added_beacons: Query<(Entity, &Transform), Added<Beacon>>,
    models: Res<BeaconModels>,
) {
    for (e, transform) in &q_added_beacons {
        let mut anchor = e;
        let mut segments = vec![];
        for _ in 0..10 {
            let segment = cmds
                .spawn(SceneBundle {
                    scene: models.segment.clone(),
                    ..default()
                })
                .set_parent(anchor)
                .id();
            segments.push(segment);
            anchor = segment;
        }
        let top_off = cmds
            .spawn(SceneBundle {
                scene: models.top_off.clone(),
                ..default()
            })
            .set_parent(anchor)
            .id();
        let top_on = cmds
            .spawn(SceneBundle {
                scene: models.top_on.clone(),
                visibility: Visibility::Hidden,
                ..default()
            })
            .set_parent(anchor)
            .id();
        cmds.entity(e)
            .insert((
                BeaconParts {
                    segments,
                    top_off,
                    top_on,
                },
                BeaconTimestamp(time.elapsed_seconds()),
                SceneBundle {
                    scene: models.bottom.clone(),
                    ..default()
                },
            ))
            .insert(Transform::from_xyz(0.0, 0.5, 0.0) * transform.clone());
    }
}

fn deploy(
    time: Res<Time>,
    q_beacons: Query<(&BeaconParts, &BeaconTimestamp)>,
    mut q_transform: Query<&mut Transform>,
    mut q_visibility: Query<&mut Visibility>,
) {
    for (parts, timestamp) in &q_beacons {
        let t = (time.elapsed_seconds() - timestamp.0) * 2.0;
        for (i, segment_e) in parts.segments.iter().enumerate() {
            let t = (t - i as f32).clamp(0.0, 1.0).powf(4.0);
            q_transform.get_mut(*segment_e).unwrap().translation.y = t * SEGMENT_HEIGHT;
        }
        if t > parts.segments.len() as f32 + 1.0 {
            *q_visibility.get_mut(parts.top_off).unwrap() = Visibility::Hidden;
            *q_visibility.get_mut(parts.top_on).unwrap() = Visibility::Visible;
        }
    }
}
