use std::collections::HashSet;

use music::Music;
use structs::{Note, Scale};
use utils::pause;

mod music;
mod structs;
mod utils;

fn main() {
    let _device = MusicContext::play("soundfonts/Perfect Sine.sf2");

    pause();
}

// struct MusicContext {}

// impl Music for MusicContext {
//     fn init() -> Self {
//         Self {}
//     }

//     fn next_notes(&mut self, time: u128) -> HashSet<Note> {
//         let mut notes: HashSet<Note> = HashSet::new();

//         let beats_per_minute = 25;
//         let tempo = 60 * 1000 / beats_per_minute;
//         let measure = time / tempo;
//         let measure_time = time % tempo;

//         let scale = Scale::new(&[1]);
//         let scale = Scale::new(&[7, 5]);
//         let scale = Scale::new(&[1, 2, 2, 2, 2, 2, 1]); // ???
//         let scale = Scale::new(&[2, 2, 3, 2, 3]); // pentatonic major scale
//         let scale = Scale::new(&[2, 5, 2, 3]);
//         let scale = Scale::new(&[2, 7, 3]);
//         let scale = Scale::new(&[5, 7]);
//         let scale = Scale::new(&[2, 1, 2, 2, 2, 2, 1]); // melodic minor scale
//         let scale = Scale::new(&[2, 1, 2, 2, 2, 1, 1, 1]); // melodic minor scale
//         let scale = Scale::new(&[2, 2, 1, 2, 2, 2, 1]); // major scale

//         // Play some notes (middle C, E, G).
//         // for i in 1..(time / 1000) {
//         //     notes.insert(Note::new(0 + 12 * (i as i32)).volume(50));
//         //     notes.insert(Note::new(4 + 12 * (i as i32)).volume(50));
//         //     notes.insert(Note::new(7 + 12 * (i as i32)).volume(50));
//         // }

//         // notes.insert(Note::new(0 + 12 * ((time / 1000) as i32)).volume(50));
//         // notes.insert(Note::new(4 + 12 * ((time / 1000) as i32)).volume(50));
//         // notes.insert(Note::new(7 + 12 * ((time / 1000) as i32)).volume(50));

//         // let base = 12 * ((time / 1000) as i32);
//         // notes.insert(scale.note(0).transpose(base).volume(50));
//         // notes.insert(scale.note(2).transpose(base).volume(50));
//         // notes.insert(scale.note(4).transpose(base).volume(50));

//         // let key = (time / 1000) as usize;
//         // let base = ((time / 700) % 2) as i32;
//         // notes.insert(scale.note(key).octave(5).volume(50));
//         // notes.insert(scale.note(key).transpose(base).volume(50));
//         // let mode = (time / 1400) as usize;
//         // notes.insert(scale.mode(mode).note(key).transpose(60).volume(50));
//         // notes.insert(
//         //     scale
//         //         .mode(mode)
//         //         .note(key)
//         //         .transpose(60 + 12 * base)
//         //         .volume(50),
//         // );
//         // notes.insert(scale.note(key).transpose(60 + 12 * base).volume(50));
//         // notes.insert(Note::new(key).volume(50));

//         if measure % 2 == 0 {
//             if measure_time < tempo / 4 {
//                 scale.note(1).transpose(60).volume(50).add_note(&mut notes);
//             } else if measure_time < tempo / 2 {
//                 scale.note(3).transpose(60).volume(50).add_note(&mut notes);
//             } else if measure_time < tempo * 3 / 4 {
//                 notes.insert(scale.note(4).transpose(60).volume(50));
//             } else if measure_time < tempo {
//                 notes.insert(scale.note(5).transpose(60).volume(50));
//             }

//             if measure_time < tempo / 4 {
//                 notes.insert(scale.note(1).transpose(48).volume(100));
//                 notes.insert(scale.note(5).transpose(48).volume(100));
//             } else if measure_time < tempo / 2 {
//                 notes.insert(scale.note(1).transpose(48).volume(100));
//                 notes.insert(scale.note(5).transpose(48).volume(100));
//                 // notes.insert(scale.note(3).transpose(36).volume(50));
//                 // notes.insert(scale.note(5).transpose(36).volume(50));
//                 // notes.insert(scale.note(7).transpose(24).volume(50));
//             } else if measure_time < tempo {
//                 notes.insert(scale.note(3).transpose(48).volume(100));
//                 notes.insert(scale.note(5).transpose(48).volume(100));
//                 notes.insert(scale.note(7).transpose(36).volume(100));
//                 // notes.insert(scale.note(5).transpose(36).volume(50));
//                 // notes.insert(scale.note(7).transpose(36).volume(50));
//                 // notes.insert(scale.note(9).transpose(36).volume(50));
//             }
//         } else if measure % 2 == 1 {
//             if measure_time < tempo / 4 {
//                 notes.insert(scale.note(7).transpose(60).volume(50));
//             } else if measure_time < tempo / 2 {
//                 notes.insert(scale.note(5).transpose(60).volume(50));
//             } else if measure_time < tempo * 3 / 4 {
//                 notes.insert(scale.note(6).transpose(60).volume(50));
//             } else if measure_time < tempo {
//                 notes.insert(scale.note(8).transpose(60).volume(50));
//             }

//             if measure_time < tempo / 4 {
//                 notes.insert(scale.note(7).transpose(36).volume(100));
//                 notes.insert(scale.note(9).transpose(36).volume(100));
//             } else if measure_time < tempo / 2 {
//                 notes.insert(scale.note(5).transpose(36).volume(100));
//                 notes.insert(scale.note(9).transpose(36).volume(100));
//             } else if measure_time < tempo * 3 / 4 {
//                 notes.insert(scale.note(1).transpose(48).volume(100));
//                 notes.insert(scale.note(3).transpose(36).volume(100));
//                 notes.insert(scale.note(5).transpose(36).volume(100));
//             } else if measure_time < tempo {
//                 notes.insert(scale.note(1).transpose(36).volume(100));
//                 notes.insert(scale.note(3).transpose(36).volume(100));
//                 notes.insert(scale.note(5).transpose(48).volume(100));
//             }
//         }

//         notes
//     }
// }

struct MusicContext {}

impl Music for MusicContext {
    fn init() -> Self {
        Self {}
    }

    fn next_notes(&mut self, time: u128) -> HashSet<Note> {
        let mut notes: HashSet<Note> = HashSet::new();

        let beats_per_minute = 25;
        let tempo = 60 * 1000 / beats_per_minute;
        let measure = time / tempo;
        let measure_time = time % tempo;

        let scale = Scale::new(&[2, 2, 1, 2, 2, 2, 1]); // major scale

        if measure_time < tempo / 2 {
            scale.clone().transpose(48).note(1).add_note(&mut notes);
            scale.clone().transpose(48).note(5).add_note(&mut notes);
        } else if measure_time < tempo / 2 {
            scale.clone().transpose(48).note(3).add_note(&mut notes);
            scale.clone().transpose(48).note(5).add_note(&mut notes);
        } else if measure_time < tempo * 3 / 4 {
            scale.clone().transpose(48).note(1).add_note(&mut notes);
            scale.clone().transpose(48).note(3).add_note(&mut notes);
        } else if measure_time < tempo {
            scale
                .clone()
                .transpose(48)
                .chord(3)
                .size(2)
                .add_notes(&mut notes);
            scale.clone().transpose(36).note(7).add_note(&mut notes);
        }

        // if measure % 2 == 0 {
        //     if measure_time < tempo / 4 {
        //         scale.note(1).transpose(60).volume(50).add_note(&mut notes);
        //     } else if measure_time < tempo / 2 {
        //         scale.note(3).transpose(60).volume(50).add_note(&mut notes);
        //     } else if measure_time < tempo * 3 / 4 {
        //         notes.insert(scale.note(4).transpose(60).volume(50));
        //     } else if measure_time < tempo {
        //         notes.insert(scale.note(5).transpose(60).volume(50));
        //     }

        //     if measure_time < tempo / 2 {
        //         notes.insert(scale.note(1).transpose(48).volume(100));
        //         notes.insert(scale.note(5).transpose(48).volume(100));
        //     } else if measure_time < tempo {
        //         scale.transpose(48).chord(3).size(2).add_notes(&mut notes);
        //         scale.transpose(36).note(7).add_note(&mut notes);
        //     }
        // } else if measure % 2 == 1 {
        //     if measure_time < tempo / 4 {
        //         notes.insert(scale.note(7).transpose(60).volume(50));
        //     } else if measure_time < tempo / 2 {
        //         notes.insert(scale.note(5).transpose(60).volume(50));
        //     } else if measure_time < tempo * 3 / 4 {
        //         notes.insert(scale.note(6).transpose(60).volume(50));
        //     } else if measure_time < tempo {
        //         notes.insert(scale.note(8).transpose(60).volume(50));
        //     }

        //     if measure_time < tempo / 4 {
        //         notes.insert(scale.note(7).transpose(36).volume(100));
        //         notes.insert(scale.note(9).transpose(36).volume(100));
        //     } else if measure_time < tempo / 2 {
        //         notes.insert(scale.note(5).transpose(36).volume(100));
        //         notes.insert(scale.note(9).transpose(36).volume(100));
        //     } else if measure_time < tempo * 3 / 4 {
        //         notes.insert(scale.note(1).transpose(48).volume(100));
        //         notes.insert(scale.note(3).transpose(36).volume(100));
        //         notes.insert(scale.note(5).transpose(36).volume(100));
        //     } else if measure_time < tempo {
        //         notes.insert(scale.note(1).transpose(36).volume(100));
        //         notes.insert(scale.note(3).transpose(36).volume(100));
        //         notes.insert(scale.note(5).transpose(48).volume(100));
        //     }
        // }

        notes
    }
}
