///////////////////////////////////////////////////////////////////////////////////////////////////
// imports
///////////////////////////////////////////////////////////////////////////////////////////////////

mod audio;
mod ui;
mod generate;
mod config;

use std::{
    error::Error,
    sync::mpsc,
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    let (synth_tx, synth_rx) = mpsc::channel();

    let audio_thread = audio::run(synth_rx)?;

    let generator_thread = thread::Builder::new()
        .name("generator".into())
        .spawn(move || {
            generate::run(synth_tx);
        })?;

    ui::run()?;

    //generator_thread.join().expect("Generator thread panicked");
    drop(generator_thread); // TODO
    drop(audio_thread); // NOTE: waiting for cpal crate push before handling more gracefully

    Ok(())
}
