use std::{fs::read_to_string, path::PathBuf};

use bevy::prelude::*;

use super::Settings;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_settings() -> Settings {
    let path = settings_path();
    // Load Settings
    match read_to_string(path) {
        Ok(s) => match ron::from_str::<Settings>(&s) {
            Ok(s) => s,
            Err(e) => {
                warn!("failed to load settings, using defaults: {e}");
                Settings::default()
            }
        },
        Err(e) => {
            warn!("failed to load settings, using defaults: {e}");
            Settings::default()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_settings() -> Settings {
    Settings::default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_settings(settings: &Settings) {
    let path = settings_path();
    // Save Settings
    match ron::to_string(settings) {
        Ok(s) => match std::fs::write(path.clone(), s.clone()) {
            Ok(_) => {}
            Err(_) => {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, s).unwrap();
            }
        },
        Err(e) => warn!("failed to save settings: {e}"),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn save_settings(_settings: &Settings) {
    // No-op
    // TODO: local storage?
}

fn settings_path() -> PathBuf {
    directories::ProjectDirs::from("", "AAPPR", "BevyJam5")
        .unwrap()
        .config_dir()
        .join("settings.ron")
}
