use std::{thread::sleep, time::Duration};

use crate::{SoundHolderMutex, FADE_IN_TIME};

use super::Holder;

pub fn looper<'a>(sound_mutex: SoundHolderMutex<'a>, sleep_size_seconds: u64) {
    let start_time = std::time::Instant::now();

    while start_time.elapsed().as_secs_f32() < FADE_IN_TIME {
        let mut holder = sound_mutex.lock().unwrap();
        holder.fade_all(start_time.elapsed().as_secs_f32() / FADE_IN_TIME);

        drop(holder);
        sleep(Duration::from_millis(10));
    }

    loop {
        let mut sound = sound_mutex.lock().unwrap();

        sound.correct_volume();
        sound.ensure_length();

        drop(sound);

        sleep(Duration::from_secs(sleep_size_seconds));
    }
}

impl Holder {
    pub fn fade_all(&mut self, percent: f32) {
        for sound in &mut self.sounds {
            sound.sink.set_volume(sound.volume * percent);
        }
    }

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
