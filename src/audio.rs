use rodio::{
    source::{Buffered, Source},
    Decoder, Device, Sink,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

/// A simple 4-track audio system to play audio clips as sound effects.
pub struct Audio {
    endpoint: Device,
    clips: HashMap<&'static str, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    channels: Vec<Sink>,
    current_channel: usize,
}

impl Audio {
    pub fn new() -> Self {
        let endpoint = rodio::default_output_device().unwrap();
        let clips = HashMap::new();
        let mut channels: Vec<Sink> = Vec::new();
        for _ in 0..4 {
            channels.push(Sink::new(&endpoint))
        }
        Self {
            endpoint,
            clips,
            channels,
            current_channel: 0,
        }
    }
    pub fn add(&mut self, name: &'static str, path: &str) {
        let mut file_vec: Vec<u8> = Vec::new();
        File::open(path)
            .expect("Couldn't find audio file to add.")
            .read_to_end(&mut file_vec)
            .expect("Failed reading in opened audio file.");
        let cursor = Cursor::new(file_vec);
        let decoder = Decoder::new(cursor).unwrap();
        let buffered = decoder.buffered();
        // This seems to pre-load the decoder buffer and avoid static on first plays
        let warm = buffered.clone();
        for i in warm {
            drop(i);
        }
        self.clips.insert(name, buffered);
    }
    pub fn play(&mut self, name: &str) {
        let buffer = self.clips.get(name).expect("No clip by that name.").clone();
        self.channels[self.current_channel].append(buffer);
        self.current_channel += 1;
        if self.current_channel >= self.channels.len() {
            self.current_channel = 0;
        }
    }
}
