use std::{
    collections::HashMap,
    fs::{self, create_dir_all},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The config option for this application. All new options **MUST** be added as
/// an optional field to avoid broken configs
pub struct Config {
    pub is_playing: Option<bool>,
    pub sound_volume: HashMap<String, f32>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            is_playing: Some(true),
            sound_volume: HashMap::new(),
        }
    }
}

impl Config {
    #[cfg(target_os = "linux")]
    fn get_path() -> PathBuf {
        let directory = xdg::BaseDirectories::with_prefix("carpet").unwrap();
        directory.place_config_file("config.json").unwrap()
    }

    #[cfg(target_os = "macos")]
    fn get_path() -> PathBuf {
        let home_dir = home::home_dir().unwrap();
        let local_config_dir = home_dir.join("Library/Application Support");

        if !local_config_dir.exists() {
            create_dir_all(local_config_dir.clone()).unwrap();
        }

        local_config_dir.join("config.json")
    }

    #[cfg(target_os = "windows")]
    fn get_path() -> PathBuf {
        unimplemented!()
    }

    pub fn load() -> Self {
        let config_path = Self::get_path();

        if config_path.exists() {
            serde_json::from_str(&fs::read_to_string(config_path).unwrap()).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let config_path = Self::get_path();
        fs::write(config_path, serde_json::to_string(self).unwrap()).unwrap();
    }
}
