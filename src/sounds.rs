//! This file contains a single function responsible for initializing all the
//! sounds in the sound folder and setting their settings to be correct by
//! the last config.

use std::{error::Error, path::Path};

use rodio::OutputStreamHandle;

use crate::{config, sound};

pub fn init(stream_handle: &OutputStreamHandle) -> Result<Vec<sound::Sound>, Box<dyn Error>> {
    let mut sounds = Vec::new();

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/birds.ogg",
        "Birds",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/boat.ogg",
        "Boat",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/city.ogg",
        "City",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/coffee-shop.ogg",
        "Coffee Shop",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/fireplace.ogg",
        "Fireplace",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/pink-noise.ogg",
        "Pink Noise",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/rain.ogg",
        "Rain",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/storm.ogg",
        "Storm",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/stream.ogg",
        "Stream",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/summer-night.ogg",
        "Summer Night",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/train.ogg",
        "Train",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/waves.ogg",
        "Waves",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/white-noise.ogg",
        "White Noise",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/wind.ogg",
        "Wind",
    )?);

    // Correctly set the volume of all the sounds
    let config = config::load();

    for sound in sounds.iter_mut() {
        let sound_config_id = path_to_sound_id(&sound.path);

        if config.sound_volume.contains_key(sound_config_id) {
            sound.volume = config.sound_volume.get(sound_config_id).unwrap().clone();
        }
    }

    Ok(sounds)
}

pub fn path_to_sound_id<'a>(path: &'a str) -> &'a str {
    Path::new(path).file_stem().unwrap().to_str().unwrap()
}
