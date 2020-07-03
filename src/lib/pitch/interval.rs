use super::Interval;

impl Interval {
    pub fn new(interval: i8) -> Interval {
        Interval(interval)
    }

    pub fn positive_less_than(self, div: i8) -> Interval {
        Interval(self.0.rem_euclid(div))
    }

    // TODO: move constness into Neg impl https://github.com/rust-lang/rfcs/pull/2632
    pub const fn neg_const(self) -> Interval {
        Interval(-self.0)
    }
}

impl std::ops::Add<Interval> for Interval {
    type Output = Self;

    fn add(self, other: Interval) -> Self {
        Interval(self.0 + other.0)
    }
}

impl std::ops::Sub<Interval> for Interval {
    type Output = Self;

    fn sub(self, other: Interval) -> Self {
        Interval(self.0 - other.0)
    }
}

impl std::ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        Interval(-self.0)
    }
}

pub const DESC_PER_15: Interval = Interval(-24);
pub const DESC_MAJ_14: Interval = Interval(-23);
pub const DESC_MIN_14: Interval = Interval(-22);
pub const DESC_MAJ_13: Interval = Interval(-21);
pub const DESC_MIN_13: Interval = Interval(-20);
pub const DESC_AUG_12: Interval = Interval(-20);
pub const DESC_PER_12: Interval = Interval(-19);
pub const DESC_DIM_12: Interval = Interval(-18);
pub const DESC_AUG_11: Interval = Interval(-18);
pub const DESC_PER_11: Interval = Interval(-17);
pub const DESC_MAJ_10: Interval = Interval(-16);
pub const DESC_MIN_10: Interval = Interval(-15);
pub const DESC_MAJ_9: Interval = Interval(-14);
pub const DESC_MIN_9: Interval = Interval(-13);
pub const DESC_PER_8: Interval = Interval(-12);
pub const DESC_MAJ_7: Interval = Interval(-11);
pub const DESC_MIN_7: Interval = Interval(-10);
pub const DESC_MAJ_6: Interval = Interval(-9);
pub const DESC_MIN_6: Interval = Interval(-8);
pub const DESC_AUG_5: Interval = Interval(-8);
pub const DESC_PER_5: Interval = Interval(-7);
pub const DESC_DIM_5: Interval = Interval(-6);
pub const DESC_AUG_4: Interval = Interval(-6);
pub const DESC_PER_4: Interval = Interval(-5);
pub const DESC_MAJ_3: Interval = Interval(-4);
pub const DESC_MIN_3: Interval = Interval(-3);
pub const DESC_MAJ_2: Interval = Interval(-2);
pub const DESC_MIN_2: Interval = Interval(-1);
pub const PER_1: Interval = Interval(0);
pub const MIN_2: Interval = Interval(1);
pub const MAJ_2: Interval = Interval(2);
pub const MIN_3: Interval = Interval(3);
pub const MAJ_3: Interval = Interval(4);
pub const DIM_4: Interval = Interval(5);
pub const PER_4: Interval = Interval(5);
pub const AUG_4: Interval = Interval(6);
pub const DIM_5: Interval = Interval(6);
pub const PER_5: Interval = Interval(7);
pub const AUG_5: Interval = Interval(8);
pub const MIN_6: Interval = Interval(8);
pub const MAJ_6: Interval = Interval(9);
pub const MIN_7: Interval = Interval(10);
pub const MAJ_7: Interval = Interval(11);
pub const PER_8: Interval = Interval(12);
pub const MIN_9: Interval = Interval(13);
pub const MAJ_9: Interval = Interval(14);
pub const MIN_10: Interval = Interval(15);
pub const MAJ_10: Interval = Interval(16);
pub const PER_11: Interval = Interval(17);
pub const AUG_11: Interval = Interval(18);
pub const DIM_12: Interval = Interval(18);
pub const PER_12: Interval = Interval(19);
pub const AUG_12: Interval = Interval(20);
pub const MIN_13: Interval = Interval(20);
pub const MAJ_13: Interval = Interval(21);
pub const MIN_14: Interval = Interval(22);
pub const MAJ_14: Interval = Interval(23);
pub const PER_15: Interval = Interval(24);
