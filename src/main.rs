use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

mod sound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    sound::play_from_file(&stream_handle, "./test.mp3")?;
    sound::play_from_file(&stream_handle, "./test2.mp3")?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(20));

    Ok(())
}
