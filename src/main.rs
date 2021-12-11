use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use orbtk::prelude::*;
use sound::{loop_sounds, Sound};

mod config;
mod sound;
mod sounds;

static mut RX: Option<Sender<(usize, f32)>> = None;
static mut SOUND: Option<Arc<Mutex<Vec<Sound>>>> = None;

#[derive(Debug, Default, AsAny)]
struct MainState {
    change_volume: Vec<(usize, Entity)>,
}

impl MainState {
    pub fn change_volume(&mut self, sound: (usize, Entity)) {
        self.change_volume.push(sound);
    }
}

impl State for MainState {
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if self.change_volume.len() == 0 {
            return;
        }

        let rx = unsafe { RX.as_ref().unwrap() };

        for (song_index, entity) in &self.change_volume {
            let slider = ctx.get_widget(*entity);
            let value = Slider::val_clone(&slider);

            rx.send((*song_index, value as f32)).unwrap();
        }

        self.change_volume.clear();
    }
}

widget!(MainView<MainState> {
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let available_sounds = unsafe { SOUND.as_ref().unwrap().lock().unwrap() };
        let mut sounds = Vec::new();

        for sound in available_sounds.iter() {
            let sound_id = sound.id.clone();

            sounds.push(
                Container::new()
                    .padding(8)
                    .child(
                        Stack::new()
                            .child(TextBlock::new().text(sound.name.clone()).build(ctx))
                            .child(
                                Slider::new()
                                    .id(format!("sound_{}", sound.id.clone()))
                                    .min(0.0)
                                    .val(sound.volume.clone())
                                    .max(1.0)
                                    .min_width(100)
                                    .on_changed("val", move |states, widget_id| {
                                        states
                                            .get_mut::<MainState>(id)
                                            .change_volume((sound_id, widget_id));
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            );
        }

        drop(available_sounds);

        let mut stack = Stack::new();

        for sound in sounds {
            stack = stack.child(sound);
        }

        self.child(ScrollViewer::new().child(stack.build(ctx)).build(ctx))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, tx) = mpsc::channel::<(usize, f32)>();

    unsafe {
        RX = Some(rx);
    }

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = sound::create_output_stream()?;

    let sounds = sounds::init(&stream_handle)?;

    unsafe {
        SOUND = Some(Arc::new(Mutex::new(sounds)));
    }

    let thread_sounds = unsafe { SOUND.as_ref().unwrap() };

    // This thread is responsible for infinitely looping the audio that will be
    // heard by the user. The duration must be set to be shorter than all of the
    //  sounds that are being played
    thread::spawn(move || loop_sounds(thread_sounds, 10));

    // This thread is responsible for adjusting the volume of the sounds that are
    // being played. It works based on messages send from the state object
    thread::spawn(move || {
        let mut config = config::load();

        loop {
            match tx.recv() {
                Ok((index, volume)) => {
                    let mut sounds = thread_sounds.lock().unwrap();
                    sounds[index].volume = volume;

                    if volume == 0.0 {
                        sounds[index].sink.pause();
                    } else if sounds[index].sink.is_paused() {
                        sounds[index].sink.play();
                    }

                    sounds[index].sink.set_volume(volume);

                    let config_id = sounds::path_to_sound_id(&sounds[index].path);
                    config.sound_volume.insert(config_id.to_string(), volume);

                    drop(sounds);

                    config::save(config.clone());
                }
                Err(_) => {}
            }
        }
    });

    Application::new()
        .window(move |ctx| {
            let window = Window::new()
                .title("Carpet")
                .size(200, 500)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx);

            window
        })
        .run();

    Ok(())
}
