use super::*;

use console::Console;

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AppCommand {
    SetMode(AppMode),
    PushInputChar(char),
    PopInputChar,
    QuitApp,
}

pub struct App {
    app_mode: AppMode,
    console: Console,
}

impl App {
    pub fn new() -> App {
        App {
            app_mode: AppMode::Normal,
            console: Console::new(),
        }
    }

    fn set_mode(&mut self, mode: AppMode) {
        self.app_mode = mode;
    }
}

impl Widget for App {
    fn render(&self, f: &mut CFrame, rect: Rect) -> Vec<Rect> {
        let chunk = match self.app_mode {
            AppMode::Normal => rect,
            AppMode::Input => self.console.render(f, rect)[0],
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunk);
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
        vec![]
    }

    fn handle_input(&self, evt: InputEvent) -> Option<AppCommand> {
        let mut cmd = None;

        match evt {
            InputEvent::Key(event) => match self.app_mode {
                AppMode::Normal => match event.code {
                    KeyCode::Char('/') => {
                        cmd = Some(AppCommand::SetMode(AppMode::Input));
                    }
                    KeyCode::Esc => {
                        cmd = Some(AppCommand::QuitApp);
                    }
                    _ => {}
                },
                AppMode::Input => {
                    cmd = self.console.handle_input(evt);
                }
            },
            InputEvent::Tick => {} // rerender
        }

        cmd
    }

    fn handle_command(&mut self, cmd: AppCommand) {
        match cmd {
            AppCommand::SetMode(mode) => self.set_mode(mode),
            _ => {
                self.console.handle_command(cmd);
            }
        }
    }
}
