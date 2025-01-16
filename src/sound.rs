use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use rodio::cpal::Stream;
use crate::resource_location::ResourceLocation;

/// Used to hold information about a sound
pub struct Sound {
    pub path : String,
    pub resource_location : ResourceLocation
}


/// A struct containing components required for sound playback
pub struct AudioManager {
    stream: OutputStream, // this value cannot be dropped, else audio playback stops, hence this struct
    stream_handle : OutputStreamHandle
}

impl AudioManager {
    pub fn create() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            stream,
            stream_handle
        }
    }

    /// Plays sound given the sound map and resource location.
    pub fn play_sound(&self, sound : &Sound) {
        // use data within the Sound type to get playable data
        let sound_data = Decoder::new(BufReader::new(File::open(&sound.path).unwrap())).unwrap();
        // play the sound
        self.stream_handle.play_raw(sound_data.convert_samples()).expect("Something went wrong with audio playback");
    }

}