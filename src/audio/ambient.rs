use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_kira_audio::prelude::*;
use rand::seq::SliceRandom;
use std::time::Duration;

pub struct AmbientAudioPlugin;

impl Plugin for AmbientAudioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AmbientMusic>();
        app.init_resource::<AmbientMusicHandles>();
        app.add_systems(
            Update,
            (
                start_random_ambient_track.run_if(
                    not(any_with_component::<AmbientMusic>)
                        .and_then(resource_exists::<AmbientMusicHandles>),
                ),
                remove_finished_ambient_audio.run_if(
                    any_with_component::<AmbientMusic>.and_then(on_timer(Duration::from_secs(1))),
                ),
            ),
        );
    }
}

/// component attached to entity which plays the current background music. Normally there should
/// only ever be one such entity
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AmbientMusic;

/// Title of an ambient music track
#[derive(Debug, Clone)]
pub struct TrackTitle(String);
impl std::fmt::Display for TrackTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// All background ambient music handles which could possibly ever play
#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct AmbientMusicHandles(Vec<(TrackTitle, Handle<bevy_kira_audio::AudioSource>)>);

impl FromWorld for AmbientMusicHandles {
    fn from_world(world: &mut World) -> Self {
        // TODO: load this in a more async and non-blocking way
        // load a fixed set of tracks on app start
        let asset_server = world.resource::<AssetServer>();
        let handles = [
            "bj5_desert.ogg",
            "desert-storm-ii.mp3",
            "desert-voices.mp3",
            "sahara-sunrise.mp3",
        ]
        .map(|name| {
            (
                TrackTitle(name.to_owned()),
                asset_server.load(format!("audio/music/{name}")),
            )
        })
        .to_vec();
        Self(handles)
    }
}

fn start_random_ambient_track(
    mut cmds: Commands,
    ambient_music: Res<AmbientMusicHandles>,
    audio: Res<Audio>,
) {
    let (track_name, track) = ambient_music
        .choose(&mut rand::thread_rng())
        .expect("non empty");
    let audio_handle = audio
        .play(track.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .with_volume(0.5)
        .handle();
    cmds.spawn((
        Name::new(format!("Background: {track_name}")),
        AmbientMusic,
        audio_handle,
    ));
}

// if a track is finished playing, despawn the related entity. This is important because the next
// track will only start once there are no track entities in the world left
fn remove_finished_ambient_audio(
    mut cmds: Commands,
    q_ambient_handles: Query<(Entity, &Handle<AudioInstance>), With<AmbientMusic>>,
    audio_instances: Res<Assets<AudioInstance>>,
) {
    q_ambient_handles
        .iter()
        .filter(|(_, handle)| {
            audio_instances
                .get(*handle)
                .is_some_and(|instance| matches!(instance.state(), PlaybackState::Stopped))
        })
        .for_each(|(finished_audio, _)| {
            cmds.entity(finished_audio).despawn_recursive();
        });
}
