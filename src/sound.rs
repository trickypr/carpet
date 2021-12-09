use rodio::{source, OutputStreamHandle, PlayError};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{BufReader, Error};

pub struct Sound {
    source: Decoder<BufReader<File>>,
}

pub fn new(path: &str) -> Result<Sound, Box<dyn std::error::Error>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?;

    Ok(Sound { source })
}

pub fn play_from_file(
    stream_handle: &OutputStreamHandle,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let sound = new(path)?;
    play(stream_handle, sound)?;

    Ok(())
}

pub fn play(stream_handle: &OutputStreamHandle, source: Sound) -> Result<(), PlayError> {
    stream_handle.play_raw(source.source.convert_samples())
}
