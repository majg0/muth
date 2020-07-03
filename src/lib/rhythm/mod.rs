mod beat_duration;
mod beat_time;
mod rhythm;
mod subdivision;

pub use beat_duration::*;
pub use beat_time::*;
pub use rhythm::*;
pub use subdivision::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct BeatDuration(u64);

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct BeatTime(u64);

pub struct Rhythm {
    durations: Vec<BeatDuration>,
}

pub struct Subdivision {
    total: u64,
    subs: Vec<u64>,
}
