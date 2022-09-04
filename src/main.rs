use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use trev::launch;

mod components_int;
mod config;
mod sounds;

type SoundHolderMutex<'a> = &'a Arc<Mutex<sounds::Holder>>;

pub static mut RX: Option<Sender<ControlThreadCommand>> = None;
pub static mut SOUND: Option<Arc<Mutex<sounds::Holder>>> = None;

fn app(cx: Scope) -> Element {
    let available_sounds = unsafe { SOUND.as_ref().unwrap().lock().unwrap() };

    // let sounds_els = available_sounds
    //     .iter()
    //     .map(|category| category.into())
    //     .enumerate()
    //     .map(|(i, category)| {
    //         rsx!(sound_category::AllSounds {
    //             category: category,
    //             category_index: i
    //         })
    //     });

    cx.render(rsx! {
        view {
            height: "100%",
            width: "100%",
            padding: "60",
            background: "black",

            // sounds_els
        }
    })
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

    launch(app);

    Ok(())
}
