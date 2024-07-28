mod ambient;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_kira_audio::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin)
            .add_plugins(ambient::AmbientAudioPlugin)
            .add_systems(Startup, pause_audio)
            .add_systems(
                Update,
                toggle_pause.run_if(input_just_pressed(KeyCode::KeyE)),
            );
    }
}

// TODO: Remove this before release
// generally pauses the main audio channel (at the start)
fn pause_audio(audio: Res<AudioChannel<MainTrack>>) {
    audio.pause();
}

// toggles the audio on/off
fn toggle_pause(audio: Res<AudioChannel<MainTrack>>, mut paused: Local<bool>) {
    if !*paused {
        audio.resume();
    } else {
        audio.pause();
    }
    *paused = !*paused;
}
