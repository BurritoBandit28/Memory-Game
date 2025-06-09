use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
#[cfg(not(target_os = "emscripten"))]
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
#[cfg(not(target_os = "emscripten"))]
use rodio::cpal::Stream;
use crate::resource_location::ResourceLocation;

/// Used to hold information about a sound
pub struct Sound {
    pub path : String,
    pub resource_location : ResourceLocation
}


/// A struct containing components required for sound playback
pub struct AudioManager {
    #[cfg(not(target_os = "emscripten"))]
    stream: Option<OutputStream>, // this value cannot be dropped, else audio playback stops, hence this struct
    #[cfg(not(target_os = "emscripten"))]
    stream_handle : Option<OutputStreamHandle>
}

impl AudioManager {
    pub fn create() -> Self {
        #[cfg(not(target_os = "emscripten"))]
        let stuff = OutputStream::try_default();
        #[cfg(not(target_os = "emscripten"))]
        if stuff.is_ok() {
            let (stream, stream_handle) = stuff.unwrap();
            Self {
                stream :Some(stream),
                stream_handle : Some(stream_handle)
            }
        }
        else {
            Self{
                stream: None,
                stream_handle: None,
            }
        }

        #[cfg(target_os = "emscripten")]
        Self{}

    }

    /// Plays sound given the sound map and resource location.
    pub fn play_sound(&self, sound : &Sound) {
        #[cfg(not(target_os = "emscripten"))]
        if self.stream.is_some() && self.stream_handle.is_some() {
            // use data within the Sound type to get playable data
            let sound_data = Decoder::new(BufReader::new(File::open(&sound.path).unwrap())).unwrap();
            // play the sound
            self.stream_handle.clone().unwrap().play_raw(sound_data.convert_samples()).expect("Something went wrong with audio playback");
        }


    }

}