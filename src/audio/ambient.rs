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
                start_ambient_music.run_if(not(any_with_component::<AmbientMusic>)),
                remove_finished_ambient_audio.run_if(
                    any_with_component::<AmbientMusic>.and_then(on_timer(Duration::from_secs(1))),
                ),
            ),
        );
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AmbientMusic;

#[derive(Debug, Clone)]
pub struct Title(String);
impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct AmbientMusicHandles(Vec<(Title, Handle<bevy_kira_audio::AudioSource>)>);

impl FromWorld for AmbientMusicHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let handles = [
            "bj5_desert",
            "desert-storm-ii",
            "desert-voices",
            "sahara-sunrise",
        ]
        .map(|name| format!("{name}.mp3"))
        .map(|name| {
            (
                Title(name.clone()),
                asset_server.load(format!("audio/music/{name}")),
            )
        })
        .to_vec();
        Self(handles)
    }
}

fn start_ambient_music(
    mut cmds: Commands,
    ambient_music: Res<AmbientMusicHandles>,
    audio: Res<Audio>,
) {
    let (track_name, track) = ambient_music
        .choose(&mut rand::thread_rng())
        .expect("non empty");
    let audio_handle = audio
        .play(track.clone())
        // The first 0.5 seconds will not be looped and are the "intro"
        // .loop_from(0.5)
        // Fade-in with a dynamic easing
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        // Only play on our right ear
        // .with_panning(1.0)
        // Increase playback rate by 50% (this also increases the pitch)
        // .with_playback_rate(1.5)
        // Play at half volume
        .with_volume(0.5)
        .handle();
    // play the track reversed
    // .reverse();
    cmds.spawn((
        Name::new(format!("Background: {track_name}")),
        AmbientMusic,
        audio_handle,
    ));
}

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
