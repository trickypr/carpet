use std::{thread::sleep, time::Duration};

use crate::SoundHolderMutex;

use super::Holder;

pub fn looper<'a>(sound_mutex: SoundHolderMutex<'a>, sleep_size_seconds: u64) {
    loop {
        let mut sound = sound_mutex.lock().unwrap();

        sound.correct_volume();
        sound.ensure_length();

        drop(sound);

        sleep(Duration::from_secs(sleep_size_seconds));
    }
}

impl Holder {
    pub fn correct_volume(&mut self) {
        for sound in &self.sounds {
            sound.sink.set_volume(sound.volume);

            if !self.is_playing {
                sound.sink.pause();
            }

            if sound.volume == 0.0 {
                sound.sink.pause();
            } else if sound.sink.is_paused() && self.is_playing {
                sound.sink.play();
            }
        }
    }

    /// Ensures that the sound has enough playback left to keep playing until
    /// the next call
    pub fn ensure_length(&mut self) {
        for sound in &self.sounds {
            if sound.sink.len() <= 1 {
                sound
                    .sink
                    .append(rodio::Decoder::new(sound.get_buffer()).unwrap());
            }
        }
    }
}
