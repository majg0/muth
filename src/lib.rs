#![allow(non_upper_case_globals)]

///////////////////////////////////////////////////////////////////////////////////////////////////
// type aliases
///////////////////////////////////////////////////////////////////////////////////////////////////

/// https://en.wikipedia.org/wiki/Degree_(music)
pub type Degree = i8;
/// https://en.wikipedia.org/wiki/Pitch_class
pub type Pc = i8;
pub type Note = i8;
pub type Interval = i8;
pub type BeatTime = u64;

///////////////////////////////////////////////////////////////////////////////////////////////////
// constants
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const DIATONIC_COUNT: usize = 7;
pub const CHROMATIC_COUNT: usize = 12;

pub const FIRST: usize = 0;
pub const SECOND: usize = 1;
pub const THIRD: usize = 2;
pub const FOURTH: usize = 3;
pub const FIFTH: usize = 4;
pub const SIXTH: usize = 5;
pub const SEVENTH: usize = 6;

pub const IONIAN: usize = 0;
pub const DORIAN: usize = 1;
pub const PHRYGIAN: usize = 2;
pub const LYDIAN: usize = 3;
pub const MIXOLYDIAN: usize = 4;
pub const AEOLIAN: usize = 5;
pub const LOCRIAN: usize = 6;

pub const DESC_PER_15: Interval = -24;
pub const DESC_MAJ_14: Interval = -23;
pub const DESC_MIN_14: Interval = -22;
pub const DESC_MAJ_13: Interval = -21;
pub const DESC_MIN_13: Interval = -20;
pub const DESC_AUG_12: Interval = -20;
pub const DESC_PER_12: Interval = -19;
pub const DESC_DIM_12: Interval = -18;
pub const DESC_AUG_11: Interval = -18;
pub const DESC_PER_11: Interval = -17;
pub const DESC_MAJ_10: Interval = -16;
pub const DESC_MIN_10: Interval = -15;
pub const DESC_MAJ_9: Interval = -14;
pub const DESC_MIN_9: Interval = -13;
pub const DESC_PER_8: Interval = -12;
pub const DESC_MAJ_7: Interval = -11;
pub const DESC_MIN_7: Interval = -10;
pub const DESC_MAJ_6: Interval = -9;
pub const DESC_MIN_6: Interval = -8;
pub const DESC_AUG_5: Interval = -8;
pub const DESC_PER_5: Interval = -7;
pub const DESC_DIM_5: Interval = -6;
pub const DESC_AUG_4: Interval = -6;
pub const DESC_PER_4: Interval = -5;
pub const DESC_MAJ_3: Interval = -4;
pub const DESC_MIN_3: Interval = -3;
pub const DESC_MAJ_2: Interval = -2;
pub const DESC_MIN_2: Interval = -1;
pub const PER_1: Interval = 0;
pub const MIN_2: Interval = 1;
pub const MAJ_2: Interval = 2;
pub const MIN_3: Interval = 3;
pub const MAJ_3: Interval = 4;
pub const PER_4: Interval = 5;
pub const AUG_4: Interval = 6;
pub const DIM_5: Interval = 6;
pub const PER_5: Interval = 7;
pub const AUG_5: Interval = 8;
pub const MIN_6: Interval = 8;
pub const MAJ_6: Interval = 9;
pub const MIN_7: Interval = 10;
pub const MAJ_7: Interval = 11;
pub const PER_8: Interval = 12;
pub const MIN_9: Interval = 13;
pub const MAJ_9: Interval = 14;
pub const MIN_10: Interval = 15;
pub const MAJ_10: Interval = 16;
pub const PER_11: Interval = 17;
pub const AUG_11: Interval = 18;
pub const DIM_12: Interval = 18;
pub const PER_12: Interval = 19;
pub const AUG_12: Interval = 20;
pub const MIN_13: Interval = 20;
pub const MAJ_13: Interval = 21;
pub const MIN_14: Interval = 22;
pub const MAJ_14: Interval = 23;
pub const PER_15: Interval = 24;

pub const C: Pc = 0;
pub const Cs: Pc = 1;
pub const Db: Pc = 1;
pub const D: Pc = 2;
pub const Ds: Pc = 3;
pub const Eb: Pc = 3;
pub const E: Pc = 4;
pub const F: Pc = 5;
pub const Fs: Pc = 6;
pub const Gb: Pc = 6;
pub const G: Pc = 7;
pub const Gs: Pc = 8;
pub const Ab: Pc = 8;
pub const A: Pc = 9;
pub const As: Pc = 10;
pub const Bb: Pc = 10;
pub const B: Pc = 11;
pub const Cb: Pc = 11;
pub const NUM_PC: Pc = 12;

pub const MAJOR_FAMILY: [Pc; DIATONIC_COUNT] = [C, D, E, F, G, A, B];
pub const HARMONIC_MINOR_FAMILY: [Pc; DIATONIC_COUNT] = [C, D, E, F, Gs, A, B];
pub const MELODIC_MINOR_FAMILY: [Pc; DIATONIC_COUNT] = [C, D, E, Fs, Gs, A, B];

pub const Csub1: Note = 0;
pub const Dbsub1: Note = 1;
pub const Dsub1: Note = 2;
pub const Ebsub1: Note = 3;
pub const Esub1: Note = 4;
pub const Fsub1: Note = 5;
pub const Gbsub1: Note = 6;
pub const Gsub1: Note = 7;
pub const Absub1: Note = 8;
pub const Asub1: Note = 9;
pub const Bbsub1: Note = 10;
pub const Bsub1: Note = 11;
pub const C0: Note = 12;
pub const Db0: Note = 13;
pub const D0: Note = 14;
pub const Eb0: Note = 15;
pub const E0: Note = 16;
pub const F0: Note = 17;
pub const Gb0: Note = 18;
pub const G0: Note = 19;
pub const Ab0: Note = 20;
pub const A0: Note = 21;
pub const Bb0: Note = 22;
pub const B0: Note = 23;
pub const C1: Note = 24;
pub const Db1: Note = 25;
pub const D1: Note = 26;
pub const Eb1: Note = 27;
pub const E1: Note = 28;
pub const F1: Note = 29;
pub const Gb1: Note = 30;
pub const G1: Note = 31;
pub const Ab1: Note = 32;
pub const A1: Note = 33;
pub const Bb1: Note = 34;
pub const B1: Note = 35;
pub const C2: Note = 36;
pub const Db2: Note = 37;
pub const D2: Note = 38;
pub const Eb2: Note = 39;
pub const E2: Note = 40;
pub const F2: Note = 41;
pub const Gb2: Note = 42;
pub const G2: Note = 43;
pub const Ab2: Note = 44;
pub const A2: Note = 45;
pub const Bb2: Note = 46;
pub const B2: Note = 47;
pub const C3: Note = 48;
pub const Db3: Note = 49;
pub const D3: Note = 50;
pub const Eb3: Note = 51;
pub const E3: Note = 52;
pub const F3: Note = 53;
pub const Gb3: Note = 54;
pub const G3: Note = 55;
pub const Ab3: Note = 56;
pub const A3: Note = 57;
pub const Bb3: Note = 58;
pub const B3: Note = 59;
pub const C4: Note = 60;
pub const Db4: Note = 61;
pub const D4: Note = 62;
pub const Eb4: Note = 63;
pub const E4: Note = 64;
pub const F4: Note = 65;
pub const Gb4: Note = 66;
pub const G4: Note = 67;
pub const Ab4: Note = 68;
pub const A4: Note = 69;
pub const Bb4: Note = 70;
pub const B4: Note = 71;
pub const C5: Note = 72;
pub const Db5: Note = 73;
pub const D5: Note = 74;
pub const Eb5: Note = 75;
pub const E5: Note = 76;
pub const F5: Note = 77;
pub const Gb5: Note = 78;
pub const G5: Note = 79;
pub const Ab5: Note = 80;
pub const A5: Note = 81;
pub const Bb5: Note = 82;
pub const B5: Note = 83;
pub const C6: Note = 84;
pub const Db6: Note = 85;
pub const D6: Note = 86;
pub const Eb6: Note = 87;
pub const E6: Note = 88;
pub const F6: Note = 89;
pub const Gb6: Note = 90;
pub const G6: Note = 91;
pub const Ab6: Note = 92;
pub const A6: Note = 93;
pub const Bb6: Note = 94;
pub const B6: Note = 95;
pub const C7: Note = 96;
pub const Db7: Note = 97;
pub const D7: Note = 98;
pub const Eb7: Note = 99;
pub const E7: Note = 100;
pub const F7: Note = 101;
pub const Gb7: Note = 102;
pub const G7: Note = 103;
pub const Ab7: Note = 104;
pub const A7: Note = 105;
pub const Bb7: Note = 106;
pub const B7: Note = 107;
pub const C8: Note = 108;
pub const Db8: Note = 109;
pub const D8: Note = 110;
pub const Eb8: Note = 111;
pub const E8: Note = 112;
pub const F8: Note = 113;
pub const Gb8: Note = 114;
pub const G8: Note = 115;
pub const Ab8: Note = 116;
pub const A8: Note = 117;
pub const Bb8: Note = 118;
pub const B8: Note = 119;
pub const C9: Note = 120;
pub const Db9: Note = 121;
pub const D9: Note = 122;
pub const Eb9: Note = 123;
pub const E9: Note = 124;
pub const F9: Note = 125;
pub const Gb9: Note = 126;
pub const G9: Note = 127;

// https://en.wikipedia.org/wiki/Vocal_range
pub const FEMALE_SOPRANO_RANGE: (Note, Note) = (C4, C6);
pub const FEMALE_MEZZO_SOPRANO_RANGE: (Note, Note) = (A3, A5);
pub const FEMALE_ALTO_RANGE: (Note, Note) = (F3, E5);
pub const MALE_TENOR_RANGE: (Note, Note) = (B2, A4);
pub const MALE_BARYTON_RANGE: (Note, Note) = (G2, F4);
pub const MALE_BASS_RANGE: (Note, Note) = (E2, E4);
// http://www.orchestralibrary.com/reftables/rang.html
pub const PICCOLO_RANGE: (Note, Note) = (D4 + PER_8, C7 + PER_8);
pub const FLUTE_RANGE: (Note, Note) = (C4, D7);
pub const ALTO_FLUTE_RANGE: (Note, Note) = (C4 - PER_4, C7 - PER_4);
pub const OBOE_RANGE: (Note, Note) = (Bb3, A6);
pub const OBOE_D_AMORE_RANGE: (Note, Note) = (Bb3 - MIN_3, E6 - MIN_3);
pub const ENGLISH_HORN_RANGE: (Note, Note) = (B3 - PER_5, G6 - PER_5);
pub const BASS_OBOE_RANGE: (Note, Note) = (A3 - PER_8, G6 - PER_8);
pub const CLARINET_BB_RANGE: (Note, Note) = (E3 - MAJ_2, C7 - MAJ_2);
pub const CLARINET_A_RANGE: (Note, Note) = (E3 - MIN_3, C7 - MIN_3);
pub const CLARINET_D_RANGE: (Note, Note) = (E3 + MAJ_2, C7 + MAJ_2);
pub const CLARINET_EB_RANGE: (Note, Note) = (E3 + MIN_3, C7 + MIN_3);
pub const BASSET_HORN_RANGE: (Note, Note) = (C3 - PER_5, G6 - PER_5);
pub const BASS_CLARINET_BB: (Note, Note) = (Eb3 - MAJ_9, G6 - MAJ_9);
pub const BASSOON: (Note, Note) = (Bb1, Eb5);
pub const CONTRABASSOON: (Note, Note) = (Bb1 - PER_8, Bb4 - PER_8);
pub const SAXOPHONE_BB_SOPRANO: (Note, Note) = (Bb3 - MAJ_2, G6 - MAJ_2);
pub const SAXOPHONE_EB_ALTO: (Note, Note) = (Bb3 - MAJ_6, G6 - MAJ_6);
pub const SAXOPHONE_BB_TENOR: (Note, Note) = (Bb3 - MAJ_9, G6 - MAJ_9);
pub const SAXOPHONE_EB_BARITONE: (Note, Note) = (Bb3 - PER_8 - MAJ_6, G6 - PER_8 - MAJ_6);
pub const SAXOPHONE_BB_BASS: (Note, Note) = (Bb3 - PER_8 - MAJ_9, G6 - PER_8 - MAJ_9);
// ...TODO
pub const GUITAR_RANGE: (Note, Note) = (E3 - PER_8, E6 - PER_8);
// ...TODO
pub const PIANO_RANGE: (Note, Note) = (A0, C8);
// ...TODO
pub const VIOLIN_RANGE: (Note, Note) = (G3, A7);
pub const VIOLA_RANGE: (Note, Note) = (C3, E6);
pub const CELLO_RANGE: (Note, Note) = (C2, C6);
pub const DOUBLE_BASS_RANGE: (Note, Note) = (C2 - PER_8, C5 - PER_8);

/// Used in order to be able to express every duration as an integer number.
/// ```
/// 31556926/60 // minutes per year
/// 2^64 // possible values in u64
/// 2^64 - 2^64 / (31556926/60 * X * Y) // maximum multiplier value, if we want to be able to keep running the application with X bpm for Y years (relative time - consider time stretch);
/// // e.g. X=300 Y=1000 gives 1.8446744e+19.
/// ```
pub const DURATION_MULTIPLIER: BeatTime = 2 * 2 * 2 * 3 * 5; // = quarter note
pub const WHOLE_NOTE: BeatTime = DURATION_MULTIPLIER * 4; // W 𝅝
pub const DOUBLE_DOTTED_HALF_NOTE: BeatTime = DURATION_MULTIPLIER * 7 / 2; // H 𝅗𝅥
pub const DOTTED_HALF_NOTE: BeatTime = DURATION_MULTIPLIER * 3;
pub const HALF_NOTE: BeatTime = DURATION_MULTIPLIER * 2;
pub const HALF_NOTE_TRIPLET: BeatTime = DURATION_MULTIPLIER * 4 / 3;
pub const DOUBLE_DOTTED_QUARTER_NOTE: BeatTime = DURATION_MULTIPLIER * 7 / 4; // Q 𝅘𝅥
pub const DOTTED_QUARTER_NOTE: BeatTime = DURATION_MULTIPLIER * 3 / 2;
pub const QUARTER_NOTE: BeatTime = DURATION_MULTIPLIER;
pub const QUARTER_NOTE_TRIPLET: BeatTime = DURATION_MULTIPLIER * 2 / 3;
pub const DOUBLE_DOTTED_EIGHTH_NOTE: BeatTime = DURATION_MULTIPLIER * 7 / 8; // E 𝅘𝅥𝅮
pub const DOTTED_EIGHTH_NOTE: BeatTime = DURATION_MULTIPLIER * 3 / 4;
pub const EIGHTH_NOTE: BeatTime = DURATION_MULTIPLIER / 2;
pub const EIGHTH_NOTE_TRIPLET: BeatTime = DURATION_MULTIPLIER / 3;
pub const DOTTED_SIXTEENTH_NOTE: BeatTime = DURATION_MULTIPLIER * 3 / 8; // S 𝅘𝅥𝅯
pub const SIXTEENTH_NOTE: BeatTime = DURATION_MULTIPLIER / 4;
pub const SIXTEENTH_NOTE_TRIPLET: BeatTime = DURATION_MULTIPLIER / 6;
pub const THIRTYSECOND_NOTE: BeatTime = DURATION_MULTIPLIER / 8; // T 𝅘𝅥𝅰
pub const THIRTYSECOND_NOTE_TRIPLET: BeatTime = DURATION_MULTIPLIER / 12;
pub const WN: BeatTime = WHOLE_NOTE; // W 𝅝
pub const DDHN: BeatTime = DOUBLE_DOTTED_HALF_NOTE; // H 𝅗𝅥
pub const DHN: BeatTime = DOTTED_HALF_NOTE;
pub const HN: BeatTime = HALF_NOTE;
pub const HNT: BeatTime = HALF_NOTE_TRIPLET;
pub const DDQN: BeatTime = DOUBLE_DOTTED_QUARTER_NOTE; // Q 𝅘𝅥
pub const DQN: BeatTime = DOTTED_QUARTER_NOTE;
pub const QN: BeatTime = QUARTER_NOTE;
pub const QNT: BeatTime = QUARTER_NOTE_TRIPLET;
pub const DDEN: BeatTime = DOUBLE_DOTTED_EIGHTH_NOTE; // E 𝅘𝅥𝅮
pub const DEN: BeatTime = DOTTED_EIGHTH_NOTE;
pub const EN: BeatTime = EIGHTH_NOTE;
pub const ENT: BeatTime = EIGHTH_NOTE_TRIPLET;
pub const DSN: BeatTime = DOTTED_SIXTEENTH_NOTE; // S 𝅘𝅥𝅯
pub const SN: BeatTime = SIXTEENTH_NOTE;
pub const SNT: BeatTime = SIXTEENTH_NOTE_TRIPLET;
pub const TN: BeatTime = THIRTYSECOND_NOTE; // T 𝅘𝅥𝅰
pub const TNT: BeatTime = THIRTYSECOND_NOTE_TRIPLET;

pub struct ScaleFamily {
    pub modes: Vec<Vec<Pc>>,
    pub names: Vec<&'static str>,
}

impl ScaleFamily {
    pub fn roman_chord_name(&self, mode: usize, scale_degree: usize) -> String {
        let mut roman_name = String::from("");

        let expected_offset = MAJOR_FAMILY[scale_degree];
        let actual_offset = self.modes[mode][scale_degree];
        if actual_offset > expected_offset {
            roman_name.push('#');
        } else if expected_offset > actual_offset {
            roman_name.push('b');
        }

        roman_name.push_str(&roman_num(scale_degree + 1));

        let chord_degree = (mode + scale_degree) % DIATONIC_COUNT;
        let chord_scale = &self.modes[chord_degree];
        if chord_scale[THIRD] == MIN_3 {
            roman_name.make_ascii_lowercase();
            if chord_scale[FIFTH] == DIM_5 {
                roman_name.push('°');
            }
        } else if chord_scale[FIFTH] == AUG_5 {
            roman_name.push('⁺');
        }

        roman_name
    }

    pub fn chord_scale_0<'a>(&'a self, mode: usize, scale_degree: usize) -> &'a Vec<Pc> {
        &self.modes[(mode + scale_degree) % DIATONIC_COUNT]
    }
}

pub fn roman_num(mut n: usize) -> String {
    let mut result = String::new();
    for &(name, value) in [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ]
    .iter()
    {
        while n >= value {
            n -= value;
            result.push_str(name);
        }
    }
    result
}

fn calc_modes(scale: &[Pc]) -> Vec<Vec<Pc>> {
    scale
        .iter()
        .enumerate()
        .map(|(i, offset)| {
            (i..i + scale.len())
                .map(|x| (scale[x % scale.len()] + NUM_PC - offset) % NUM_PC)
                .collect()
        })
        .collect()
}

pub fn major_family() -> ScaleFamily {
    ScaleFamily {
        modes: calc_modes(&MAJOR_FAMILY[..]),
        names: vec![
            "Ionian",
            "Dorian",
            "Phrygian",
            "Lydian", // NOTE(MG): IMO, Lydian is the proper major chord, but let's keep it off index 0 anyway.
            "Mixolydian",
            "Aeolian",
            "Locrian",
        ],
    }
}

pub fn harmonic_minor_family() -> ScaleFamily {
    ScaleFamily {
        modes: calc_modes(&HARMONIC_MINOR_FAMILY[..]),
        names: vec![
            "Ionian #5",
            "Dorian #4",
            "Phrygian Dominant",
            "Lydian #2",
            "Superlocrian",
            "Harmonic Minor",
            "Locrian #6",
        ],
    }
}

pub fn melodic_minor_family() -> ScaleFamily {
    ScaleFamily {
        modes: calc_modes(&MELODIC_MINOR_FAMILY[..]),
        names: vec![
            "Lydian Augmented",
            "Lydian Dominant",
            "Mixolydian b6",
            "Half-Diminished",
            "Altered",
            "Melodic Minor",
            "Dorian b2",
        ],
    }
}

// NOTE(MG): If we're going to be able to use more families, we should probably compute them instead and tie them all together somehow. We're not really interested in names; we want to produce interesting music. Is there some better theory for handling scales? Can dissonance analysis help? Can Hindemith's system help?
