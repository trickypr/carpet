use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

// use orbtk::prelude::themes::material_icons_font;
// use orbtk::prelude::*;

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use sounds::SoundCategory;
use trev::launch;

mod components;
mod config;
mod sound;
mod sound_category;
mod sounds;

static mut RX: Option<Sender<ControlThreadCommand>> = None;
static mut SOUND: Option<Arc<Mutex<Vec<SoundCategory>>>> = None;

fn app(cx: Scope) -> Element {
    let available_sounds = unsafe { SOUND.as_ref().unwrap().lock().unwrap() };

    let sounds_els = available_sounds
        .iter()
        .map(|category| category.into())
        .map(|category| rsx!(sound_category::AllSounds { category: category }));

    cx.render(rsx! {
        container {
            height: "100%",
            width: "100%",
            padding: "60",
            background: "black",
            text {
                "Hello, world!"
            }

            sounds_els
        }
    })
}

// #[derive(Debug, Default, AsAny)]
// struct MainState {
//     change_volume: Vec<((usize, usize), Entity)>,
//     playing: bool,
// }

// impl MainState {
//     pub fn change_volume(&mut self, sound: ((usize, usize), Entity)) {
//         self.change_volume.push(sound);
//     }

//     pub fn toggle_paused(&mut self) {
//         self.playing = !self.playing;
//     }
// }

// impl State for MainState {
//     fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
//         self.playing = *ctx.widget().get::<bool>("playing");
//     }

//     fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
//         if self.change_volume.len() != 0 {
//             let rx = unsafe { RX.as_ref().unwrap() };

//             for (song_index, entity) in &self.change_volume {
//                 let slider = ctx.get_widget(*entity);
//                 let value = Slider::val_clone(&slider);

//                 rx.send(ControlThreadCommand::ChangeVolume(
//                     *song_index,
//                     value as f32,
//                 ))
//                 .unwrap();
//             }

//             self.change_volume.clear();
//         }

//         if &self.playing != ctx.widget().get::<bool>("playing") {
//             let playing = self.playing;

//             let rx = unsafe { RX.as_ref().unwrap() };
//             rx.send(ControlThreadCommand::SetPlaying(playing)).unwrap();

//             ctx.widget().set("playing", playing);
//         }
//     }
// }

// widget!(MainView<MainState> {
//     playing: bool
// });

// impl Template for MainView {
//     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
//         let available_sounds = unsafe { SOUND.as_ref().unwrap().lock().unwrap() };
//         let mut sounds = Vec::new();

//         for (index, category) in available_sounds.iter().enumerate() {
//             sounds.push(sound_category::display(ctx, category, id, index));
//         }

//         drop(available_sounds);

//         let mut stack = Stack::new().spacing(16).child(
//             Stack::new()
//                 .orientation(Orientation::Horizontal)
//                 .child(
//                     Button::new()
//                         .icon(material_icons_font::MD_PAUSE)
//                         .id("play_pause_button")
//                         .on_click(move |ctx, _position| {
//                             ctx.get_mut::<MainState>(id).toggle_paused();
//                             true
//                         })
//                         .build(ctx),
//                 )
//                 .build(ctx),
//         );

//         for sound in sounds {
//             stack = stack.child(sound);
//         }

//         self.child(
//             Container::new()
//                 .child(ScrollViewer::new().child(stack.build(ctx)).build(ctx))
//                 .padding(32)
//                 .build(ctx),
//         )
//         .playing(true)
//     }
// }

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
    let (_stream, stream_handle) = sound::create_output_stream()?;
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

        loop {
            match tx.recv() {
                Ok(command) => {
                    //(index, volume)
                    match command {
                        ControlThreadCommand::ChangeVolume(index, volume) => {
                            let mut categories = thread_sounds.lock().unwrap();

                            let category = &mut categories[index.0];
                            let sounds = &mut category.sounds;

                            sounds[index.1].volume = volume;

                            if volume == 0.0 {
                                sounds[index.1].sink.pause();
                            } else if sounds[index.1].sink.is_paused() {
                                if is_playing {
                                    sounds[index.1].sink.play();
                                }
                            }

                            sounds[index.1].sink.set_volume(volume);

                            let config_id = sounds::path_to_sound_id(&sounds[index.1].path);
                            config.sound_volume.insert(config_id.to_string(), volume);

                            drop(sounds);

                            config::save(config.clone());
                        }
                        ControlThreadCommand::SetPlaying(local_is_playing) => {
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
                Err(_) => {}
            }
        }
    });

    launch(app);

    Ok(())
}
