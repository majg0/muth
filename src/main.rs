///////////////////////////////////////////////////////////////////////////////////////////////////
// imports
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::DOT,
    widgets::{Block, Borders, List, Tabs, Text},
    Terminal,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode, KeyEvent,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use cpal::{
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    StreamData, UnknownTypeOutputBuffer,
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use rand::{prelude::*, rngs::SmallRng};

use muth::*;

///////////////////////////////////////////////////////////////////////////////////////////////////
// constants
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const TAU: f64 = 2. * std::f64::consts::PI;

///////////////////////////////////////////////////////////////////////////////////////////////////
// utility fns
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pitch(n: Note) -> f64 {
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
    pub degrees: Vec<Degree>,
    pub durations: Vec<BeatTime>,
}

#[derive(Debug, Default)]
pub struct VoiceController {
    pub voice_ix: usize,
    pub start_beat: BeatTime,
    pub end_beat: BeatTime,
    pub midi_note: Option<Note>,
}

#[derive(Debug, Default)]
pub struct Voice {
    pub phase: f64,
    pub freq: f64,
    pub amp: f64,
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
    pub beat: f64,
}
impl Timing {
    pub fn new(sample_rate: f64, bpm: f64) -> Timing {
        let dt = 1. / sample_rate;
        let time_speed = 1.0;
        Timing {
            sample_rate,
            sample_num: 0,
            time_speed,
            dt_rel: dt * time_speed,
            t_rel: 0.,
            dt_abs: dt,
            t_abs: 0.,
            bpm,
            beat: 0.,
        }
    }

    pub fn step(&mut self) {
        self.sample_num = self.sample_num + 1;
        self.t_abs = self.sample_num as f64 * self.dt_abs;
        self.t_rel += self.dt_rel;
        self.beat += (DURATION_MULTIPLIER as f64 * self.dt_rel * self.bpm) / 60.;
    }
}

#[derive(Debug, Default)]
pub struct MelodyLine {
    phrase: Phrase,
    note_ix: usize,
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

enum InputEvent {
    Key(KeyEvent),
    Tick,
}

#[derive(Clone, Copy)]
enum InputCommand {
    Quit,
}

fn main() -> Result<(), Box<dyn Error>> {
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

    let mut melody_lines = Vec::new();
    for _ in 0..4 {
        let mut m = MelodyLine::default();
        m.voice_controller.voice_ix = synth_0_voices.len();
        melody_lines.push(m);
        synth_0_voices.push(Voice::default());
    }

    let mut chords_data = Vec::new();
    let mut chords_len = Vec::new();
    let mut chord_end_beat = 0;
    let mut chord_ix = 0;

    //let mut rng = SmallRng::seed_from_u64(1337);
    let mut rng = SmallRng::from_entropy();

    let major_family = major_family();

    let gen_chords = |rng: &mut SmallRng| -> (Vec<(Pc, Pc)>, Vec<BeatTime>) {
        let first = rng.gen_range(0, 7);
        let borrow_mode = rng.gen_range(1, 7);
        let borrow_ix = rng.gen_range(1, 3);
        (
            vec![
                (0, first),
                (
                    if borrow_ix == 1 { borrow_mode } else { 0 },
                    (first + 5) % 7,
                ),
                (
                    if borrow_ix == 2 { borrow_mode } else { 0 },
                    (first + 10) % 7,
                ),
                (0, (first + 15) % 7),
            ],
            vec![WN, WN, WN, WN],
        )
    };

    let gen_phrase = |rng: &mut SmallRng| {
        let mut degrees = Vec::new();
        let mut durations = Vec::new();

        match rng.gen_range(0, 3) {
            0 => {
                degrees.push(0);
                durations.push(QN);
                degrees.push(2);
                durations.push(QN);
                degrees.push(4);
                durations.push(QN);
                degrees.push(2);
                durations.push(QN);
            }
            1 => {
                degrees.push(0);
                durations.push(DEN);
                degrees.push(1);
                durations.push(SN);
                degrees.push(2);
                durations.push(DEN);
                degrees.push(3);
                durations.push(SN);
                degrees.push(4);
                durations.push(EN);
                degrees.push(5);
                durations.push(EN);
                degrees.push(6);
                durations.push(EN);
                degrees.push(4);
                durations.push(EN);
            }
            _ => {
                degrees.push(0);
                durations.push(WN);
            }
        }

        Phrase { degrees, durations }
    };

    let mut next_value = move || -> f64 {
        // progress chords
        if timing.beat as u64 >= chord_end_beat {
            chord_ix += 1;
            if chord_ix >= chords_data.len() {
                chord_ix = 0;
                let chords = gen_chords(&mut rng);
                chords_data = chords.0;
                chords_len = chords.1;
                // for &(mode, degree) in chords_data.iter() {
                //     print!(
                //         "{} ",
                //         major_family.roman_chord_name(mode as usize, degree as usize)
                //     );
                // }
                // println!();
            }
            chord_end_beat += chords_len[chord_ix];
        }

        // progress melodies
        for m in melody_lines.iter_mut() {
            if timing.beat as u64 >= m.voice_controller.end_beat {
                m.note_ix = m.note_ix + 1;
                if m.note_ix >= m.phrase.degrees.len() {
                    m.note_ix = 0;
                    m.phrase = gen_phrase(&mut rng);
                    assert_ne!(m.phrase.degrees.len(), 0);
                }
                let ix = m.phrase.degrees[m.note_ix] as usize;
                let duration = m.phrase.durations[m.note_ix];
                assert!(ix < DIATONIC_COUNT);
                m.voice_controller = VoiceController {
                    // start_beat: m.voice_controller.end_beat,
                    // end_beat: m.voice_controller.end_beat
                    start_beat: timing.beat as u64,
                    end_beat: timing.beat as u64 + duration, // TODO subdivision working correctly
                    midi_note: if ix == 255 {
                        None
                    } else {
                        let chord = chords_data[chord_ix];
                        Some(
                            major_family.modes[(chord.0 + chord.1) as usize % DIATONIC_COUNT][ix]
                                + C4
                                + major_family.modes[chord.0 as usize][chord.1 as usize],
                        )
                    },
                    voice_ix: m.voice_controller.voice_ix,
                };
            }
        }

        // update voices
        for m in melody_lines.iter_mut() {
            if m.voice_controller.midi_note.is_some() {
                let mut v = &mut synth_0_voices[m.voice_controller.voice_ix];
                v.freq = pitch(m.voice_controller.midi_note.unwrap())
                    + synth_0_settings.vib_amp
                        * (timing.t_rel * TAU * synth_0_settings.vib_hz).sin();
                v.phase = (v.phase + timing.dt_rel * v.freq) % 1.;
                v.amp = f64::min(
                    f64::min(
                        lerp(
                            0.,
                            1.,
                            (timing.beat - m.voice_controller.start_beat as f64)
                                / DURATION_MULTIPLIER as f64
                                / synth_0_settings.fade_in_t,
                        ),
                        lerp(
                            0.,
                            1.,
                            (m.voice_controller.end_beat as f64 - timing.beat)
                                / DURATION_MULTIPLIER as f64
                                / synth_0_settings.fade_out_t,
                        ),
                    ),
                    1.,
                ) * wavetable_lerp_sample(&synth_0_settings.wavetable, v.phase);
            }
        }

        // sum amplitude
        let amp = synth_0_voices.iter().map(|v| v.amp).sum::<f64>() / synth_0_voices.len() as f64;
        assert!(amp.abs() <= 1.);

        // progress time
        timing.step();

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

    // UI

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?; // NOTE: works natively but not in vscode terminal

    let (input_tx, input_rx) = mpsc::channel();
    let (input_kill_tx, input_kill_rx) = mpsc::channel();
    let input_thread = thread::Builder::new()
        .name("input".to_string())
        .spawn(move || {
            let tick_rate = Duration::from_millis(16);
            let mut last_tick = Instant::now();
            loop {
                if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                    if let CrosstermEvent::Key(key) = event::read().unwrap() {
                        input_tx.send(InputEvent::Key(key)).unwrap();
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    input_tx.send(InputEvent::Tick).unwrap();
                    last_tick = Instant::now();
                }
                match input_kill_rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    _ => {}
                }
            }
        })?;

    terminal.clear()?;

    let mut input_string = "".to_string();

    let matcher = SkimMatcherV2::default().ignore_case();
    let cmds = vec![("quit", InputCommand::Quit)];

    loop {
        let input_str = &input_string;
        let mut matched: Option<(usize, i64)> = None;
        for (i, cmd) in cmds.iter().enumerate() {
            if let &Some(score) = &matcher.fuzzy_match(cmd.0, input_str) {
                if score > 0 && (matched.is_none() || score > matched.unwrap().1) {
                    matched = Some((i, score));
                }
            }
        }
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(f.size());
            use tui::widgets::Paragraph;
            f.render_widget(
                Paragraph::new(
                    [
                        Text::styled("> ", Style::default().fg(Color::Cyan)),
                        if input_str.len() == 0 {
                            Text::styled("Type something...", Style::default().fg(Color::DarkGray))
                        } else {
                            Text::raw(input_str)
                        },
                        if let Some((matched_ix, _)) = matched {
                            Text::styled(
                                " (".to_string() + cmds[matched_ix].0 + ")",
                                Style::default().fg(Color::LightBlue),
                            )
                        } else {
                            Text::raw("")
                        },
                    ]
                    .iter(),
                ),
                chunks[1],
            );
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);
            let mut list_state = tui::widgets::ListState::default();
            list_state.select(Some(1));
            f.render_stateful_widget(
                List::new(["Item 1", "Item 2", "Item 3"].iter().map(|i| Text::raw(*i)))
                    .block(
                        Block::default()
                            .title("List")
                            .title_style(Style::default().fg(Color::Cyan))
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().modifier(Modifier::ITALIC))
                    .highlight_symbol(">>"),
                chunks[0],
                &mut list_state,
            );
            f.render_widget(
                Tabs::default()
                    .block(
                        Block::default()
                            .title("Tabs")
                            .title_style(Style::default().fg(Color::Cyan))
                            .borders(Borders::ALL),
                    )
                    .titles(&["Tab1", "Tab2", "Tab3", "Tab4"])
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .select(2)
                    .divider(DOT),
                chunks[1],
            );
        })?;
        let mut cmd = None;
        match input_rx.recv()? {
            InputEvent::Key(event) => match event.code {
                KeyCode::Esc => {
                    cmd = Some(InputCommand::Quit);
                }
                KeyCode::Enter => match matched {
                    Some((matched_ix, _)) => {
                        cmd = Some(cmds[matched_ix].1);
                    }
                    _ => {}
                },
                KeyCode::Backspace => {
                    input_string.pop();
                }
                KeyCode::Char(c) => {
                    input_string.push(c);
                }
                _ => {}
            },
            InputEvent::Tick => {} // rerender
        }
        if let Some(cmd) = cmd {
            match cmd {
                InputCommand::Quit => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
            }
        }
    }

    input_kill_tx.send(())?;
    input_thread.join().expect("Input thread panicked");
    drop(music_thread); // NOTE: waiting for cpal crate push before handling more gracefully

    Ok(())
}
