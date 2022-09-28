use std::{fs::File, io::BufReader};

use rodio::Sink;

use super::{Category, Holder};

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

#[derive(Debug, Clone)]
pub struct SoundMetadata {
    pub name: String,
    pub id: usize,
    pub volume: f32,
    pub category: Category,
}

impl From<&Sound> for SoundMetadata {
    fn from(sound: &Sound) -> Self {
        Self {
            name: sound.name.clone(),
            id: sound.id,
            volume: sound.volume,
            category: sound.category,
        }
    }
}

impl PartialEq for SoundMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.volume == other.volume
    }
}
