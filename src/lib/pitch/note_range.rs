use super::{Interval, Note, NoteRange};
use super::*; // TODO: super::consts::*;

impl NoteRange {
    pub const fn new(lower_sounding: Note, upper_sounding: Note) -> NoteRange {
        NoteRange {
            lower_sounding,
            upper_sounding,
            write_transposition: PER_1,
        }
    }

    pub const fn sounding_transposition(self, transposition: Interval) -> NoteRange {
        NoteRange {
            lower_sounding: self.lower_sounding.add_const(transposition),
            upper_sounding: self.upper_sounding.add_const(transposition),
            write_transposition: transposition.neg_const(),
        }
    }
}

// https://en.wikipedia.org/wiki/Vocal_range
pub const FEMALE_SOPRANO_RANGE: NoteRange = NoteRange::new(C4, C6);
pub const FEMALE_MEZZO_SOPRANO_RANGE: NoteRange = NoteRange::new(A3, A5);
pub const FEMALE_ALTO_RANGE: NoteRange = NoteRange::new(F3, E5); 
 pub const MALE_TENOR_RANGE: NoteRange = NoteRange::new(B2, A4);
pub const MALE_BARYTON_RANGE: NoteRange = NoteRange::new(G2, F4);
pub const MALE_BASS_RANGE: NoteRange = NoteRange::new(E2, E4);
// http://www.orchestralibrary.com/reftables/rang.html
pub const PICCOLO_RANGE: NoteRange = NoteRange::new(D4, C7).sounding_transposition(PER_8);
pub const FLUTE_RANGE: NoteRange = NoteRange::new(C4, D7);
pub const ALTO_FLUTE_RANGE: NoteRange = NoteRange::new(C4, C7).sounding_transposition(PER_4);
pub const OBOE_RANGE: NoteRange = NoteRange::new(Bb3, A6);
pub const OBOE_D_AMORE_RANGE: NoteRange = NoteRange::new(Bb3, E6).sounding_transposition(MIN_3);
pub const ENGLISH_HORN_RANGE: NoteRange = NoteRange::new(B3, G6).sounding_transposition(PER_5);
pub const BASS_OBOE_RANGE: NoteRange = NoteRange::new(A3, G6).sounding_transposition(PER_8);
pub const CLARINET_BB_RANGE: NoteRange = NoteRange::new(E3, C7).sounding_transposition(MAJ_2);
pub const CLARINET_A_RANGE: NoteRange = NoteRange::new(E3, C7).sounding_transposition(MIN_3);
pub const CLARINET_D_RANGE: NoteRange = NoteRange::new(E3, C7).sounding_transposition(MAJ_2);
pub const CLARINET_EB_RANGE: NoteRange = NoteRange::new(E3, C7).sounding_transposition(MIN_3);
pub const BASSET_HORN_RANGE: NoteRange = NoteRange::new(C3, G6).sounding_transposition(PER_5);
pub const BASS_CLARINET_BB: NoteRange = NoteRange::new(Eb3, G6).sounding_transposition(MAJ_9);
pub const BASSOON: NoteRange = NoteRange::new(Bb1, Eb5);
pub const CONTRABASSOON: NoteRange = NoteRange::new(Bb1, Bb4).sounding_transposition(PER_8);
pub const SAXOPHONE_BB_SOPRANO: NoteRange = NoteRange::new(Bb3, G6).sounding_transposition(MAJ_2);
pub const SAXOPHONE_EB_ALTO: NoteRange = NoteRange::new(Bb3, G6).sounding_transposition(MAJ_6);
pub const SAXOPHONE_BB_TENOR: NoteRange = NoteRange::new(Bb3, G6).sounding_transposition(MAJ_9);
pub const SAXOPHONE_EB_BARITONE: NoteRange = NoteRange::new(Bb2, G5).sounding_transposition(MAJ_6);
pub const SAXOPHONE_BB_BASS: NoteRange = NoteRange::new(Bb2, G5).sounding_transposition(MAJ_9);
// ...TODO
pub const GUITAR_RANGE: NoteRange = NoteRange::new(E3, E6).sounding_transposition(PER_8);
// ...TODO
pub const PIANO_RANGE: NoteRange = NoteRange::new(A0, C8);
// ...TODO
pub const VIOLIN_RANGE: NoteRange = NoteRange::new(G3, A7);
pub const VIOLA_RANGE: NoteRange = NoteRange::new(C3, E6);
pub const CELLO_RANGE: NoteRange = NoteRange::new(C2, C6);
pub const DOUBLE_BASS_RANGE: NoteRange = NoteRange::new(C2, C5).sounding_transposition(PER_8);
