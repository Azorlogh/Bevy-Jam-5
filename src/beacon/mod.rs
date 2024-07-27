use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, AudioSource};

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
pub struct BeaconAssets {
    model_bottom: Handle<Scene>,
    model_segment: Handle<Scene>,
    model_top_off: Handle<Scene>,
    model_top_on: Handle<Scene>,
    sfx_plant: Handle<AudioSource>,
    sfx_segment: Handle<AudioSource>,
    sfx_light: Handle<AudioSource>,
}

fn setup_models(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.insert_resource(BeaconAssets {
        model_bottom: asset_server.load("models/beacon/bottom.glb#Scene0"),
        model_segment: asset_server.load("models/beacon/segment.glb#Scene0"),
        model_top_off: asset_server.load("models/beacon/top_off.glb#Scene0"),
        model_top_on: asset_server.load("models/beacon/top_on.glb#Scene0"),
        sfx_plant: asset_server.load("audio/sfx/beacon_plant.ogg"),
        sfx_segment: asset_server.load("audio/sfx/beacon_segment.ogg"),
        sfx_light: asset_server.load("audio/sfx/beacon_light.ogg"),
    });
}

fn beacon_spawn(
    mut cmds: Commands,
    time: Res<Time>,
    q_added_beacons: Query<(Entity, &Transform), Added<Beacon>>,
    assets: Res<BeaconAssets>,
    audio: Res<Audio>,
) {
    for (e, transform) in &q_added_beacons {
        let mut anchor = e;
        let mut segments = vec![];
        for _ in 0..10 {
            let segment = cmds
                .spawn((
                    SceneBundle {
                        scene: assets.model_segment.clone(),
                        ..default()
                    },
                    AudioEmitter {
                        instances: vec![audio.play(assets.sfx_segment.clone()).paused().handle()],
                    },
                ))
                .set_parent(anchor)
                .id();
            segments.push(segment);
            anchor = segment;
        }
        let top_off = cmds
            .spawn(SceneBundle {
                scene: assets.model_top_off.clone(),
                ..default()
            })
            .set_parent(anchor)
            .id();
        let top_on = cmds
            .spawn((
                SceneBundle {
                    scene: assets.model_top_on.clone(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                AudioEmitter {
                    instances: vec![audio.play(assets.sfx_light.clone()).paused().handle()],
                },
            ))
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
                    scene: assets.model_bottom.clone(),
                    ..default()
                },
                AudioEmitter {
                    instances: vec![audio.play(assets.sfx_plant.clone()).handle()],
                },
            ))
            .insert(Transform::from_xyz(0.0, 0.5, 0.0) * transform.clone());
    }
}

fn deploy(
    time: Res<Time>,
    q_beacons: Query<(&BeaconParts, &BeaconTimestamp)>,
    mut q_segment: Query<(&mut Transform, &AudioEmitter)>,
    mut q_end: Query<&mut Visibility>,
    q_emitter: Query<&AudioEmitter>,
    mut prev_time: Local<f32>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (parts, timestamp) in &q_beacons {
        let t = (time.elapsed_seconds() - timestamp.0) * 2.0;
        let prev_t = (*prev_time - timestamp.0) * 2.0;
        for (i, segment_e) in parts.segments.iter().enumerate() {
            let (mut segment_tr, emitter) = q_segment.get_mut(*segment_e).unwrap();
            if (t - i as f32) > 1.0 && (prev_t - i as f32) <= 1.0 {
                println!("playing because: {} {}", t, prev_t);
                emitter
                    .instances
                    .get(0)
                    .and_then(|inst| audio_instances.get_mut(inst.id()))
                    .map(|s| s.resume(default()));
            }
            let t = (t - i as f32).clamp(0.0, 1.0).powf(4.0);
            segment_tr.translation.y = t * SEGMENT_HEIGHT;
        }
        let end_t = parts.segments.len() as f32 + 1.0;
        if t > end_t && prev_t <= end_t {
            *q_end.get_mut(parts.top_off).unwrap() = Visibility::Hidden;
            *q_end.get_mut(parts.top_on).unwrap() = Visibility::Visible;
            let audio = q_emitter.get(parts.top_on).unwrap();
            audio_instances
                .get_mut(audio.instances[0].id())
                .map(|s| s.resume(default()));
        }
    }
    *prev_time = time.elapsed_seconds();
}
