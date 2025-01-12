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


pub struct AudioManager {
    stream: OutputStream,
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

    pub fn play_sound(&self, rl : ResourceLocation, sounds : &HashMap<String, Sound>) {
        let sound_data = Decoder::new(BufReader::new(File::open(&sounds.get(&rl.to_string()).unwrap().path).unwrap())).unwrap();
        self.stream_handle.play_raw(sound_data.convert_samples()).expect("Something went wrong with audio playback");
    }

}