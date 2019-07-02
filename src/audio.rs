use rodio::{
    source::{Buffered, Source},
    Decoder, Sink,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};

/// A simple 4-track audio system to load/decode audio files from disk to play later. Supported
/// formats are: MP3, WAV, Vorbis and Flac.  This is just a thin convenience layer on top of
/// [rodio](https://github.com/tomaka/rodio).
pub struct Audio {
    clips: HashMap<&'static str, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    channels: Vec<Sink>,
    current_channel: usize,
}

impl Audio {
    /// Create a new sound subsystem.  You only need one of these -- you can use it to load and play
    /// any number of audio clips.
    pub fn new() -> Self {
        let endpoint = rodio::default_output_device().unwrap();
        let clips = HashMap::new();
        let mut channels: Vec<Sink> = Vec::new();
        for _ in 0..4 {
            channels.push(Sink::new(&endpoint))
        }
        Self {
            clips,
            channels,
            current_channel: 0,
        }
    }
    /// Add an audio clip to play.  Audio clips will be decoded and buffered during this call so
    /// the first call to `.play()` is not staticky if you compile in debug mode.  `name` is what
    /// you will refer to this clip as when you need to play it.
    pub fn add(&mut self, name: &'static str, path: &str) {
        let mut file_vec: Vec<u8> = Vec::new();
        File::open(path)
            .expect("Couldn't find audio file to add.")
            .read_to_end(&mut file_vec)
            .expect("Failed reading in opened audio file.");
        let cursor = Cursor::new(file_vec);
        let decoder = Decoder::new(cursor).unwrap();
        let buffered = decoder.buffered();
        // Buffers are lazily decoded, which often leads to static on first play on low-end systems
        // or when you compile in debug mode.  Since this is an educational project, those are going
        // to be common conditions.  So, to optimize for our use-case, we will pre-warm all of our
        // audio buffers by forcing things to be decoded and cached right now when we first load the
        // file.  I would like to find a cleaner way to do this, but the following scheme (iterating
        // through a clone and discarding the decoded frames) works since clones of a Buffered share
        // the actual decoded data buffer cache by means of Arc and Mutex.
        let warm = buffered.clone();
        for i in warm {
            drop(i);
        }
        self.clips.insert(name, buffered);
    }
    /// Play an audio clip that has already been loaded.  `name` is the name you chose when you
    /// added the clip to the `Audio` system. If you forgot to load the clip first, this will crash.
    pub fn play(&mut self, name: &str) {
        let buffer = self.clips.get(name).expect("No clip by that name.").clone();
        self.channels[self.current_channel].append(buffer);
        self.current_channel += 1;
        if self.current_channel >= self.channels.len() {
            self.current_channel = 0;
        }
    }
}
