use rodio::source::Repeat;
use rodio::{source::Source, Decoder, OutputStream};
use rodio::{OutputStreamHandle, PlayError};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct Sound {
    source: Repeat<Decoder<BufReader<File>>>,
}

pub fn create_output_stream() -> Result<OutputStreamHandle, Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    Ok(stream_handle)
}

pub fn new(path: &str) -> Result<Sound, Box<dyn std::error::Error>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?.repeat_infinite();

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
