use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread,
};

use freya::launch_with_title;
use frontend::app;
use sounds::Holder;

mod config;
mod frontend;
mod sounds;

type SoundHolderMutex<'a> = &'a Arc<Mutex<sounds::Holder>>;

pub static mut RX: Option<Sender<ControlThreadCommand>> = None;
pub static mut SOUND: Option<Arc<Mutex<sounds::Holder>>> = None;

pub fn get_holder<'a>() -> MutexGuard<'a, Holder> {
    unsafe { SOUND.as_ref().unwrap().lock().unwrap() }
}

pub enum ControlThreadCommand {
    ChangeVolume(usize, f32),
    SetPlaying(bool),
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

    // This thread is responsible for infinitely looping the audio that will be
    // heard by the user. The duration must be set to be shorter than all of the
    //  sounds that are being played
    thread::spawn(move || sounds::looper(unsafe { SOUND.as_ref().unwrap() }, 10));

    // This thread is responsible for adjusting the volume of the sounds that are
    // being played. It works based on messages send from the state object
    thread::spawn(move || sounds::control(tx));

    launch_with_title(app, "Carpet");

    Ok(())
}
