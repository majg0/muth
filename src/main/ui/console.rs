use super::*;

pub struct Console {
    input_string: String,
    matcher: SkimMatcherV2,
    matched: Option<(usize, i64)>,
    cmds: Vec<(&'static str, AppCommand)>,
}

impl Console {
    pub fn new() -> Console {
        Console {
            input_string: "".to_string(),
            matcher: SkimMatcherV2::default().ignore_case(),
            matched: None,
            cmds: vec![("quit", AppCommand::QuitApp)],
        }
    }

    fn on_string_changed(&mut self) {
        let input_str = &self.input_string;
        let mut matched: Option<(usize, i64)> = None;
        for (i, cmd) in self.cmds.iter().enumerate() {
            if let Some(score) = self.matcher.fuzzy_match(cmd.0, input_str) {
                if score > 0 && (matched.is_none() || score > matched.unwrap().1) {
                    matched = Some((i, score));
                }
            }
        }
        self.matched = matched;
    }
}

impl Widget for Console {
    fn render(&self, f: &mut CFrame, rect: Rect) -> Vec<Rect> {
        let input_str = &self.input_string;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(rect);

        use tui::widgets::Paragraph;
        f.render_widget(
            Paragraph::new(
                [
                    Text::styled("> ", Style::default().fg(Color::Cyan)),
                    if input_str.is_empty() {
                        Text::styled("Type something...", Style::default().fg(Color::DarkGray))
                    } else {
                        Text::raw(input_str)
                    },
                    if let Some((matched_ix, _)) = self.matched {
                        Text::styled(
                            " (".to_string() + self.cmds[matched_ix].0 + ")",
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

        vec![chunks[0]]
    }
    fn handle_input(&self, evt: InputEvent) -> Option<AppCommand> {
        let mut cmd = None;

        match evt {
            InputEvent::Key(event) => match event.code {
                KeyCode::Esc => {
                    cmd = Some(AppCommand::SetMode(AppMode::Normal));
                }
                KeyCode::Enter => {
                    if let Some((matched_ix, _)) = self.matched {
                        cmd = Some(self.cmds[matched_ix].1);
                    }
                }
                KeyCode::Backspace => {
                    cmd = Some(AppCommand::PopInputChar);
                }
                KeyCode::Char(c) => {
                    cmd = Some(AppCommand::PushInputChar(c));
                }
                _ => {}
            },
            InputEvent::Tick => {} // rerender
        }

        cmd
    }
    fn handle_command(&mut self, cmd: AppCommand) {
        match cmd {
            AppCommand::PopInputChar => {
                self.input_string.pop();
                self.on_string_changed();
            }
            AppCommand::PushInputChar(c) => {
                self.input_string.push(c);
                self.on_string_changed();
            }
            _ => {}
        }
    }
}
