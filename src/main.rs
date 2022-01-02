use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use orbtk::prelude::*;
use sound::loop_sounds;
use sounds::SoundCategory;

mod config;
mod sound;
mod sound_category;
mod sounds;

static mut RX: Option<Sender<((usize, usize), f32)>> = None;
static mut SOUND: Option<Arc<Mutex<Vec<SoundCategory>>>> = None;

#[derive(Debug, Default, AsAny)]
struct MainState {
    change_volume: Vec<((usize, usize), Entity)>,
}

impl MainState {
    pub fn change_volume(&mut self, sound: ((usize, usize), Entity)) {
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

        for (index, category) in available_sounds.iter().enumerate() {
            sounds.push(sound_category::display(ctx, category, id, index));
        }

        drop(available_sounds);

        let mut stack = Stack::new().spacing(16);

        for sound in sounds {
            stack = stack.child(sound);
        }

        self.child(
            Container::new()
                .child(ScrollViewer::new().child(stack.build(ctx)).build(ctx))
                .padding(32)
                .build(ctx),
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, tx) = mpsc::channel::<((usize, usize), f32)>();

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
                    let mut categories = thread_sounds.lock().unwrap();

                    let category = &mut categories[index.0];
                    let sounds = &mut category.sounds;

                    sounds[index.1].volume = volume;

                    if volume == 0.0 {
                        sounds[index.1].sink.pause();
                    } else if sounds[index.1].sink.is_paused() {
                        sounds[index.1].sink.play();
                    }

                    sounds[index.1].sink.set_volume(volume);

                    let config_id = sounds::path_to_sound_id(&sounds[index.1].path);
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
                .resizable(true)
                .child(MainView::new().build(ctx))
                .build(ctx);

            window
        })
        .run();

    Ok(())
}
