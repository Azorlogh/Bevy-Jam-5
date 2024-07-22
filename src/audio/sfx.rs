use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{camera::MainCamera, storm::SandstormParticle};

pub struct AudioSfxPlugin;
impl Plugin for AudioSfxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialAudio { max_distance: 50.0 })
            .add_systems(
                Update,
                (
                    init_sandstorm_emitter
                        .run_if(any_with_component::<SandstormParticle>.and_then(run_once())),
                    init_camera_audio_receiver
                        .run_if(any_with_component::<MainCamera>.and_then(run_once())),
                ),
            );
    }
}

fn init_sandstorm_emitter(
    mut cmds: Commands,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    sandstorm: Query<Entity, With<SandstormParticle>>,
) {
    cmds.entity(sandstorm.iter().next().unwrap())
        .insert(AudioEmitter {
            instances: vec![audio
                .play(asset_server.load("audio/sandstorm-looping.mp3"))
                .looped()
                .handle()],
        });
}

fn init_camera_audio_receiver(mut cmds: Commands, camera: Query<Entity, With<MainCamera>>) {
    cmds.entity(camera.single()).insert(AudioReceiver);
}
