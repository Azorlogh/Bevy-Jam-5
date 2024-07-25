use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::camera::MainCamera;

pub struct AudioSfxPlugin;
impl Plugin for AudioSfxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialAudio { max_distance: 50.0 })
            .add_systems(Startup, init_sandstorm_emitter)
            .add_systems(
                Update,
                init_camera_audio_receiver
                    .run_if(any_with_component::<MainCamera>.and_then(run_once())),
            );
    }
}

fn init_sandstorm_emitter(mut cmds: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let sandstorm_handle = asset_server.load("audio/sfx/sandstorm-looping.mp3");
    let mut i = 1;
    let spawn_sandstorm_at = |pos: Vec3| {
        cmds.spawn((
            Name::new(format!("Sandstorm {i}")),
            AudioEmitter {
                instances: vec![audio.play(sandstorm_handle.clone()).looped().handle()],
            },
            SpatialBundle {
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
        ));
        i += 1;
    };
    [Vec3::X, Vec3::NEG_X, Vec3::NEG_Z]
        .map(|x| x * 40.0)
        .into_iter()
        .for_each(spawn_sandstorm_at);
}

fn init_camera_audio_receiver(mut cmds: Commands, camera: Query<Entity, With<MainCamera>>) {
    cmds.entity(camera.single()).insert(AudioReceiver);
}
