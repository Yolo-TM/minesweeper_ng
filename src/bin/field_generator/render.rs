use super::params::{FIELDS, Field, Params, field_label, field_value, is_toggle};
use super::screen::Screen;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Wrap},
};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use std::time::{Duration, Instant};

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let v = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(v[0])[0]
}

pub fn render(frame: &mut Frame, screen: &Screen) {
    match screen {
        Screen::Form {
            params,
            focused,
            error,
        } => render_form(frame, params, *focused, error),
        Screen::Generating {
            params,
            total,
            done,
            errors,
            start,
            finished,
            elapsed,
            ..
        } => render_generating(
            frame, params, *total, done, errors, *start, *finished, *elapsed,
        ),
    }
}

pub fn render_form(frame: &mut Frame, params: &Params, focused: Field, error: &Option<String>) {
    let area = frame.area();

    let title_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new("Minesweeper Field Generator — TAB/↑↓: navigate | SPACE: toggle | ENTER: generate | ESC: quit")
            .style(Style::default().fg(Color::DarkGray)),
        title_area,
    );

    let form_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height - 1,
    };
    let rows: Vec<Constraint> = FIELDS.iter().map(|_| Constraint::Length(1)).collect();
    let cells = Layout::vertical(rows).split(form_area);

    for (i, &f) in FIELDS.iter().enumerate() {
        let is_focused = f == focused;
        let label = field_label(f);
        let value = field_value(f, params);

        let label_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let value_style = if is_focused {
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        let cursor = if is_focused && !is_toggle(f) { "_" } else { "" };

        let line = Line::from(vec![
            Span::styled(format!("  {:>20}:  ", label), label_style),
            Span::styled(format!("{}{}", value, cursor), value_style),
        ]);
        frame.render_widget(Paragraph::new(line), cells[i]);
    }

    let preview_y = area.y + 1 + FIELDS.len() as u16 + 1;
    if preview_y < area.height {
        let preview_area = Rect {
            x: area.x,
            y: preview_y,
            width: area.width,
            height: 1,
        };
        let (text, style) = match params.validate() {
            Some(out) => {
                let w = params.width_val().unwrap_or(0);
                let h = params.height_val().unwrap_or(0);
                let cnt = params.count_val().unwrap_or(1);
                let ng = if params.no_guess { " no-guess" } else { "" };
                (
                    format!("  → {} × {}{} field(s), output: {}", cnt, w * h, ng, out),
                    Style::default().fg(Color::Green),
                )
            }
            None => (
                "  → Invalid parameters".into(),
                Style::default().fg(Color::Red),
            ),
        };
        frame.render_widget(Paragraph::new(Span::styled(text, style)), preview_area);
    }

    let cmd_y = preview_y + 1;
    if cmd_y < area.height {
        let cmd_area = Rect {
            x: area.x,
            y: cmd_y,
            width: area.width,
            height: 1,
        };
        let (cmd_text, cmd_style) = match params.cli_command() {
            Some(cmd) => (format!("  $ {}", cmd), Style::default().fg(Color::Cyan)),
            None => ("  $ —".into(), Style::default().fg(Color::DarkGray)),
        };
        frame.render_widget(Paragraph::new(Span::styled(cmd_text, cmd_style)), cmd_area);
    }

    if let Some(err) = error {
        let popup = centered_rect(60, 5, area);
        frame.render_widget(Clear, popup);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Error")
            .border_style(Style::default().fg(Color::Red));
        let inner = block.inner(popup);
        frame.render_widget(block, popup);
        frame.render_widget(
            Paragraph::new(err.as_str())
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::Red)),
            inner,
        );
    }
}

pub fn render_generating(
    frame: &mut Frame,
    params: &Params,
    total: u32,
    done: &Arc<AtomicU32>,
    errors: &Arc<AtomicU32>,
    start: Instant,
    finished: bool,
    elapsed: Duration,
) {
    let area = frame.area();
    let done_n = done.load(Ordering::Relaxed);
    let errors_n = errors.load(Ordering::Relaxed);
    let ratio = if total > 0 {
        done_n as f64 / total as f64
    } else {
        1.0
    };

    let elapsed_d = if finished { elapsed } else { start.elapsed() };
    let secs = elapsed_d.as_secs();
    let eta = if done_n > 0 && !finished {
        let rate = done_n as f64 / elapsed_d.as_secs_f64();
        let remaining = (total - done_n) as f64 / rate;
        format!(" ETA: {:.0}s", remaining)
    } else {
        String::new()
    };

    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    let ng = if params.no_guess { " no-guess" } else { "" };
    let status = if finished {
        format!(
            "Done! {}/{} fields{}  ({} errors)  in {}:{:02}",
            done_n,
            total,
            ng,
            errors_n,
            secs / 60,
            secs % 60
        )
    } else {
        format!(
            "Generating {}/{} fields{}  ({} errors)  {}:{:02}{}",
            done_n,
            total,
            ng,
            errors_n,
            secs / 60,
            secs % 60,
            eta
        )
    };
    frame.render_widget(
        Paragraph::new(status).style(Style::default().fg(Color::White)),
        chunks[0],
    );

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .ratio(ratio.min(1.0))
        .label(format!("{}/{}", done_n, total));
    frame.render_widget(gauge, chunks[1]);

    let footer = if finished {
        "Press any key to exit"
    } else {
        "ESC to cancel (already-generated files will remain)"
    };
    frame.render_widget(
        Paragraph::new(footer).style(Style::default().fg(Color::DarkGray)),
        chunks[2],
    );
}
