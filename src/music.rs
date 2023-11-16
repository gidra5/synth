use std::{collections::HashSet, fs::File, sync::Arc, time::Instant};

use itertools::Itertools;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

use crate::structs::Note;
use tinyaudio::prelude::*;

pub trait Music
where
    Self: Sized + Send + 'static,
{
    fn next_notes(&mut self, time: u128) -> HashSet<Note>;
    fn init() -> Self;

    fn play(sf: &str) -> Box<dyn BaseAudioOutputDevice> {
        // Setup the audio output.
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 4410,
        };

        // Buffer for the audio output.
        let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
        let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

        // Load the SoundFont.
        let mut sf2 = File::open(sf).unwrap();
        let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

        // Create the MIDI file sequencer.
        let settings = SynthesizerSettings::new(params.sample_rate as i32);
        let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

        let start = Instant::now();
        let mut notes: HashSet<Note> = HashSet::new();
        let mut music: Self = Self::init();

        // Start the audio output.
        run_output_device(params, move |data| {
            let time = Instant::now() - start;
            let ms = time.as_millis();
            let next = music.next_notes(ms);

            for note in notes.difference(&next) {
                note.off(&mut synthesizer);
            }
            for note in next.difference(&notes) {
                note.on(&mut synthesizer);
            }

            notes = next;

            synthesizer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        })
        .unwrap()
    }
}
