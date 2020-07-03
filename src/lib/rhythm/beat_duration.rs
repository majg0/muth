use super::Subdivision;
use super::BeatDuration;

/******************************************************************************
* CONSTANTS
******************************************************************************/

/// Used in order to be able to express every duration as an integer number.
///
/// minutes per year: 31556926/60
///
/// possible values in u64: 2^64
///
/// maximum multiplier value, if we want to be able to keep running the application with X bpm for Y years (relative time - consider time stretch): 2^64 - 2^64 / (31556926/60 * X * Y)
///
/// e.g. X=300 Y=1000 gives 1.8446744e+19.
pub const DURATION_MULTIPLIER: BeatDuration = BeatDuration(2 * 2 * 2 * 3 * 5); // = quarter note
pub const WHOLE_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 4); // W 𝅝
pub const DOUBLE_DOTTED_HALF_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 7 / 2); // H 𝅗𝅥
pub const DOTTED_HALF_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 3);
pub const HALF_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 2);
pub const HALF_NOTE_TRIPLET: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 4 / 3);
pub const DOUBLE_DOTTED_QUARTER_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 7 / 4); // Q 𝅘𝅥
pub const DOTTED_QUARTER_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 3 / 2);
pub const QUARTER_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0);
pub const QUARTER_NOTE_TRIPLET: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 2 / 3);
pub const DOUBLE_DOTTED_EIGHTH_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 7 / 8); // E 𝅘𝅥𝅮
pub const DOTTED_EIGHTH_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 3 / 4);
pub const EIGHTH_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 2);
pub const EIGHTH_NOTE_TRIPLET: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 3);
pub const DOTTED_SIXTEENTH_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 * 3 / 8); // S 𝅘𝅥𝅯
pub const SIXTEENTH_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 4);
pub const SIXTEENTH_NOTE_TRIPLET: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 6);
pub const THIRTYSECOND_NOTE: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 8); // T 𝅘𝅥𝅰
pub const THIRTYSECOND_NOTE_TRIPLET: BeatDuration = BeatDuration(DURATION_MULTIPLIER.0 / 12);
pub const WN: BeatDuration = WHOLE_NOTE; // W 𝅝
pub const DDHN: BeatDuration = DOUBLE_DOTTED_HALF_NOTE; // H 𝅗𝅥
pub const DHN: BeatDuration = DOTTED_HALF_NOTE;
pub const HN: BeatDuration = HALF_NOTE;
pub const HNT: BeatDuration = HALF_NOTE_TRIPLET;
pub const DDQN: BeatDuration = DOUBLE_DOTTED_QUARTER_NOTE; // Q 𝅘𝅥
pub const DQN: BeatDuration = DOTTED_QUARTER_NOTE;
pub const QN: BeatDuration = QUARTER_NOTE;
pub const QNT: BeatDuration = QUARTER_NOTE_TRIPLET;
pub const DDEN: BeatDuration = DOUBLE_DOTTED_EIGHTH_NOTE; // E 𝅘𝅥𝅮
pub const DEN: BeatDuration = DOTTED_EIGHTH_NOTE;
pub const EN: BeatDuration = EIGHTH_NOTE;
pub const ENT: BeatDuration = EIGHTH_NOTE_TRIPLET;
pub const DSN: BeatDuration = DOTTED_SIXTEENTH_NOTE; // S 𝅘𝅥𝅯
pub const SN: BeatDuration = SIXTEENTH_NOTE;
pub const SNT: BeatDuration = SIXTEENTH_NOTE_TRIPLET;
pub const TN: BeatDuration = THIRTYSECOND_NOTE; // T 𝅘𝅥𝅰
pub const TNT: BeatDuration = THIRTYSECOND_NOTE_TRIPLET;

/******************************************************************************
* IMPLS
******************************************************************************/

impl BeatDuration {
    pub fn subdivided(self, sub: Subdivision) -> Option<Vec<BeatDuration>> {
        if self % sub.total != 0 {
            return None;
        }
        let atom = self / sub.total;
        Some(sub.subs.iter().map(|&sub| atom * sub).collect())
    }
}

impl std::ops::Add<BeatDuration> for BeatDuration {
    type Output = Self;

    fn add(self, other: BeatDuration) -> Self {
        BeatDuration(self.0 + other.0)
    }
}

impl std::ops::Mul<u64> for BeatDuration {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        BeatDuration(self.0 * other)
    }
}

impl std::ops::Div<u64> for BeatDuration {
    type Output = Self;

    fn div(self, other: u64) -> Self {
        BeatDuration(self.0 / other)
    }
}

impl std::ops::Rem<u64> for BeatDuration {
    type Output = u64;

    fn rem(self, other: u64) -> u64 {
        self.0 % other
    }
}

impl From<BeatDuration> for f64 {
    fn from(x: BeatDuration) -> f64 {
        x.0 as f64
    }
}
