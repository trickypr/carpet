use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use sounds::SoundCategory;
use trev::launch;

mod components_int;
mod config;
mod sound;
mod sound_category;
mod sounds;

pub static mut RX: Option<Sender<ControlThreadCommand>> = None;
pub static mut SOUND: Option<Arc<Mutex<Vec<SoundCategory>>>> = None;

fn app(cx: Scope) -> Element {
    let available_sounds = unsafe { SOUND.as_ref().unwrap().lock().unwrap() };

    let sounds_els = available_sounds
        .iter()
        .map(|category| category.into())
        .enumerate()
        .map(|(i, category)| {
            rsx!(sound_category::AllSounds {
                category: category,
                category_index: i
            })
        });

    cx.render(rsx! {
        view {
            height: "100%",
            width: "100%",
            padding: "60",
            background: "black",

            sounds_els
        }
    })
}

pub enum ControlThreadCommand {
    ChangeVolume((usize, usize), f32),
    SetPlaying(bool),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, tx) = mpsc::channel();

    unsafe {
        RX = Some(rx);
    }

    // Get the application config
    let config = config::load();

    // Get a output stream handle to the default physical sound device
    let (stream, stream_handle) = sound::create_output_stream()?;
    let sounds = sounds::init(&stream_handle, &config)?;

    unsafe {
        SOUND = Some(Arc::new(Mutex::new(sounds)));
    }

    let thread_sounds = unsafe { SOUND.as_ref().unwrap() };

    // This thread is responsible for infinitely looping the audio that will be
    // heard by the user. The duration must be set to be shorter than all of the
    //  sounds that are being played
    thread::spawn(move || sounds::looper(thread_sounds, 10));

    // This thread is responsible for adjusting the volume of the sounds that are
    // being played. It works based on messages send from the state object
    thread::spawn(move || {
        let mut config = config::load();
        let mut is_playing = config.is_playing.unwrap_or(true);

        is_playing = true;

        loop {
            if let Ok(command) = tx.recv() {
                //(index, volume)
                match command {
                    ControlThreadCommand::ChangeVolume(index, volume) => {
                        println!("cat: {}, sound: {}, vol: {}", index.0, index.1, volume);

                        let mut categories = thread_sounds.lock().unwrap();

                        let category = &mut categories[index.0];
                        let sounds = &mut category.sounds;

                        sounds[index.1].volume = volume;

                        println!("{:#?}", sounds[index.1]);

                        if volume == 0.0 {
                            sounds[index.1].sink.pause();
                        } else if sounds[index.1].sink.is_paused() && is_playing {
                            sounds[index.1].sink.play();
                        }

                        sounds[index.1].sink.set_volume(volume);

                        let config_id = sounds::path_to_sound_id(&sounds[index.1].path);
                        config.sound_volume.insert(config_id.to_string(), volume);

                        config::save(config.clone());
                    }
                    ControlThreadCommand::SetPlaying(local_is_playing) => {
                        println!("{}", local_is_playing);


                        is_playing = local_is_playing;
                        config.is_playing = Some(is_playing);

                        if local_is_playing {
                            let mut categories = thread_sounds.lock().unwrap();

                            for category in categories.iter_mut() {
                                for sound in category.sounds.iter_mut() {
                                    if sound.volume != 0.0 {
                                        sound.sink.play();
                                    }
                                }
                            }

                            drop(categories);
                        } else {
                            let mut categories = thread_sounds.lock().unwrap();

                            for category in categories.iter_mut() {
                                for sound in category.sounds.iter_mut() {
                                    if sound.volume != 0.0 {
                                        sound.sink.pause();
                                    }
                                }
                            }

                            drop(categories);
                        }

                        config::save(config.clone());
                    }
                }
            }
        }
    });

    launch(app);

    Ok(())
}
