use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

static mut CURRENT_SOUND_ID: usize = 0;

pub struct Sound {
    pub name: String,
    pub path: String,
    pub id: usize,
    pub sink: Sink,
    pub volume: f32,
}

pub fn create_output_stream() -> Result<(OutputStream, OutputStreamHandle), Box<dyn Error>> {
    Ok(OutputStream::try_default()?)
}

pub fn play_from_file(
    stream_handle: &OutputStreamHandle,
    path: &str,
    name: &str,
) -> Result<Sound, Box<dyn std::error::Error>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    let sink = stream_handle.play_once(file)?;
    let id = unsafe {
        let id = CURRENT_SOUND_ID;
        CURRENT_SOUND_ID += 1;
        id
    };

    sink.set_volume(0.0);

    Ok(Sound {
        name: name.to_string(),
        path: path.to_string(),
        id,
        sink,
        volume: 0.0,
    })
}
