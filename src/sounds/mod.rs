//! This file contains a single function responsible for initializing all the
//! sounds in the sound folder and setting their settings to be correct by
//! the last config.

use std::fmt::Display;

use rodio::{OutputStream, OutputStreamHandle};

mod control;
mod looper;
mod register;
mod sound;

pub use control::control;
pub use looper::looper;
pub use sound::{Sound, SoundMetadata};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    Water,
    Nature,
    Humans,
    Artificial,
    None,
}

impl Category {
    pub const fn get_all() -> [Category; 4] {
        [
            Category::Water,
            Category::Nature,
            Category::Humans,
            Category::Artificial,
        ]
    }
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
