use super::interval_constants::{AUG_5, DIM_5, MIN_3};
use super::{Interval, ScaleFamily};
use super::{DIATONIC_COUNT, FIFTH, THIRD};

impl ScaleFamily {
    pub fn new(intervals: &[Interval], names: Vec<String>) -> ScaleFamily {
        ScaleFamily {
            modes: calc_modes(intervals),
            names,
        }
    }

    pub fn major() -> ScaleFamily {
        ScaleFamily::new(
            &scale_family_constants::MAJOR_FAMILY,
            vec![
                String::from("Ionian"),
                String::from("Dorian"),
                String::from("Phrygian"),
                String::from("Lydian"), // NOTE: Lydian might be considered the most major chord, but let's keep it off index 0 anyway.
                String::from("Mixolydian"),
                String::from("Aeolian"),
                String::from("Locrian"),
            ],
        )
    }

    pub fn harmonic_minor() -> ScaleFamily {
        ScaleFamily::new(
            &scale_family_constants::HARMONIC_MINOR_FAMILY,
            vec![
                String::from("Ionian #5"),
                String::from("Dorian #4"),
                String::from("Phrygian Dominant"),
                String::from("Lydian #2"),
                String::from("Superlocrian"),
                String::from("Harmonic Minor"),
                String::from("Locrian #6"),
            ],
        )
    }

    pub fn melodic_minor() -> ScaleFamily {
        ScaleFamily::new(
            &scale_family_constants::MELODIC_MINOR_FAMILY,
            vec![
                String::from("Lydian Augmented"),
                String::from("Lydian Dominant"),
                String::from("Mixolydian b6"),
                String::from("Half-Diminished"),
                String::from("Altered"),
                String::from("Melodic Minor"),
                String::from("Dorian b2"),
            ],
        )
    }

    pub fn roman_chord_name(&self, mode: usize, scale_degree: usize) -> String {
        let mut roman_name = String::from("");

        let expected_offset = scale_family_constants::MAJOR_FAMILY[scale_degree];
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

fn roman_num(mut n: usize) -> String {
    let mut result = String::new();
    for &(name, value) in [("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)].iter() {
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
                .map(|x| (scale[x % scale.len()] - offset).positive_less_than(DIATONIC_COUNT as i8))
                .collect()
        })
        .collect()
}

pub mod scale_family_constants {
    use super::super::interval_constants::{
        DIM_4, DIM_5, MAJ_2, MAJ_3, MAJ_6, MAJ_7, PER_1, PER_4, PER_5,
    };
    use super::super::{Interval, DIATONIC_COUNT};

    pub const MAJOR_FAMILY: [Interval; DIATONIC_COUNT] =
        [PER_1, MAJ_2, MAJ_3, PER_4, PER_5, MAJ_6, MAJ_7];
    pub const HARMONIC_MINOR_FAMILY: [Interval; DIATONIC_COUNT] =
        [PER_1, MAJ_2, MAJ_3, PER_4, DIM_5, MAJ_6, MAJ_7];
    pub const MELODIC_MINOR_FAMILY: [Interval; DIATONIC_COUNT] =
        [PER_1, MAJ_2, MAJ_3, DIM_4, DIM_5, MAJ_6, MAJ_7];
}
