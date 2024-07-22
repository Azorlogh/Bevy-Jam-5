mod ambient;
mod sfx;

use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::AudioPlugin)
            .add_plugins((sfx::AudioSfxPlugin, ambient::AmbientAudioPlugin));
    }
}
