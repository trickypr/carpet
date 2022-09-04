//! This file contains a single function responsible for initializing all the
//! sounds in the sound folder and setting their settings to be correct by
//! the last config.

use std::{fmt::Display, fs::File, io::BufReader};

use rodio::{OutputStream, OutputStreamHandle, Sink};

mod control;
mod looper;
mod register;

pub use control::control;
pub use looper::looper;

#[derive(Copy, Clone)]
pub enum Category {
    Water,
    Nature,
    Humans,
    Artificial,
    None,
}

impl Display for Category {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Water => write!(formatter, "Water"),
            Category::Nature => write!(formatter, "Nature"),
            Category::Humans => write!(formatter, "Humans"),
            Category::Artificial => write!(formatter, "Artificial"),
            Category::None => write!(formatter, "None"),
        }
    }
}

pub struct Holder {
    pub is_playing: bool,

    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,

    pub current_sound_id: usize,
    pub sounds: Vec<Sound>,
}

impl Holder {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let mut new = Self {
            is_playing: true,

            stream,
            stream_handle,

            current_sound_id: 0,
            sounds: vec![],
        };

        new.register_sounds();

        new
    }

    pub fn get_sound_mut(&mut self, id: usize) -> Option<&mut Sound> {
        self.sounds.iter_mut().find(|sound| sound.id == id)
    }

    pub fn get_new_sound_index(&mut self) -> usize {
        let index = self.current_sound_id;
        self.current_sound_id += 1;
        index
    }
}

pub struct Sound {
    pub name: String,
    pub path: String,
    pub id: usize,
    pub sink: Sink,
    pub volume: f32,
    pub category: Category,
}

impl Sound {
    pub fn get_buffer(&self) -> BufReader<File> {
        BufReader::new(File::open(self.path.clone()).unwrap())
    }

    pub fn new(
        holder: &mut Holder,
        category: Category,
        name: &str,
        file_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = BufReader::new(File::open(file_path)?);
        let sink = holder.stream_handle.play_once(file)?;
        let id = holder.get_new_sound_index();

        // TODO: Reset these to zero
        sink.set_volume(0.0);

        Ok(Sound {
            name: name.to_string(),
            path: file_path.to_string(),
            id,
            sink,
            volume: 0.0,
            category,
        })
    }
}
