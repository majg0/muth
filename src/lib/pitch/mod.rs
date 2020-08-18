mod chromatic_range;
mod degree;
mod interval;
mod note;
mod pitch_class;
mod scale_family;

pub use chromatic_range::*;
pub use degree::*;
pub use interval::*;
pub use note::*;
pub use pitch_class::*;
pub use scale_family::*;

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

/// https://en.wikipedia.org/wiki/Degree_(music)
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Degree(i8);

/// https://en.wikipedia.org/wiki/Interval_(music)
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Interval(i8);

/// https://en.wikipedia.org/wiki/Musical_note
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Note(i8);

/// https://en.wikipedia.org/wiki/Range_(music)
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct ChromaticRange {
    lower_sounding: Note,
    upper_sounding: Note,
    write_transposition: Interval,
}

/// https://en.wikipedia.org/wiki/Pitch_class
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct PitchClass(i8);

pub struct ScaleFamily {
    /// Transposed interval stacks.
    pub modes: Vec<Vec<Interval>>,
    pub names: Vec<String>,
}
