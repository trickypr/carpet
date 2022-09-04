//! This file contains a single function responsible for initializing all the
//! sounds in the sound folder and setting their settings to be correct by
//! the last config.

use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rodio::OutputStreamHandle;

use crate::{
    config::Config,
    sound::{self, Sound},
};

static mut CURRENT_SOUND_ID: usize = 0;

#[derive(Debug, Default)]
pub struct SoundCategory {
    pub name: String,
    pub id: usize,
    pub sounds: Vec<sound::Sound>,
}

pub struct SoundCategoryLite {
    pub name: String,
    pub id: usize,
    pub sounds: Vec<sound::SoundMetadata>,
}

impl From<&SoundCategory> for SoundCategoryLite {
    fn from(category: &SoundCategory) -> Self {
        SoundCategoryLite {
            name: category.name.clone(),
            id: category.id,
            sounds: category.sounds.iter().map(|sound| sound.into()).collect(),
        }
    }
}

impl PartialEq for SoundCategoryLite {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for SoundCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn new(name: &str, sounds: Vec<sound::Sound>) -> SoundCategory {
    unsafe { CURRENT_SOUND_ID += 1 };

    SoundCategory {
        name: name.to_string(),
        id: unsafe { CURRENT_SOUND_ID },
        sounds,
    }
}

fn push(name: &str, sounds: &mut Vec<SoundCategory>, sound: Vec<sound::Sound>) {
    sounds.push(new(name, sound));
    sound::reset_ids();
}

pub fn init(
    stream_handle: &OutputStreamHandle,
    config: &Config,
) -> Result<Vec<SoundCategory>, Box<dyn Error>> {
    let mut sounds = Vec::new();

    let sound = |name: &str, audio_name: &str| -> Result<sound::Sound, Box<dyn Error>> {
        sound::play_from_file(
            stream_handle,
            &format!("./sounds/{}.ogg", audio_name),
            name,
        )
    };

    push(
        "Water",
        &mut sounds,
        vec![
            sound("Rain", "rain")?,
            sound("Thunder", "storm")?,
            sound("Stream", "stream")?,
            sound("Waves", "waves")?,
            sound("Boat", "boat")?,
        ],
    );

    push(
        "Nature",
        &mut sounds,
        vec![
            sound("Birds", "birds")?,
            sound("Wind", "wind")?,
            sound("Summer Night", "summer-night")?,
        ],
    );

    push(
        "Humans",
        &mut sounds,
        vec![
            sound("City", "city")?,
            sound("Coffee Shop", "coffee-shop")?,
            sound("Fireplace", "fireplace")?,
            sound("Train", "train")?,
        ],
    );

    push(
        "Artificial",
        &mut sounds,
        vec![
            sound("Pink Noise", "pink-noise")?,
            sound("White Noise", "white-noise")?,
        ],
    );

    for category in sounds.iter_mut() {
        for sound in category.sounds.iter_mut() {
            let sound_config_id = path_to_sound_id(&sound.path);

            if config.sound_volume.contains_key(sound_config_id) {
                sound.volume = *config.sound_volume.get(sound_config_id).unwrap();
            }
        }
    }

    Ok(sounds)
}

pub fn path_to_sound_id<'a>(path: &'a str) -> &'a str {
    Path::new(path).file_stem().unwrap().to_str().unwrap()
}

#[inline]
pub fn looper<'a>(sounds_mutex: &'a Arc<Mutex<Vec<SoundCategory>>>, sleep_time_seconds: u64) {
    loop {
        let mut sounds = sounds_mutex.lock().unwrap();

        for category in sounds.iter_mut() {
            for sound in category.sounds.iter_mut() {
                if sound.sink.len() <= 1 {
                    sound.sink.append(
                        rodio::Decoder::new(BufReader::new(
                            File::open(sound.path.clone()).unwrap(),
                        ))
                        .unwrap(),
                    );
                }

                sound.sink.set_volume(sound.volume);
            }
        }

        drop(sounds);

        thread::sleep(Duration::from_secs(sleep_time_seconds));
    }
}
