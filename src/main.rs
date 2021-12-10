use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use orbtk::prelude::*;
use sound::{loop_sounds, Sound};

mod sound;

static mut RX: Option<Sender<(usize, f32)>> = None;
static mut SOUND: Option<Arc<Mutex<Vec<Sound>>>> = None;

type SoundVec = Vec<String>;

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
    count: usize,
    sounds: SoundVec
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.count(0).sounds(vec![]).child(
            ItemsWidget::new()
                .count(id)
                .items_builder(move |bc, index| {
                    let sound_name = bc.get_widget(id).get::<SoundVec>("sounds")[index].clone();

                    Stack::new()
                        .child(TextBlock::new().text(sound_name).build(bc))
                        .child(
                            Slider::new()
                                .id(format!("sound_{}", index))
                                .min(0.0)
                                .val(0.0)
                                .max(1.0)
                                .min_width(100)
                                .on_changed("val", move |states, widget_id| {
                                    states
                                        .get_mut::<MainState>(id)
                                        .change_volume((index, widget_id));
                                })
                                .build(bc),
                        )
                        .build(bc)
                })
                .build(ctx),
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, tx) = mpsc::channel::<(usize, f32)>();

    unsafe {
        RX = Some(rx);
    }

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = sound::create_output_stream()?;

    let mut sounds = Vec::new();

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/birds.ogg",
        "Birds",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/boat.ogg",
        "Boat",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/city.ogg",
        "City",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/coffee-shop.ogg",
        "Coffee Shop",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/fireplace.ogg",
        "Fireplace",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/pink-noise.ogg",
        "Pink Noise",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/rain.ogg",
        "Rain",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/storm.ogg",
        "Storm",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/stream.ogg",
        "Stream",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/summer-night.ogg",
        "Summer Night",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/train.ogg",
        "Train",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/waves.ogg",
        "Waves",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/white-noise.ogg",
        "White Noise",
    )?);

    sounds.push(sound::play_from_file(
        &stream_handle,
        "./sounds/wind.ogg",
        "Wind",
    )?);

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
    thread::spawn(move || loop {
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
            }
            Err(_) => {}
        }
    });

    let sounds = thread_sounds;

    Application::new()
        .window(move |ctx| {
            let sounds = sounds.lock().unwrap();

            let window = Window::new()
                .title("Carpet")
                .size(200, 500)
                .resizeable(true)
                .child(
                    MainView::new()
                        .count(sounds.len())
                        .sounds(
                            sounds
                                .iter()
                                .map(|sound| sound.name.clone())
                                .collect::<SoundVec>(),
                        )
                        .build(ctx),
                )
                .build(ctx);

            drop(sounds);

            window
        })
        .run();

    Ok(())
}
