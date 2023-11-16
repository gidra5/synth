use std::{cmp::min, collections::HashSet};

use itertools::Itertools;
use rustysynth::Synthesizer;

#[derive(Eq, Hash, Clone, Copy)]
pub struct Note {
    channel: i32,
    key: i32,
    volume: i32,
    interrupt: bool,
    overlap: bool,
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.channel == other.channel
            && self.key == other.key
            && !self.interrupt
            && !self.overlap
            && !other.interrupt
            && !other.overlap
    }
}

impl Note {
    pub fn new(key: i32) -> Note {
        Note {
            key,
            channel: 0,
            volume: 100,
            interrupt: false,
            overlap: false,
        }
    }

    pub fn octave(self, octave: i32) -> Note {
        Note {
            key: 12 * octave + (self.key % 12),
            ..self
        }
    }

    pub fn volume(self, volume: i32) -> Note {
        Note { volume, ..self }
    }

    pub fn channel(self, channel: i32) -> Note {
        Note { channel, ..self }
    }

    pub fn key(self, key: i32) -> Note {
        Note { key, ..self }
    }

    pub fn note(self, note: i32) -> Note {
        Note {
            key: self.key / 12 + (note),
            ..self
        }
    }

    pub fn interrupt(self) -> Note {
        Note {
            interrupt: true,
            ..self
        }
    }

    pub fn transpose(self, interval: i32) -> Note {
        Note {
            key: self.key + interval,
            ..self
        }
    }

    pub fn on(&self, synthesizer: &mut Synthesizer) {
        synthesizer.note_on(self.channel, self.key, self.volume);
    }

    pub fn off(&self, synthesizer: &mut Synthesizer) {
        synthesizer.note_off(self.channel, self.key);
    }

    pub fn add_note(self, notes: &mut HashSet<Note>) {
        notes.insert(self);
    }
}

#[derive(Clone)]
pub struct Scale {
    intervals: Box<[u8]>,
    mode: usize,
    base: i32,
}

impl Scale {
    pub fn new(intervals: &[u8]) -> Scale {
        Scale {
            intervals: intervals.into(),
            mode: 0,
            base: 0,
        }
    }

    pub fn note(&self, note: usize) -> Note {
        Note::new(
            self.intervals
                .iter()
                .map(|x| *x as i32)
                .cycle()
                .skip(self.mode)
                .take(note + 1)
                .sum(),
        )
        .transpose(self.base)
    }

    pub fn mode(self, mode: i32) -> Scale {
        let len = self.intervals.len() as i32;
        let rem = mode % len;
        Scale {
            mode: (if rem < 0 { len - rem } else { rem }) as usize,
            ..self
        }
    }

    pub fn rotate(self) -> Scale {
        let len = self.intervals.len();
        let intervals = self
            .intervals
            .iter()
            .map(|x| *x)
            .cycle()
            .skip(self.mode)
            .take(len)
            .collect_vec();
        Scale {
            mode: 0,
            intervals: intervals.into(),
            ..self
        }
    }

    pub fn mode_rotate(self, mode: i32) -> Scale {
        self.mode(mode).rotate()
    }

    pub fn transpose(self, base: i32) -> Scale {
        Scale { base, ..self }
    }

    pub fn chord(&self, base: i32) -> Chord {
        Chord {
            size: 3,
            base,
            scale: self.clone(),
        }
    }

    pub fn distance(&self, other: &Scale) -> u8 {
        let is_self_higher = self.base > other.base;
        let (higher, lower) = if is_self_higher {
            (self, other)
        } else {
            (other, self)
        };
        let (higher, lower) = (higher.clone(), lower.clone());

        let diff = higher.base - lower.base;
        let higher = higher
            .clone()
            .transpose(lower.base)
            .mode_rotate(higher.mode as i32 - diff);

        let (_self, other) = if is_self_higher {
            (higher, lower)
        } else {
            (lower, higher)
        };

        // how many note alterations are required to get from one to the other
        // that means how many keys are different
        // pure alterations count as dist 1, missing notes count as dist 0.5, and added notes count as 2
        // TODO: use tree_edit_distance
        Self::intervals_distance(_self.intervals, other.intervals)
    }

    fn intervals_distance(left: Box<[u8]>, right: Box<[u8]>) -> u8 {
        0
    }
}

#[derive(Clone)]
pub struct Chord {
    size: u32,
    base: i32,
    scale: Scale,
}

impl Chord {
    pub fn size(self, size: u32) -> Self {
        Chord { size, ..self }
    }

    pub fn base(self, base: i32) -> Self {
        Chord { base, ..self }
    }

    pub fn scale(self, base: i32) -> Self {
        Chord { base, ..self }
    }

    pub fn notes(self) -> Box<[Note]> {
        (0..self.size)
            .into_iter()
            .map(|i| self.scale.note((self.base + 2 * (i as i32)) as usize))
            .collect()
    }

    pub fn add_notes(self, notes: &mut HashSet<Note>) {
        for note in self.notes().iter() {
            notes.insert(*note);
        }
    }
}

impl Node for Box<[u8]> {
    type Kind = u8;
    fn kind(&self) -> Self::Kind {}

    type Weight = u64;
    fn weight(&self) -> Self::Weight {
        1
    }
}

impl Tree for Box<[u8]> {
    type Children<'c> = Box<dyn Iterator<Item = &'c Self> + 'c>
    where
        Self: 'c;

    fn children(&self) -> Self::Children<'_> {
        match self {
            Json::Array(a) => Box::new(a.iter()),
            Json::Map(m) => Box::new(m.iter().map(|(_, v)| v)),
            _ => Box::new(empty()),
        }
    }
}
