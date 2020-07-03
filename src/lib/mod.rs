#![allow(non_upper_case_globals)]

mod rhythm;
mod pitch;

pub use rhythm::*;
pub use pitch::*;

///////////////////////////////////////////////////////////////////////////////////////////////////
// types
///////////////////////////////////////////////////////////////////////////////////////////////////

/// https://en.wikipedia.org/wiki/Degree_(music)
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Degree(i8);

impl Degree {
    pub fn new(x: i8) -> Degree {
        Degree(x)
    }
}

/// https://en.wikipedia.org/wiki/Pitch_class
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Pc(i8);


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


pub const C: Pc = Pc(0);
pub const Cs: Pc = Pc(1);
pub const Db: Pc = Pc(1);
pub const D: Pc = Pc(2);
pub const Ds: Pc = Pc(3);
pub const Eb: Pc = Pc(3);
pub const E: Pc = Pc(4);
pub const F: Pc = Pc(5);
pub const Fs: Pc = Pc(6);
pub const Gb: Pc = Pc(6);
pub const G: Pc = Pc(7);
pub const Gs: Pc = Pc(8);
pub const Ab: Pc = Pc(8);
pub const A: Pc = Pc(9);
pub const As: Pc = Pc(10);
pub const Bb: Pc = Pc(10);
pub const B: Pc = Pc(11);
pub const Cb: Pc = Pc(11);
pub const NUM_PC_I8: i8 = 12;

pub const MAJOR_FAMILY: [Interval; DIATONIC_COUNT] =
    [PER_1, MAJ_2, MAJ_3, PER_4, PER_5, MAJ_6, MAJ_7];
pub const HARMONIC_MINOR_FAMILY: [Interval; DIATONIC_COUNT] =
    [PER_1, MAJ_2, MAJ_3, PER_4, DIM_5, MAJ_6, MAJ_7];
pub const MELODIC_MINOR_FAMILY: [Interval; DIATONIC_COUNT] =
    [PER_1, MAJ_2, MAJ_3, DIM_4, DIM_5, MAJ_6, MAJ_7];

pub struct ScaleFamily {
    /// Transposed interval stacks.
    pub modes: Vec<Vec<Interval>>,
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

    pub fn chord_scale_0<'a>(&'a self, mode: usize, scale_degree: usize) -> &'a Vec<Interval> {
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

fn calc_modes(scale: &[Interval]) -> Vec<Vec<Interval>> {
    scale
        .iter()
        .enumerate()
        .map(|(i, &offset)| {
            (i..i + scale.len())
                .map(|x| (scale[x % scale.len()] - offset).positive_less_than(NUM_PC_I8))
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
            "Lydian", // NOTE: Lydian might be considered the most major chord, but let's keep it off index 0 anyway.
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
