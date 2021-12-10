use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub sound_volume: HashMap<String, f32>,
}

pub fn get_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    let linux = {
        let directory = xdg::BaseDirectories::with_prefix("carpet").unwrap();
        let config_path = directory.place_config_file("config.json").unwrap();
        config_path
    };

    #[cfg(target_os = "windows")]
    let windows = {};

    #[cfg(target_os = "macos")]
    let macos = {};

    #[cfg(target_os = "linux")]
    return linux;

    #[cfg(target_os = "windows")]
    return windows;

    #[cfg(target_os = "macos")]
    return macos;
}

pub fn default() -> Config {
    Config {
        sound_volume: HashMap::new(),
    }
}

pub fn load() -> Config {
    let config_path = get_path();

    if config_path.exists() {
        serde_json::from_str(&fs::read_to_string(config_path).unwrap()).unwrap()
    } else {
        default()
    }
}

pub fn save(config: Config) {
    let config_path = get_path();
    fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
}
