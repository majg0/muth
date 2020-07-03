use std::{
    error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
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

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

mod app;
mod console;

use app::*;

pub type CFrame<'a> = tui::terminal::Frame<'a, CrosstermBackend<std::io::Stdout>>;

pub trait Widget {
    fn render(&self, f: &mut CFrame, rect: Rect) -> Vec<Rect>;
    fn handle_input(&self, evt: InputEvent) -> Option<AppCommand>;
    fn handle_command(&mut self, cmd: AppCommand);
}

#[derive(Clone, Copy)]
pub enum InputEvent {
    Key(KeyEvent),
    Tick,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let (input_tx, input_rx) = mpsc::channel();

    // NOTE: separate thread in order to not block on input
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
                    if input_tx.send(InputEvent::Tick).is_err() {
                        break; // NOTE: exiting when disconnected by rx
                    }
                    last_tick = Instant::now();
                }
            }
        })?;

    run_tui(input_rx).unwrap(); // NOTE: moving rx into here

    input_thread.join().expect("Input thread panicked");

    Ok(())
}

fn run_tui(input_rx: mpsc::Receiver<InputEvent>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?; // NOTE: works natively but not in vscode terminal
    terminal.clear()?;

    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let rect = f.size();
            app.render(&mut f, rect);
        })?;

        let cmd = app.handle_input(input_rx.recv()?);
        if let Some(cmd) = cmd {
            app.handle_command(cmd);
            if cmd == AppCommand::QuitApp {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
