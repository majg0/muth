use super::{BeatDuration, BeatTime};

impl BeatTime {
    pub fn zero() -> BeatTime {
        BeatTime(0)
    }
}

impl From<BeatTime> for f64 {
    fn from(x: BeatTime) -> f64 {
        x.0 as f64
    }
}

impl From<f64> for BeatTime {
    fn from(x: f64) -> BeatTime {
        BeatTime(x as u64)
    }
}

impl std::ops::Add<BeatDuration> for BeatTime {
    type Output = Self;

    fn add(self, other: BeatDuration) -> Self {
        BeatTime(self.0 + other.0)
    }
}

impl std::ops::Mul<u64> for BeatTime {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        BeatTime(self.0 * other)
    }
}

impl std::ops::Div<u64> for BeatTime {
    type Output = Self;

    fn div(self, other: u64) -> Self {
        BeatTime(self.0 / other)
    }
}

impl std::ops::AddAssign<BeatDuration> for BeatTime {
    fn add_assign(&mut self, x: BeatDuration) {
        self.0 += x.0;
    }
}
