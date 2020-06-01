use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};
use rand::prelude::*;

pub const TAU: f64 = 2. * std::f64::consts::PI;

///////////////////////////////////////////////////////////////////////////////////////////////////
// utility fns
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pitch(n: u8) -> f64 {
    2f64.powf(((n as f64) - 69.) / 12.) * 440.
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1. - t) + b * t
}

pub fn clamp(a: f64, b: f64, x: f64) -> f64 {
    x.min(b).max(a)
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// structs
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct Phrase {
    pub data: Vec<u8>,
    pub len: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct PhraseContext {
    pub notes: Vec<u8>, // 0 = rest
    pub sub_pow: u8,    // subdivide
}

#[derive(Debug, Default)]
pub struct PhraseIterator {
    pub ix: usize,
}
impl PhraseIterator {
    pub fn next(&mut self, data: &Vec<u8>, len: &Vec<u8>) -> Option<PhraseNote> {
        let ix = self.ix;
        if ix < data.len() {
            self.ix = ix + 1;
            return Some(PhraseNote {
                ix: data[ix] as usize,
                len: len[ix] as f64,
            });
        }
        None
    }
}

#[derive(Debug)]
pub struct PhraseNote {
    pub ix: usize,
    pub len: f64,
}

#[derive(Debug, Default)]
pub struct VoiceController {
    pub voice_ix: usize,
    pub age: f64,
    pub lifetime: f64,
    pub midi_note: u8,
}
impl VoiceController {
    pub fn step_time(&mut self, life_step: f64) {
        self.age += life_step;
        self.lifetime -= life_step;
    }
}

#[derive(Debug)]
pub struct Voice {
    pub phase: f64,
    pub freq: f64,
    pub amp: f64,
}
impl Voice {
    pub fn new() -> Voice {
        Voice {
            phase: 0.,
            freq: 0.,
            amp: 0.,
        }
    }

    pub fn step_time(
        &mut self,
        note_freq: f64,
        age: f64,
        lifetime: f64,
        t_rel: f64,
        dt_rel: f64,
        vib_hz: f64,
        vib_amp: f64,
        fade_in_t: f64,
        fade_out_t: f64,
        wavetable: &[f64; WAVETABLE_SIZE],
    ) {
        self.freq = note_freq + vib_amp * (t_rel * TAU * vib_hz).sin();
        self.phase = (self.phase + dt_rel * self.freq) % 1.;
        self.amp = f64::min(
            f64::min(
                lerp(0., 1., age / fade_in_t),
                lerp(0., 1., lifetime / fade_out_t),
            ),
            1.,
        ) * wavetable_lerp_sample(&wavetable, self.phase);
    }
}

pub struct SynthSettings {
    pub vib_hz: f64,
    pub vib_amp: f64,
    pub fade_in_t: f64,
    pub fade_out_t: f64,
    pub wavetable: [f64; WAVETABLE_SIZE],
}

#[derive(Debug)]
pub struct Timing {
    pub sample_rate: f64,
    pub sample_num: u64,
    pub time_speed: f64,
    pub dt_rel: f64,
    pub t_rel: f64,
    pub dt_abs: f64, // = inv sample freq
    pub t_abs: f64,
    pub bpm: f64,
    pub life_step: f64,
    pub measures: f64,
}
impl Timing {
    pub fn new(sample_rate: f64, bpm: f64) -> Timing {
        let dt = 1. / sample_rate;
        let life_step = (dt * bpm) as f64 / 4. / 60.;
        Timing {
            sample_rate,
            sample_num: 0,
            time_speed: 1.,
            dt_rel: dt,
            t_rel: 0.,
            dt_abs: dt,
            t_abs: 0.,
            bpm,
            life_step,
            measures: 0.,
        }
    }

    pub fn step(&mut self) {
        self.sample_num = self.sample_num + 1;
        self.t_abs = self.sample_num as f64 * self.dt_abs;
        self.t_rel += self.dt_rel;
        self.measures += self.life_step;
    }
}

#[derive(Debug, Default)]
pub struct MelodyLine {
    phrase: Phrase,
    iter: PhraseIterator,
    beat_countdown: f64,
    voice_controller: VoiceController,
}

const WAVETABLE_SIZE: usize = 1024;
const WAVETABLE_SIZE_F: f64 = 1024.;

pub fn wavetable_from_harmonics(harmonics: Vec<f64>) -> [f64; WAVETABLE_SIZE] {
    // Precompute harmonics normalization factor n.
    let mut n = 0.;
    for amplitude in harmonics.iter() {
        n += amplitude;
    }
    n = 1. / n;

    let harmonics_len = harmonics.len();

    // Fill the wavetable.
    let mut buffer = [0.; WAVETABLE_SIZE];
    for i in 0..buffer.len() {
        for k in 0..harmonics_len {
            const C: f64 = TAU / WAVETABLE_SIZE_F;
            buffer[i] += n * harmonics[k] * (C * (i * (k + 1)) as f64).sin();
        }
    }
    buffer
}

pub fn wavetable_lerp_sample(wavetable: &[f64; WAVETABLE_SIZE], t: f64) -> f64 {
    let ixf = (t - t.floor()) * WAVETABLE_SIZE_F;
    let ix = ixf.floor() as usize % WAVETABLE_SIZE;
    lerp(
        wavetable[ix],
        wavetable[(ix + 1) % WAVETABLE_SIZE],
        ixf - ix as f64,
    )
}

fn main() {
    let host = cpal::default_host();
    let event_loop = host.event_loop();

    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    // setup static data

    let mut timing = Timing::new(format.sample_rate.0 as f64, 120.);
    let synth_0_settings = SynthSettings {
        vib_hz: 4.,
        vib_amp: 2.,
        fade_in_t: 1. / 15.,
        fade_out_t: 1. / 40.,
        wavetable: wavetable_from_harmonics(vec![
            1., 0.75, 0.65, 0.55, 0.5, 0.45, 0.4, 0.35, 0.3, 0.25, 0.25, 0.2,
        ]),
    };
    let mut synth_0_voices = Vec::new();

    synth_0_voices.push(Voice::new());

    let phrase_context = PhraseContext {
        notes: vec![60, 62, 64, 67, 69],
        sub_pow: 2,
    };

    let mut melody_lines = Vec::new();
    for _ in 0..4 {
        let mut m = MelodyLine::default();
        m.voice_controller.voice_ix = synth_0_voices.len();
        melody_lines.push(m);
        synth_0_voices.push(Voice::new());
    }

    let gen_phrase = || {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();
        let mut len = Vec::new();

        let tonic = rng.gen_range(0, 5);

        match rng.gen_range(0, 4) {
            0 | 1 => {
                data.push(tonic);
                len.push(2);
                data.push((tonic + 2) % 5);
                len.push(1);
                data.push((tonic + 4) % 5);
                len.push(1);
            }
            2 => {
                data.push(tonic);
                len.push(4);
            }
            _ => {
                data.push(tonic);
                len.push(3);
                data.push((tonic + 1) % 5);
                len.push(1);
            }
        }

        Phrase { data, len }
    };

    let mut next_value = || -> f64 {
        // progress melodies
        for m in melody_lines.iter_mut() {
            if m.beat_countdown <= 0.0 {
                if let Some(PhraseNote { ix, len }) = m.iter.next(&m.phrase.data, &m.phrase.len) {
                    m.voice_controller = VoiceController {
                        age: 0.,
                        lifetime: len / 2f64.powf(phrase_context.sub_pow as f64),
                        midi_note: phrase_context.notes[ix],
                        voice_ix: m.voice_controller.voice_ix,
                    };
                } else {
                    m.iter = PhraseIterator::default();
                    m.phrase = gen_phrase();
                    assert_ne!(m.phrase.data.len(), 0);
                    if let Some(PhraseNote { ix, len }) = m.iter.next(&m.phrase.data, &m.phrase.len)
                    {
                        m.voice_controller = VoiceController {
                            age: 0.,
                            lifetime: len / 2f64.powf(phrase_context.sub_pow as f64),
                            midi_note: phrase_context.notes[ix],
                            voice_ix: m.voice_controller.voice_ix,
                        };
                    }
                }
                m.beat_countdown = m.voice_controller.lifetime;
            }
        }

        // sum amplitude
        let amp = synth_0_voices.iter().map(|v| v.amp).sum::<f64>() / synth_0_voices.len() as f64;

        // progress time
        timing.step();
        for m in melody_lines.iter_mut() {
            m.voice_controller.step_time(timing.life_step);
            synth_0_voices[m.voice_controller.voice_ix].step_time(
                pitch(m.voice_controller.midi_note),
                m.voice_controller.age,
                m.voice_controller.lifetime,
                timing.t_rel,
                timing.dt_rel,
                synth_0_settings.vib_hz,
                synth_0_settings.vib_amp,
                synth_0_settings.fade_in_t,
                synth_0_settings.fade_out_t,
                &synth_0_settings.wavetable,
            );
            m.beat_countdown -= timing.life_step as f64;
        }

        amp * 0.3 // (about equal to Spotify at full volume)
    };

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f64) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f64) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value() as f32;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            _ => (),
        }
    });
}
