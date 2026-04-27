use super::events::handle_form_key;
use super::generation::start_generation;
use super::params::{Field, Params};
use super::render::render;
use super::screen::{ProgressEvent, Screen};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{
    io,
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

pub fn run_tui() -> io::Result<()> {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        default_hook(info);
    }));

    let mut screen = Screen::Form {
        params: Params::default(),
        focused: Field::Width,
        error: None,
    };

    let mut terminal = ratatui::init();
    let result = tui_loop(&mut terminal, &mut screen);
    ratatui::restore();
    result
}

fn tui_loop(terminal: &mut ratatui::DefaultTerminal, screen: &mut Screen) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(f, screen))?;

        match screen {
            Screen::Form {
                params,
                focused,
                error,
            } => {
                if !event::poll(Duration::from_secs(60))? {
                    continue;
                }
                if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                    if kind != KeyEventKind::Press {
                        continue;
                    }
                    if error.is_some() {
                        *error = None;
                        continue;
                    }
                    match handle_form_key(params, focused, code) {
                        Some(false) => return Ok(()),
                        Some(true) => {
                            if params.validate().is_none() {
                                *error = Some(
                                    "Invalid parameters — check width, height, mines, count."
                                        .into(),
                                );
                                continue;
                            }
                            let total = params.count_val().unwrap();
                            let (done, errors, rx) = start_generation(params);
                            *screen = Screen::Generating {
                                params: params.clone(),
                                total,
                                done,
                                errors,
                                rx,
                                start: Instant::now(),
                                finished: false,
                                elapsed: Duration::ZERO,
                            };
                        }
                        None => {}
                    }
                }
            }
            Screen::Generating {
                done,
                total,
                rx,
                finished,
                elapsed,
                start,
                ..
            } => {
                loop {
                    match rx.try_recv() {
                        Ok(ProgressEvent::AllDone) => {
                            *elapsed = start.elapsed();
                            *finished = true;
                        }
                        Ok(ProgressEvent::Done) => {}
                        Err(_) => break,
                    }
                }

                let timeout = if *finished {
                    Duration::from_secs(60)
                } else {
                    Duration::from_millis(100)
                };
                if event::poll(timeout)? {
                    if let Event::Key(KeyEvent { kind, code, .. }) = event::read()? {
                        if kind == KeyEventKind::Press {
                            if *finished || code == KeyCode::Esc {
                                return Ok(());
                            }
                        }
                    }
                }

                if done.load(Ordering::Relaxed) >= *total {
                    *elapsed = start.elapsed();
                    *finished = true;
                }
            }
        }
    }
}
