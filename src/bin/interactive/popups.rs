use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}

pub struct TextInput {
    pub title: String,
    pub prompt: String,
    pub value: String,
}

impl TextInput {
    pub fn new(title: &str, prompt: &str) -> Self {
        Self {
            title: title.into(),
            prompt: prompt.into(),
            value: String::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = centered_rect(40, 5, frame.area());
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let text = Line::from(vec![
            Span::raw(&self.prompt),
            Span::styled(
                format!("{}_", self.value),
                Style::default().fg(Color::White).bold(),
            ),
        ]);
        frame.render_widget(Paragraph::new(text), inner);
    }

    /// Returns Some(value) on Enter, None otherwise.
    pub fn handle_key(&mut self, code: KeyCode) -> Option<String> {
        match code {
            KeyCode::Enter => Some(self.value.clone()),
            KeyCode::Backspace => {
                self.value.pop();
                None
            }
            KeyCode::Char(c) => {
                self.value.push(c);
                None
            }
            _ => None,
        }
    }
}

pub struct ConfirmPopup {
    pub title: String,
    pub message: String,
    pub options: Vec<String>,
    pub selected: usize,
}

impl ConfirmPopup {
    pub fn new(title: &str, message: &str, options: Vec<&str>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            options: options.into_iter().map(String::from).collect(),
            selected: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = centered_rect(50, 7, frame.area());
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(inner);

        let msg = Paragraph::new(self.message.as_str()).wrap(Wrap { trim: true });
        frame.render_widget(msg, chunks[0]);

        let option_spans: Vec<Span> = self
            .options
            .iter()
            .enumerate()
            .flat_map(|(i, opt)| {
                let style = if i == self.selected {
                    Style::default().fg(Color::Black).bg(Color::White).bold()
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                vec![Span::styled(format!(" {} ", opt), style), Span::raw("  ")]
            })
            .collect();
        frame.render_widget(Paragraph::new(Line::from(option_spans)), chunks[1]);
    }

    /// Returns Some(selected_index) on Enter, None otherwise.
    pub fn handle_key(&mut self, code: KeyCode) -> Option<usize> {
        match code {
            KeyCode::Left | KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                None
            }
            KeyCode::Right | KeyCode::Down | KeyCode::Tab => {
                if self.selected + 1 < self.options.len() {
                    self.selected += 1;
                }
                None
            }
            KeyCode::Enter => Some(self.selected),
            _ => None,
        }
    }
}
