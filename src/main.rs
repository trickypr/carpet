use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread,
};

use config::Config;
use freya::launch_with_title;
use frontend::app;
use sounds::Holder;

mod config;
mod frontend;
mod sounds;

type SoundHolderMutex<'a> = &'a Arc<Mutex<sounds::Holder>>;

/// The amount of time, in seconds, for the sound to fade in
pub const FADE_IN_TIME: f32 = 1.5;

pub static mut RX: Option<Sender<ControlThreadCommand>> = None;
pub static mut SOUND: Option<Arc<Mutex<sounds::Holder>>> = None;

pub fn get_holder<'a>() -> MutexGuard<'a, Holder> {
    unsafe { SOUND.as_ref().unwrap().lock().unwrap() }
}

pub enum ControlThreadCommand {
    ChangeVolume(usize, f32),
    SetPlaying(bool),
}

fn sync_config(config: &Config) {
    let mut holder = get_holder();

    holder.is_playing = config.is_playing.unwrap_or(true);

    for sound in &mut holder.sounds {
        if let Some(config_volume) = config.sound_volume.get(&sound.name) {
            sound.volume = *config_volume;
        }
    }

    holder.correct_volume();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, tx) = mpsc::channel();

    unsafe {
        RX = Some(rx);
    }

    // Create the sound handler
    let sound_holder = sounds::Holder::new();

    unsafe {
        SOUND = Some(Arc::new(Mutex::new(sound_holder)));
    }

    // We need to ensure that the config is initialized before any threads are
    // spawned or the UI is created, otherwise they might default to zero when
    // setting up.
    let config = Config::load();
    sync_config(&config);

    // This thread is responsible for infinitely looping the audio that will be
    // heard by the user. The duration must be set to be shorter than all of the
    //  sounds that are being played
    thread::spawn(move || sounds::looper(unsafe { SOUND.as_ref().unwrap() }, 10));

    // This thread is responsible for adjusting the volume of the sounds that are
    // being played. It works based on messages send from the state object
    thread::spawn(move || sounds::control(tx, config));

    launch_with_title(app, "Carpet");

    Ok(())
}
