use std::{error::Error, sync::mpsc, thread};

use cpal::{
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    StreamData, UnknownTypeOutputBuffer,
};

use muth::{BeatTime, MIDI_NOTE_COUNT, Note, BeatDuration, DURATION_MULTIPLIER};

pub const OK_AUDIO_DELAY_MILLISECONDS: u64 = 5;

pub const TAU: f64 = 2. * std::f64::consts::PI;

#[derive(Clone, Copy, Debug, Default)]
pub struct Voice {
    pub phase: f64,
    pub target_freq: f64,
    /// Affected by e.g. vibrato, unlike `target_freq`.
    pub freq: f64,
    /// How hard the note was hit [0,1].
    pub velocity: f64,
    pub amp: f64,
    pub start_beat: BeatTime,
    pub end_beat: BeatTime,
}

pub struct SynthPatch {
    pub vib_hz: f64,
    pub vib_amp: f64,
    pub fade_in_t: f64,
    pub fade_out_t: f64,
    pub wavetable: [f64; WAVETABLE_SIZE],
    pub voices: [Voice; MIDI_NOTE_COUNT],
}

#[derive(Clone, Copy, Debug)]
pub enum SynthCommand {
    NoteOnForDuration(Note, BeatDuration, f64),
}

#[derive(Debug)]
pub struct Timing {
    pub sample_rate: f64,
    pub sample_num: u64,
    pub dt_abs: f64, // = inv sample freq
    pub t_abs: f64,
    pub playback_speed: f64,
    pub dt_rel: f64,
    pub t_rel: f64,
    pub beat: f64,
}
impl Timing {
    pub fn new(sample_rate: f64, playback_speed: f64) -> Timing {
        let dt = 1. / sample_rate;
        Timing {
            sample_rate,
            sample_num: 0,
            playback_speed,
            dt_rel: dt * playback_speed,
            t_rel: 0.,
            dt_abs: dt,
            t_abs: 0.,
            beat: 0.,
        }
    }

    pub fn step(&mut self, bpm: f64) {
        self.sample_num += 1;
        self.t_abs = self.sample_num as f64 * self.dt_abs;
        self.t_rel += self.dt_rel;
        self.beat += (f64::from(DURATION_MULTIPLIER) * self.dt_rel * bpm) / 60.;
    }
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1. - t) + b * t
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

    // Fill the wavetable.
    let mut buffer = [0.; WAVETABLE_SIZE];
    for (i, amplitude) in buffer.iter_mut().enumerate() {
        for (k, harmonic) in harmonics.iter().enumerate() {
            const C: f64 = TAU / WAVETABLE_SIZE_F;
            *amplitude += n * harmonic * (C * (i * (k + 1)) as f64).sin();
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

pub fn run(
    synth_rx: mpsc::Receiver<SynthCommand>,
) -> Result<thread::JoinHandle<()>, Box<dyn Error>> {
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

    let mut timing = Timing::new(format.sample_rate.0 as f64, 1.);
    let bpm = 120.; // TODO: ?

    let mut synth_patches = vec![SynthPatch {
        vib_hz: 4.,
        vib_amp: 2.,
        fade_in_t: 1. / 15.,
        fade_out_t: 1. / 40.,
        wavetable: wavetable_from_harmonics(vec![
            1., 0.75, 0.65, 0.55, 0.5, 0.45, 0.4, 0.35, 0.3, 0.25, 0.25, 0.2,
        ]),
        voices: [Voice::default(); MIDI_NOTE_COUNT],
    }];

    let mut next_value = move || -> f64 {
        let synth_patch = &mut synth_patches[0];

        match synth_rx.try_recv() {
            Ok(cmd) => match cmd {
                SynthCommand::NoteOnForDuration(n, beats, velocity) => {
                    let mut v = &mut synth_patch.voices[usize::from(n)];
                    v.target_freq = n.pitch();
                    v.velocity = velocity;
                    let start_beat = timing.beat.into();
                    v.start_beat = start_beat;
                    v.end_beat = start_beat + beats;
                }
            },
            Err(mpsc::TryRecvError::Empty) => {}
            Err(x) => {
                dbg!(x);
            }
        };

        // update voices
        for voice_ix in 0..MIDI_NOTE_COUNT {
            let mut v = &mut synth_patch.voices[voice_ix];
            if v.target_freq == 0. {
                continue;
            }
            if f64::from(v.end_beat) <= timing.beat {
                continue;
            }
            v.freq = v.target_freq
                + synth_patch.vib_amp * (timing.t_rel * TAU * synth_patch.vib_hz).sin();
            v.phase = (v.phase + timing.dt_rel * v.freq) % 1.;
            v.amp = f64::min(
                f64::min(
                    lerp(
                        0.,
                        1.,
                        (timing.beat - f64::from(v.start_beat))
                            / f64::from(DURATION_MULTIPLIER)
                            / synth_patch.fade_in_t,
                    ),
                    lerp(
                        0.,
                        1.,
                        (f64::from(v.end_beat) - timing.beat)
                            / f64::from(DURATION_MULTIPLIER)
                            / synth_patch.fade_out_t,
                    ),
                ),
                1.,
            ) * wavetable_lerp_sample(&synth_patch.wavetable, v.phase);
        }

        // sum amplitude
        let amp =
            synth_patch.voices.iter().map(|v| v.amp).sum::<f64>() / synth_patch.voices.len() as f64;
        assert!(amp.abs() <= 1.);

        // progress time
        timing.step(bpm);

        amp * 0.9
    };

    let music_thread = thread::Builder::new()
        .name("music".to_string())
        .spawn(move || {
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
        })?;

    Ok(music_thread)
}
