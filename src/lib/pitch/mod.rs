mod interval;
mod note;
mod note_range;

pub use interval::*;
pub use note::*;
pub use note_range::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Note(i8);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Interval(i8);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct NoteRange {
    lower_sounding: Note,
    upper_sounding: Note,
    write_transposition: Interval,
}
