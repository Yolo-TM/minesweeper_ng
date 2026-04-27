use super::popups::{ConfirmPopup, TextInput};
use minesweeper_ng_gen::{Cell, DefinedField, MineSweeperField, Mines};
use std::time::{Duration, Instant};

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerCellState {
    Hidden,
    Revealed,
    Flagged,
}

pub enum AppScreen {
    Create(CreateState),
    Play(PlayState),
}

pub enum Overlay {
    None,
    TextInput(TextInput, InputPurpose),
    Confirm(ConfirmPopup, ConfirmPurpose),
}

pub enum InputPurpose {
    SetWidth,
    SetHeight,
    SaveFilename,
}

pub enum ConfirmPurpose {
    QuitCreate,
}

pub struct App {
    pub screen: AppScreen,
    pub overlay: Overlay,
    pub quit: bool,
}

impl App {
    pub fn new_create(width: u32, height: u32) -> Self {
        Self {
            screen: AppScreen::Create(CreateState::new(width, height)),
            overlay: Overlay::None,
            quit: false,
        }
    }

    pub fn new_play(field: DefinedField) -> Self {
        Self {
            screen: AppScreen::Play(PlayState::new(field)),
            overlay: Overlay::None,
            quit: false,
        }
    }
}

pub struct PlayState {
    pub field: DefinedField,
    pub state: Vec<Vec<PlayerCellState>>,
    pub cursor_x: u32,
    pub cursor_y: u32,
    pub game_over: bool,
    pub won: bool,
    pub message: Option<String>,
    pub flags_placed: u32,
    pub scroll_x: u32,
    pub scroll_y: u32,
    pub timer_start: Option<Instant>,
    pub timer_elapsed: Duration,
}

impl PlayState {
    pub fn new(field: DefinedField) -> Self {
        let state = vec![
            vec![PlayerCellState::Hidden; field.get_height() as usize];
            field.get_width() as usize
        ];
        let (start_x, start_y) = field.get_start_cell();
        Self {
            field,
            state,
            cursor_x: start_x,
            cursor_y: start_y,
            game_over: false,
            won: false,
            message: None,
            flags_placed: 0,
            scroll_x: 0,
            scroll_y: 0,
            timer_start: None,
            timer_elapsed: Duration::ZERO,
        }
    }

    pub fn start_timer(&mut self) {
        if self.timer_start.is_none() {
            self.timer_start = Some(Instant::now());
        }
    }

    pub fn stop_timer(&mut self) {
        if let Some(start) = self.timer_start.take() {
            self.timer_elapsed = start.elapsed();
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.timer_start {
            Some(start) => start.elapsed(),
            None => self.timer_elapsed,
        }
    }

    pub fn update_scroll(&mut self, visible_w: u32, visible_h: u32) {
        if visible_w > 0 && self.cursor_x < self.scroll_x {
            self.scroll_x = self.cursor_x;
        }
        if visible_w > 0 && self.cursor_x >= self.scroll_x + visible_w {
            self.scroll_x = self.cursor_x - visible_w + 1;
        }
        if visible_h > 0 && self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        }
        if visible_h > 0 && self.cursor_y >= self.scroll_y + visible_h {
            self.scroll_y = self.cursor_y - visible_h + 1;
        }
    }

    pub fn reveal_cell(&mut self, x: u32, y: u32) {
        if self.game_over || self.state[x as usize][y as usize] != PlayerCellState::Hidden {
            return;
        }

        self.start_timer();

        let mut stack = vec![(x, y)];
        while let Some((cx, cy)) = stack.pop() {
            if self.game_over || self.state[cx as usize][cy as usize] != PlayerCellState::Hidden {
                continue;
            }

            self.state[cx as usize][cy as usize] = PlayerCellState::Revealed;
            match self.field.get_cell(cx, cy) {
                Cell::Mine => {
                    self.game_over = true;
                    self.stop_timer();
                    self.message = Some("GAME OVER! You hit a mine!".into());
                    return;
                }
                Cell::Empty => {
                    for (nx, ny) in self.field.surrounding_fields(cx, cy, None) {
                        if self.state[nx as usize][ny as usize] == PlayerCellState::Hidden {
                            stack.push((nx, ny));
                        }
                    }
                }
                Cell::Number(_) => {}
            }
        }
        self.check_win();
    }

    pub fn try_chord(&mut self, x: u32, y: u32) {
        if self.game_over {
            return;
        }
        let required = self.field.get_cell(x, y).get_number();

        let mut flag_count: u8 = 0;
        let mut hidden_count: u8 = 0;
        for (nx, ny) in self.field.surrounding_fields(x, y, None) {
            match self.state[nx as usize][ny as usize] {
                PlayerCellState::Flagged => flag_count += 1,
                PlayerCellState::Hidden => hidden_count += 1,
                PlayerCellState::Revealed => {}
            }
        }

        if hidden_count + flag_count == required && hidden_count > 0 {
            for (nx, ny) in self.field.surrounding_fields(x, y, None) {
                if self.state[nx as usize][ny as usize] == PlayerCellState::Hidden {
                    self.state[nx as usize][ny as usize] = PlayerCellState::Flagged;
                    self.flags_placed += 1;
                }
            }
            return;
        }

        if flag_count != required {
            return;
        }

        let to_reveal: Vec<_> = self
            .field
            .surrounding_fields(x, y, None)
            .filter(|(nx, ny)| self.state[*nx as usize][*ny as usize] == PlayerCellState::Hidden)
            .collect();

        for (nx, ny) in to_reveal {
            self.reveal_cell(nx, ny);
            if self.game_over {
                break;
            }
        }
    }

    pub fn toggle_flag(&mut self, x: u32, y: u32) {
        if self.game_over {
            return;
        }
        match self.state[x as usize][y as usize] {
            PlayerCellState::Hidden => {
                self.state[x as usize][y as usize] = PlayerCellState::Flagged;
                self.flags_placed += 1;
            }
            PlayerCellState::Flagged => {
                self.state[x as usize][y as usize] = PlayerCellState::Hidden;
                self.flags_placed -= 1;
            }
            PlayerCellState::Revealed => {}
        }
    }

    pub fn check_win(&mut self) {
        let unrevealed: u32 = self
            .field
            .sorted_fields()
            .filter(|(x, y)| self.state[*x as usize][*y as usize] != PlayerCellState::Revealed)
            .count() as u32;

        if unrevealed == self.field.get_mines() {
            self.game_over = true;
            self.won = true;
            self.stop_timer();
            let secs = self.timer_elapsed.as_secs();
            self.message = Some(format!(
                "CONGRATULATIONS! You won in {}:{:02}!",
                secs / 60,
                secs % 60
            ));
        }
    }

    pub fn reveal_all(&mut self) {
        if !self.game_over {
            return;
        }
        for x in 0..self.field.get_width() {
            for y in 0..self.field.get_height() {
                self.state[x as usize][y as usize] = PlayerCellState::Revealed;
            }
        }
    }

    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.cursor_x = (self.cursor_x as i32 + dx)
            .max(0)
            .min(self.field.get_width() as i32 - 1) as u32;
        self.cursor_y = (self.cursor_y as i32 + dy)
            .max(0)
            .min(self.field.get_height() as i32 - 1) as u32;
    }
}

pub struct CreateState {
    pub field: DefinedField,
    pub cursor_x: u32,
    pub cursor_y: u32,
    pub start_cell_set: bool,
    pub error_message: Option<String>,
    pub scroll_x: u32,
    pub scroll_y: u32,
}

impl CreateState {
    pub fn new(width: u32, height: u32) -> Self {
        let mut field = DefinedField::new(width, height, Mines::Count(1)).unwrap();
        field.initialize(vec![(0, 0)]);
        field.remove_mine(0, 0);
        Self {
            field,
            cursor_x: 0,
            cursor_y: 0,
            start_cell_set: false,
            error_message: None,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn update_scroll(&mut self, visible_w: u32, visible_h: u32) {
        if visible_w > 0 && self.cursor_x < self.scroll_x {
            self.scroll_x = self.cursor_x;
        }
        if visible_w > 0 && self.cursor_x >= self.scroll_x + visible_w {
            self.scroll_x = self.cursor_x - visible_w + 1;
        }
        if visible_h > 0 && self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        }
        if visible_h > 0 && self.cursor_y >= self.scroll_y + visible_h {
            self.scroll_y = self.cursor_y - visible_h + 1;
        }
    }

    pub fn toggle_mine(&mut self) {
        match self.field.get_cell(self.cursor_x, self.cursor_y) {
            Cell::Mine => self.field.remove_mine(self.cursor_x, self.cursor_y),
            _ => self.field.place_mine(self.cursor_x, self.cursor_y),
        }
    }

    pub fn set_start_cell(&mut self) {
        self.field.set_start_cell(self.cursor_x, self.cursor_y);
        self.start_cell_set = true;
    }

    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.cursor_x = (self.cursor_x as i32 + dx)
            .max(0)
            .min(self.field.get_width() as i32 - 1) as u32;
        self.cursor_y = (self.cursor_y as i32 + dy)
            .max(0)
            .min(self.field.get_height() as i32 - 1) as u32;
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let old_width = self.field.get_width();
        let old_height = self.field.get_height();
        let old_field = self.field.clone();
        let old_start_set = self.start_cell_set;

        let mut field = DefinedField::new(new_width, new_height, Mines::Count(1)).unwrap();
        field.initialize(vec![(0, 0)]);
        field.remove_mine(0, 0);

        let copy_w = old_width.min(new_width);
        let copy_h = old_height.min(new_height);
        for x in 0..copy_w {
            for y in 0..copy_h {
                if matches!(old_field.get_cell(x, y), Cell::Mine) {
                    field.place_mine(x, y);
                }
            }
        }

        if old_start_set {
            let (sx, sy) = old_field.get_start_cell();
            if sx < new_width && sy < new_height {
                field.set_start_cell(sx, sy);
                self.start_cell_set = true;
            } else {
                self.start_cell_set = false;
            }
        }

        self.field = field;
        self.cursor_x = self.cursor_x.min(new_width - 1);
        self.cursor_y = self.cursor_y.min(new_height - 1);
    }

    pub fn validate_for_quit(&self) -> Option<String> {
        if !self.start_cell_set {
            return Some("Start cell not set! Press S to set it.".into());
        }
        let (sx, sy) = self.field.get_start_cell();
        if self.field.get_cell(sx, sy) != &Cell::Empty {
            return Some(format!(
                "Start cell ({}, {}) is not empty! Move it.",
                sx, sy
            ));
        }
        None
    }
}
