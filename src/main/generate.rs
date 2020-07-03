use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use rand::{prelude::*, rngs::SmallRng};

use muth::*;

use super::audio::{SynthCommand, Timing, OK_AUDIO_DELAY_MILLISECONDS};

#[derive(Clone, Copy, Debug)]
struct ChordTrackEvent {
    mode: Degree,
    degree: Degree,
}

#[derive(Debug)]
struct MelodyTrack {
    notes: Vec<Note>,
    beats: Vec<BeatTime>,
    durations: Vec<BeatDuration>,
}

pub fn run(synth_tx: mpsc::Sender<SynthCommand>) {
    // let mut rng = SmallRng::from_entropy();
    let mut rng = SmallRng::seed_from_u64(1337); // TODO: seed from config

    //let mut chord_track_events = Vec::new();
    //let mut chord_track_beats = Vec::new();
    //let mut chord_track_durations = Vec::new();

    let mut melody_tracks = Vec::new();
    melody_tracks.push(MelodyTrack {
        notes: Vec::new(),
        beats: Vec::new(),
        durations: Vec::new(),
    });

    let mut gen_measure = |beat: BeatTime, melody_tracks: &mut Vec<MelodyTrack>| {
        // chord_track_events.push(ChordTrackEvent {
        //     mode: Degree::new(rng.gen_range(0, 7)),
        //     degree: Degree::new(rng.gen_range(0, 7)),
        // });
        // chord_track_beats.push(beat);
        // chord_track_durations.push(WN);

        for mt in melody_tracks.iter_mut() {
            let mut beat = beat;
            mt.notes.push(Note::new(60 + rng.gen_range(0, 7)));
            mt.beats.push(beat);
            mt.durations.push(QN);

            beat += QN;
            mt.notes.push(Note::new(60 + rng.gen_range(0, 7)));
            mt.beats.push(beat);
            mt.durations.push(QN);

            beat += QN;
            mt.notes.push(Note::new(60 + rng.gen_range(0, 7)));
            mt.beats.push(beat);
            mt.durations.push(QN);

            beat += QN;
            mt.notes.push(Note::new(60 + rng.gen_range(0, 7)));
            mt.beats.push(beat);
            mt.durations.push(QN);
        }
    };

    //let major_family = major_family();

    let mut timing = Timing::new(1000. / OK_AUDIO_DELAY_MILLISECONDS as f64, 1.);
    let bpm = 120.;

    let tick_rate = Duration::from_millis(OK_AUDIO_DELAY_MILLISECONDS);
    let mut last_tick = Instant::now() - tick_rate;

    let mut mel_ix = 0;

    loop {
        if last_tick.elapsed() >= tick_rate {
            let mut cmd = None;

            let mt = &mut melody_tracks[0];
            let num_beats = mt.beats.len();
            if num_beats > 0 {
                if mel_ix >= num_beats {
                    let last_ix = num_beats - 1;
                    gen_measure(
                        mt.beats[last_ix] + mt.durations[last_ix],
                        &mut melody_tracks,
                    );
                }
            } else {
                gen_measure(BeatTime::zero(), &mut melody_tracks); // initial case
            }

            let mt = &mut melody_tracks[0];
            if timing.beat >= f64::from(mt.beats[mel_ix]) {
                cmd = Some(SynthCommand::NoteOnForDuration(
                    mt.notes[mel_ix],
                    mt.durations[mel_ix],
                    1.,
                ));
                mel_ix += 1;
            }

            timing.step(bpm); // TODO: I expect this way of working to go out of sync with the audio thread eventually. How should we handle this?

            if let Some(cmd) = cmd {
                if synth_tx.send(cmd).is_err() {
                    break; // NOTE: exiting when disconnected
                }
            }
            last_tick = Instant::now();
        }
    }
}
