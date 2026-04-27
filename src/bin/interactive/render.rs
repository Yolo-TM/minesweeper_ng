use super::state::{App, AppScreen, CreateState, Overlay, PlayState, PlayerCellState};
use minesweeper_ng_gen::{Cell, MineSweeperField};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}

fn field_rect(field_w: u32, field_h: u32, area: Rect) -> Rect {
    let w = (field_w * 2 + 2) as u16;
    let h = (field_h + 2) as u16;
    centered_rect(w.min(area.width), h.min(area.height), area)
}

fn cell_color(num: u8) -> Color {
    match num {
        0 => Color::DarkGray,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Red,
        4 => Color::Magenta,
        5 => Color::Yellow,
        6 => Color::Cyan,
        7 => Color::DarkGray,
        8 => Color::White,
        _ => Color::White,
    }
}

fn cell_span(cell: &Cell) -> Span<'static> {
    match cell {
        Cell::Empty => Span::styled("  ", Style::default()),
        Cell::Mine => Span::styled("# ", Style::default().fg(Color::White).bold()),
        Cell::Number(n) => Span::styled(format!("{} ", n), Style::default().fg(cell_color(*n))),
    }
}

pub fn render(frame: &mut Frame, app: &App) {
    match &app.screen {
        AppScreen::Play(state) => render_play(frame, state),
        AppScreen::Create(state) => render_create(frame, state),
    }

    match &app.overlay {
        Overlay::None => {}
        Overlay::TextInput(input, _) => input.render(frame),
        Overlay::Confirm(popup, _) => popup.render(frame),
    }
}

pub fn render_play(frame: &mut Frame, state: &PlayState) {
    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(frame.area());

    let elapsed = state.elapsed();
    let secs = elapsed.as_secs();
    let time_str = format!("{}:{:02}", secs / 60, secs % 60);

    let mut status_spans = vec![
        Span::raw(format!(
            "Mines: {} | Flags: {} | Time: {} ",
            state.field.get_mines(),
            state.flags_placed,
            time_str,
        )),
        Span::styled(
            "| SPACE: reveal/chord | F: flag | R: reveal all | Q: quit",
            Style::default().fg(Color::DarkGray),
        ),
    ];
    if let Some(ref msg) = state.message {
        let color = if state.won { Color::Green } else { Color::Red };
        status_spans.push(Span::styled(
            format!(" | {}", msg),
            Style::default().fg(color).bold(),
        ));
    }
    frame.render_widget(Paragraph::new(Line::from(status_spans)), chunks[0]);

    let field_area = chunks[1];
    let border_cells = 2u32;
    let visible_w =
        ((field_area.width as u32).saturating_sub(border_cells) / 2).min(state.field.get_width());
    let visible_h =
        ((field_area.height as u32).saturating_sub(border_cells)).min(state.field.get_height());

    let area = field_rect(visible_w, visible_h, field_area);
    let scroll_indicator =
        if visible_w < state.field.get_width() || visible_h < state.field.get_height() {
            format!(" [{},{}]", state.scroll_x + 1, state.scroll_y + 1)
        } else {
            String::new()
        };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Minesweeper{}", scroll_indicator));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let start = state.field.get_start_cell();
    let mut lines: Vec<Line> = Vec::with_capacity(visible_h as usize);

    for vy in 0..visible_h {
        let y = state.scroll_y + vy;
        let mut spans: Vec<Span> = Vec::with_capacity(visible_w as usize);
        for vx in 0..visible_w {
            let x = state.scroll_x + vx;
            let is_cursor = x == state.cursor_x && y == state.cursor_y;
            let is_start =
                (x, y) == start && state.state[x as usize][y as usize] == PlayerCellState::Hidden;

            let (text, mut style) = match state.state[x as usize][y as usize] {
                PlayerCellState::Hidden => ("? ".to_string(), Style::default()),
                PlayerCellState::Flagged => {
                    ("F ".to_string(), Style::default().fg(Color::Red).bold())
                }
                PlayerCellState::Revealed => match state.field.get_cell(x, y) {
                    Cell::Empty => ("  ".to_string(), Style::default()),
                    Cell::Mine => ("# ".to_string(), Style::default().fg(Color::White).bold()),
                    Cell::Number(n) => (format!("{} ", n), Style::default().fg(cell_color(*n))),
                },
            };

            if is_cursor {
                style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
            } else if is_start {
                style = style.bg(Color::Indexed(22));
            }
            spans.push(Span::styled(text, style));
        }
        lines.push(Line::from(spans));
    }
    frame.render_widget(Paragraph::new(lines), inner);
}

pub fn render_create(frame: &mut Frame, state: &CreateState) {
    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(frame.area());

    let total = state.field.get_width() * state.field.get_height();
    let mines = state.field.get_mines();
    let density = if total > 0 {
        (mines as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    let start_info = if state.start_cell_set {
        let (sx, sy) = state.field.get_start_cell();
        format!("({}, {})", sx, sy)
    } else {
        "Not set".into()
    };

    let mut status_spans = vec![
        Span::raw(format!(
            "{}x{} | Mines: {} ({:.1}%) | Start: {} ",
            state.field.get_width(),
            state.field.get_height(),
            mines,
            density,
            start_info,
        )),
        Span::styled(
            "| SPACE: mine | S: start | D: dimensions | Q: save/play/quit",
            Style::default().fg(Color::DarkGray),
        ),
    ];
    if let Some(ref err) = state.error_message {
        status_spans.push(Span::styled(
            format!(" | {}", err),
            Style::default().fg(Color::Red).bold(),
        ));
    }
    frame.render_widget(Paragraph::new(Line::from(status_spans)), chunks[0]);

    let field_area = chunks[1];
    let border_cells = 2u32;
    let visible_w =
        ((field_area.width as u32).saturating_sub(border_cells) / 2).min(state.field.get_width());
    let visible_h =
        ((field_area.height as u32).saturating_sub(border_cells)).min(state.field.get_height());

    let area = field_rect(visible_w, visible_h, field_area);
    let scroll_indicator =
        if visible_w < state.field.get_width() || visible_h < state.field.get_height() {
            format!(" [{},{}]", state.scroll_x + 1, state.scroll_y + 1)
        } else {
            String::new()
        };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Field Creator{}", scroll_indicator));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::with_capacity(visible_h as usize);
    for vy in 0..visible_h {
        let y = state.scroll_y + vy;
        let mut spans: Vec<Span> = Vec::with_capacity(visible_w as usize);
        for vx in 0..visible_w {
            let x = state.scroll_x + vx;
            let is_cursor = x == state.cursor_x && y == state.cursor_y;
            let is_start = state.start_cell_set && {
                let (sx, sy) = state.field.get_start_cell();
                x == sx && y == sy
            };

            let cell = state.field.get_cell(x, y);
            let span = cell_span(cell);
            let mut style = span.style;
            if is_cursor {
                style = style.bg(Color::DarkGray).add_modifier(Modifier::BOLD);
            } else if is_start {
                style = style.bg(Color::Indexed(22));
            }
            spans.push(Span::styled(span.content.to_string(), style));
        }
        lines.push(Line::from(spans));
    }
    frame.render_widget(Paragraph::new(lines), inner);
}
