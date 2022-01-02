use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::sounds::SoundCategory;

static mut CURRENT_SOUND_ID: usize = 0;

pub struct SoundMetadata {
    pub name: String,
    pub path: String,
    pub id: usize,
    pub volume: f32,
}

impl From<&Sound> for SoundMetadata {
    fn from(sound: &Sound) -> Self {
        SoundMetadata {
            name: sound.name.clone(),
            path: sound.path.clone(),
            id: sound.id,
            volume: sound.volume,
        }
    }
}

pub struct Sound {
    pub name: String,
    pub path: String,
    pub id: usize,
    pub sink: Sink,
    pub volume: f32,
}

impl Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sound")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("id", &self.id)
            .field("volume", &self.volume)
            .finish()
    }
}

#[inline]
pub fn loop_sounds<'a>(sounds_mutex: &'a Arc<Mutex<Vec<SoundCategory>>>, sleep_time_seconds: u64) {
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

#[inline]
pub fn create_output_stream() -> Result<(OutputStream, OutputStreamHandle), Box<dyn Error>> {
    Ok(OutputStream::try_default()?)
}

pub fn reset_ids() {
    unsafe { CURRENT_SOUND_ID = 0 };
}

pub fn play_from_file(
    stream_handle: &OutputStreamHandle,
    path: &str,
    name: &str,
) -> Result<Sound, Box<dyn std::error::Error>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    let sink = stream_handle.play_once(file)?;
    let id = unsafe {
        let id = CURRENT_SOUND_ID;
        CURRENT_SOUND_ID += 1;
        id
    };

    sink.set_volume(0.0);

    Ok(Sound {
        name: name.to_string(),
        path: path.to_string(),
        id,
        sink,
        volume: 0.0,
    })
}
