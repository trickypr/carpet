use std::{
    fmt,
    fs::File,
    io::BufReader,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use orbtk::prelude::*;
use sound::Sound;

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
                        .orientation(Orientation::Horizontal)
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
        "./pink-noise.ogg",
        "Pink Noise",
    )?);

    unsafe {
        SOUND = Some(Arc::new(Mutex::new(sounds)));
    }

    let thread_sounds = unsafe { SOUND.as_ref().unwrap() };

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(5));

        let mut sounds = thread_sounds.lock().unwrap();

        for sound in sounds.iter_mut() {
            if sound.sink.len() < 100_000 {
                let file = BufReader::new(File::open(&sound.path).unwrap());
                sound.sink.append(rodio::Decoder::new(file).unwrap());
            }

            sound.sink.set_volume(sound.volume);
        }

        match tx.recv() {
            Ok((index, volume)) => {
                sounds[index].volume = volume;
            }
            Err(_) => {}
        }

        drop(sounds);
    });

    let sounds = unsafe { SOUND.as_ref().unwrap() };

    Application::new()
        .window(move |ctx| {
            let sounds = sounds.lock().unwrap();

            let window = Window::new()
                .title("Carpet")
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
