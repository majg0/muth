use super::{BeatDuration, Rhythm};

impl Rhythm {
    pub fn new(bd: BeatDuration) -> Rhythm {
        Rhythm {
            durations: vec![bd],
        }
    }
}
