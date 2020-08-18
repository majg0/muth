use super::{Interval, Note};

impl Note {
    pub fn new(x: i8) -> Note {
        Note(x)
    }

    pub fn pitch(&self) -> f64 {
        const BASE_NOTE_FREQ: f64 = 440.;
        const BASE_NOTE_NUM: f64 = 69.;
        const NOTES_PER_OCTAVE: f64 = 12.;
        2f64.powf(((self.0 as f64) - BASE_NOTE_NUM) / NOTES_PER_OCTAVE) * BASE_NOTE_FREQ
    }

    // TODO: move constness into Add impl https://github.com/rust-lang/rfcs/pull/2632
    pub const fn add_const(self, i: Interval) -> Note {
        Note(self.0 + i.0)
    }
}

impl From<Note> for usize {
    fn from(x: Note) -> usize {
        x.0 as usize
    }
}

impl std::ops::Add<Interval> for Note {
    type Output = Self;

    fn add(self, other: Interval) -> Self {
        Note(self.0 + other.0)
    }
}

impl std::ops::Sub<Interval> for Note {
    type Output = Self;

    fn sub(self, other: Interval) -> Self {
        Note(self.0 - other.0)
    }
}

pub mod note_constants {
    use super::Note;

    pub const MIDI_NOTE_COUNT: usize = 128;

    pub const Csub1: Note = Note(0);
    pub const Cssub1: Note = Note(1);
    pub const Dbsub1: Note = Note(1);
    pub const Dsub1: Note = Note(2);
    pub const Dssub1: Note = Note(3);
    pub const Ebsub1: Note = Note(3);
    pub const Esub1: Note = Note(4);
    pub const Fsub1: Note = Note(5);
    pub const Fssub1: Note = Note(6);
    pub const Gbsub1: Note = Note(6);
    pub const Gsub1: Note = Note(7);
    pub const Gssub1: Note = Note(8);
    pub const Absub1: Note = Note(8);
    pub const Asub1: Note = Note(9);
    pub const Assub1: Note = Note(10);
    pub const Bbsub1: Note = Note(10);
    pub const Bsub1: Note = Note(11);
    pub const C0: Note = Note(12);
    pub const Cs0: Note = Note(13);
    pub const Db0: Note = Note(13);
    pub const D0: Note = Note(14);
    pub const Ds0: Note = Note(15);
    pub const Eb0: Note = Note(15);
    pub const E0: Note = Note(16);
    pub const F0: Note = Note(17);
    pub const Fs0: Note = Note(18);
    pub const Gb0: Note = Note(18);
    pub const G0: Note = Note(19);
    pub const Gs0: Note = Note(20);
    pub const Ab0: Note = Note(20);
    pub const A0: Note = Note(21);
    pub const As0: Note = Note(22);
    pub const Bb0: Note = Note(22);
    pub const B0: Note = Note(23);
    pub const C1: Note = Note(24);
    pub const Cs1: Note = Note(25);
    pub const Db1: Note = Note(25);
    pub const D1: Note = Note(26);
    pub const Ds1: Note = Note(27);
    pub const Eb1: Note = Note(27);
    pub const E1: Note = Note(28);
    pub const F1: Note = Note(29);
    pub const Fs1: Note = Note(30);
    pub const Gb1: Note = Note(30);
    pub const G1: Note = Note(31);
    pub const Gs1: Note = Note(32);
    pub const Ab1: Note = Note(32);
    pub const A1: Note = Note(33);
    pub const As1: Note = Note(34);
    pub const Bb1: Note = Note(34);
    pub const B1: Note = Note(35);
    pub const C2: Note = Note(36);
    pub const Cs2: Note = Note(37);
    pub const Db2: Note = Note(37);
    pub const D2: Note = Note(38);
    pub const Ds2: Note = Note(39);
    pub const Eb2: Note = Note(39);
    pub const E2: Note = Note(40);
    pub const F2: Note = Note(41);
    pub const Fs2: Note = Note(42);
    pub const Gb2: Note = Note(42);
    pub const G2: Note = Note(43);
    pub const Gs2: Note = Note(44);
    pub const Ab2: Note = Note(44);
    pub const A2: Note = Note(45);
    pub const As2: Note = Note(46);
    pub const Bb2: Note = Note(46);
    pub const B2: Note = Note(47);
    pub const C3: Note = Note(48);
    pub const Cs3: Note = Note(49);
    pub const Db3: Note = Note(49);
    pub const D3: Note = Note(50);
    pub const Ds3: Note = Note(51);
    pub const Eb3: Note = Note(51);
    pub const E3: Note = Note(52);
    pub const F3: Note = Note(53);
    pub const Fs3: Note = Note(54);
    pub const Gb3: Note = Note(54);
    pub const G3: Note = Note(55);
    pub const Gs3: Note = Note(56);
    pub const Ab3: Note = Note(56);
    pub const A3: Note = Note(57);
    pub const As3: Note = Note(58);
    pub const Bb3: Note = Note(58);
    pub const B3: Note = Note(59);
    pub const C4: Note = Note(60);
    pub const Cs4: Note = Note(61);
    pub const Db4: Note = Note(61);
    pub const D4: Note = Note(62);
    pub const Ds4: Note = Note(63);
    pub const Eb4: Note = Note(63);
    pub const E4: Note = Note(64);
    pub const F4: Note = Note(65);
    pub const Fs4: Note = Note(66);
    pub const Gb4: Note = Note(66);
    pub const G4: Note = Note(67);
    pub const Gs4: Note = Note(68);
    pub const Ab4: Note = Note(68);
    pub const A4: Note = Note(69);
    pub const As4: Note = Note(70);
    pub const Bb4: Note = Note(70);
    pub const B4: Note = Note(71);
    pub const C5: Note = Note(72);
    pub const Cs5: Note = Note(73);
    pub const Db5: Note = Note(73);
    pub const D5: Note = Note(74);
    pub const Ds5: Note = Note(75);
    pub const Eb5: Note = Note(75);
    pub const E5: Note = Note(76);
    pub const F5: Note = Note(77);
    pub const Fs5: Note = Note(78);
    pub const Gb5: Note = Note(78);
    pub const G5: Note = Note(79);
    pub const Gs5: Note = Note(80);
    pub const Ab5: Note = Note(80);
    pub const A5: Note = Note(81);
    pub const As5: Note = Note(82);
    pub const Bb5: Note = Note(82);
    pub const B5: Note = Note(83);
    pub const C6: Note = Note(84);
    pub const Cs6: Note = Note(85);
    pub const Db6: Note = Note(85);
    pub const D6: Note = Note(86);
    pub const Ds6: Note = Note(87);
    pub const Eb6: Note = Note(87);
    pub const E6: Note = Note(88);
    pub const F6: Note = Note(89);
    pub const Fs6: Note = Note(90);
    pub const Gb6: Note = Note(90);
    pub const G6: Note = Note(91);
    pub const Gs6: Note = Note(92);
    pub const Ab6: Note = Note(92);
    pub const A6: Note = Note(93);
    pub const As6: Note = Note(94);
    pub const Bb6: Note = Note(94);
    pub const B6: Note = Note(95);
    pub const C7: Note = Note(96);
    pub const Cs7: Note = Note(97);
    pub const Db7: Note = Note(97);
    pub const D7: Note = Note(98);
    pub const Ds7: Note = Note(99);
    pub const Eb7: Note = Note(99);
    pub const E7: Note = Note(100);
    pub const F7: Note = Note(101);
    pub const Fs7: Note = Note(102);
    pub const Gb7: Note = Note(102);
    pub const G7: Note = Note(103);
    pub const Gs7: Note = Note(104);
    pub const Ab7: Note = Note(104);
    pub const A7: Note = Note(105);
    pub const As7: Note = Note(106);
    pub const Bb7: Note = Note(106);
    pub const B7: Note = Note(107);
    pub const C8: Note = Note(108);
    pub const Cs8: Note = Note(109);
    pub const Db8: Note = Note(109);
    pub const D8: Note = Note(110);
    pub const Ds8: Note = Note(111);
    pub const Eb8: Note = Note(111);
    pub const E8: Note = Note(112);
    pub const F8: Note = Note(113);
    pub const Fs8: Note = Note(114);
    pub const Gb8: Note = Note(114);
    pub const G8: Note = Note(115);
    pub const Gs8: Note = Note(116);
    pub const Ab8: Note = Note(116);
    pub const A8: Note = Note(117);
    pub const As8: Note = Note(118);
    pub const Bb8: Note = Note(118);
    pub const B8: Note = Note(119);
    pub const C9: Note = Note(120);
    pub const Cs9: Note = Note(121);
    pub const Db9: Note = Note(121);
    pub const D9: Note = Note(122);
    pub const Ds9: Note = Note(123);
    pub const Eb9: Note = Note(123);
    pub const E9: Note = Note(124);
    pub const F9: Note = Note(125);
    pub const Fs9: Note = Note(126);
    pub const Gb9: Note = Note(126);
    pub const G9: Note = Note(127);
}
