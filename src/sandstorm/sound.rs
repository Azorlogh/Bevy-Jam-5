use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::camera::MainCamera;

// controls the max distance at which you can barely hear spatial audio
const SPATIAL_AUDIO_DISTANCE: f32 = 50.0;

pub struct SandstormSoundPlugin;
impl Plugin for SandstormSoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialAudio {
            max_distance: SPATIAL_AUDIO_DISTANCE,
        })
        .add_systems(Startup, init_sandstorm_emitter)
        .add_systems(
            Update,
            init_camera_audio_receiver
                .run_if(any_with_component::<MainCamera>.and_then(run_once())),
        );
    }
}

#[derive(Clone, Resource)]
pub struct SandstormAudioInstances {
    pub weak: Handle<AudioInstance>,
    pub strong: Handle<AudioInstance>,
}

fn init_sandstorm_emitter(mut cmds: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    // load general sandstorm audio effect
    let weak_handle = asset_server.load("audio/sfx/sandstorm-weak.mp3");
    let strong_handle = asset_server.load("audio/sfx/sandstorm-strong.ogg");

    let instances = SandstormAudioInstances {
        weak: audio
            .play(weak_handle.clone())
            .looped()
            .with_volume(0.0)
            .handle(),
        strong: audio
            .play(strong_handle.clone())
            .looped()
            .with_volume(0.0)
            .handle(),
    };

    cmds.insert_resource(instances.clone());
}

// attach the spatial audio receiver component to the `MainCamera` entity for now
fn init_camera_audio_receiver(mut cmds: Commands, camera: Query<Entity, With<MainCamera>>) {
    cmds.entity(camera.single()).insert(AudioReceiver);
}
