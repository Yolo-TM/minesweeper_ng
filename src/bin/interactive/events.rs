use super::popups::{ConfirmPopup, TextInput};
use super::state::{
    App, AppScreen, ConfirmPurpose, CreateState, InputPurpose, Overlay, PlayState, PlayerCellState,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use minesweeper_ng_gen::{MineSweeperField, MineSweeperFieldFileIO};
use ratatui::layout::Rect;
use std::{io, time::Duration};

pub enum AppAction {
    Nothing,
    Quit,
    ShowOverlay(Overlay),
}

pub fn handle_event(app: &mut App, terminal_size: Rect) -> io::Result<()> {
    let has_active_timer = matches!(&app.screen, AppScreen::Play(s) if s.timer_start.is_some());
    let timeout = if has_active_timer {
        Duration::from_millis(250)
    } else {
        Duration::from_secs(60)
    };
    if !event::poll(timeout)? {
        return Ok(());
    }

    if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
        if kind != KeyEventKind::Press {
            return Ok(());
        }

        if !matches!(app.overlay, Overlay::None) {
            handle_overlay_key(app, code);
            return Ok(());
        }

        let action = match &mut app.screen {
            AppScreen::Play(state) => handle_play_key(state, code),
            AppScreen::Create(state) => handle_create_key(state, code),
        };

        match action {
            AppAction::Nothing => {}
            AppAction::Quit => app.quit = true,
            AppAction::ShowOverlay(overlay) => app.overlay = overlay,
        }
    }

    let status_h = 2u32;
    let border_h = 2u32;
    let border_w = 2u32;
    let avail_h = (terminal_size.height as u32).saturating_sub(status_h + border_h);
    let avail_w = (terminal_size.width as u32).saturating_sub(border_w) / 2;
    match &mut app.screen {
        AppScreen::Play(state) => {
            let vw = avail_w.min(state.field.get_width());
            let vh = avail_h.min(state.field.get_height());
            state.update_scroll(vw, vh);
        }
        AppScreen::Create(state) => {
            let vw = avail_w.min(state.field.get_width());
            let vh = avail_h.min(state.field.get_height());
            state.update_scroll(vw, vh);
        }
    }

    Ok(())
}

fn handle_overlay_key(app: &mut App, code: KeyCode) {
    let overlay = std::mem::replace(&mut app.overlay, Overlay::None);

    match overlay {
        Overlay::TextInput(mut input, purpose) => {
            if code == KeyCode::Esc {
                return;
            }
            if let Some(value) = input.handle_key(code) {
                handle_input_submit(app, purpose, value);
            } else {
                app.overlay = Overlay::TextInput(input, purpose);
            }
        }
        Overlay::Confirm(mut popup, purpose) => {
            if code == KeyCode::Esc {
                return;
            }
            if let Some(idx) = popup.handle_key(code) {
                handle_confirm_submit(app, purpose, idx);
            } else {
                app.overlay = Overlay::Confirm(popup, purpose);
            }
        }
        Overlay::None => unreachable!(),
    }
}

fn handle_input_submit(app: &mut App, purpose: InputPurpose, value: String) {
    match purpose {
        InputPurpose::SetWidth => {
            if let Ok(w) = value.parse::<u32>() {
                if w >= 3 {
                    if let AppScreen::Create(state) = &mut app.screen {
                        let old_h = state.field.get_height();
                        state.resize(w, old_h);
                    }
                    app.overlay = Overlay::TextInput(
                        TextInput::new("Resize", "New height: "),
                        InputPurpose::SetHeight,
                    );
                }
            }
        }
        InputPurpose::SetHeight => {
            if let Ok(h) = value.parse::<u32>() {
                if h >= 3 {
                    if let AppScreen::Create(state) = &mut app.screen {
                        let cur_w = state.field.get_width();
                        state.resize(cur_w, h);
                    }
                }
            }
        }
        InputPurpose::SaveFilename => {
            if !value.is_empty() {
                if let AppScreen::Create(state) = &app.screen {
                    let filepath = format!("{}.minesweeper", value);
                    let _ = state.field.to_file(&filepath);
                }
            }
            app.quit = true;
        }
    }
}

fn handle_confirm_submit(app: &mut App, purpose: ConfirmPurpose, idx: usize) {
    match purpose {
        ConfirmPurpose::QuitCreate => match idx {
            0 => {
                app.overlay = Overlay::TextInput(
                    TextInput::new("Save", "Filename (no ext): "),
                    InputPurpose::SaveFilename,
                );
            }
            1 => {
                if let AppScreen::Create(state) = &app.screen {
                    let field = state.field.clone();
                    app.screen = AppScreen::Play(super::state::PlayState::new(field));
                }
            }
            _ => {}
        },
    }
}

pub fn handle_play_key(state: &mut PlayState, code: KeyCode) -> AppAction {
    match code {
        KeyCode::Up => state.move_cursor(0, -1),
        KeyCode::Down => state.move_cursor(0, 1),
        KeyCode::Left => state.move_cursor(-1, 0),
        KeyCode::Right => state.move_cursor(1, 0),
        KeyCode::Char(' ') if !state.game_over => {
            let cs = state.state[state.cursor_x as usize][state.cursor_y as usize];
            if cs == PlayerCellState::Revealed {
                state.try_chord(state.cursor_x, state.cursor_y);
            } else {
                state.reveal_cell(state.cursor_x, state.cursor_y);
            }
        }
        KeyCode::Char('f' | 'F') => state.toggle_flag(state.cursor_x, state.cursor_y),
        KeyCode::Char('r' | 'R') => state.reveal_all(),
        KeyCode::Char('q' | 'Q') | KeyCode::Esc => return AppAction::Quit,
        _ => {}
    }
    AppAction::Nothing
}

pub fn handle_create_key(state: &mut CreateState, code: KeyCode) -> AppAction {
    match code {
        KeyCode::Up => state.move_cursor(0, -1),
        KeyCode::Down => state.move_cursor(0, 1),
        KeyCode::Left => state.move_cursor(-1, 0),
        KeyCode::Right => state.move_cursor(1, 0),
        KeyCode::Char(' ') => {
            state.toggle_mine();
            state.error_message = None;
        }
        KeyCode::Char('s' | 'S') => {
            state.set_start_cell();
            state.error_message = None;
        }
        KeyCode::Char('d' | 'D') => {
            return AppAction::ShowOverlay(Overlay::TextInput(
                TextInput::new("Resize", "New width: "),
                InputPurpose::SetWidth,
            ));
        }
        KeyCode::Char('q' | 'Q') | KeyCode::Esc => {
            if let Some(err) = state.validate_for_quit() {
                state.error_message = Some(err);
                return AppAction::Nothing;
            }
            return AppAction::ShowOverlay(Overlay::Confirm(
                ConfirmPopup::new(
                    "Done",
                    "What would you like to do?",
                    vec!["Save", "Play", "Cancel"],
                ),
                ConfirmPurpose::QuitCreate,
            ));
        }
        _ => {}
    }
    AppAction::Nothing
}
