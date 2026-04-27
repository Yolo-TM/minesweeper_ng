use super::params::{FIELDS, Field, Params, get_field_str_mut, is_toggle};
use crossterm::event::KeyCode;

/// Returns Some(true) = start generation, Some(false) = quit, None = handled
pub fn handle_form_key(params: &mut Params, focused: &mut Field, code: KeyCode) -> Option<bool> {
    let idx = FIELDS.iter().position(|&f| f == *focused).unwrap_or(0);

    match code {
        KeyCode::Esc => return Some(false),
        KeyCode::Enter => return Some(true),
        KeyCode::Tab | KeyCode::Down => {
            *focused = FIELDS[(idx + 1) % FIELDS.len()];
        }
        KeyCode::BackTab | KeyCode::Up => {
            *focused = FIELDS[(idx + FIELDS.len() - 1) % FIELDS.len()];
        }
        KeyCode::Char(' ') if is_toggle(*focused) => match focused {
            Field::UseDensity => params.use_density = !params.use_density,
            Field::NoGuess => params.no_guess = !params.no_guess,
            _ => {}
        },
        KeyCode::Backspace if !is_toggle(*focused) => {
            get_field_str_mut(params, *focused).pop();
        }
        KeyCode::Char(c) if !is_toggle(*focused) => {
            get_field_str_mut(params, *focused).push(c);
        }
        _ => {}
    }
    None
}
