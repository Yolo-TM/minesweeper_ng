use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use minesweeper_ng_gen::*;
use std::io::{self, Write};

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

struct InteractivePlayer {
    field: DefinedField,
    state: Vec<Vec<CellState>>,
    cursor_x: u32,
    cursor_y: u32,
    game_over: bool,
    won: bool,
    message: Option<String>,
    flags_placed: u32,
}

impl InteractivePlayer {
    fn new(field: DefinedField) -> Self {
        let state =
            vec![vec![CellState::Hidden; field.get_height() as usize]; field.get_width() as usize];
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
        }
    }

    fn reveal_cell(&mut self, x: u32, y: u32) {
        if self.game_over {
            return;
        }

        let state = self.state[x as usize][y as usize];
        if state != CellState::Hidden {
            return;
        }

        self.state[x as usize][y as usize] = CellState::Revealed;
        let cell = self.field.get_cell(x, y);

        match cell {
            Cell::Mine => {
                self.game_over = true;
                self.message = Some("GAME OVER! You hit a mine!".to_string());
            }
            Cell::Empty => {
                for (nx, ny) in self.field.surrounding_fields(x, y, None) {
                    self.reveal_cell(nx, ny);
                }

                self.check_win();
            }
            Cell::Number(num) => {
                let flags: Vec<(u32, u32)> = self
                    .field
                    .surrounding_fields(x, y, None)
                    .filter(|(nx, ny)| self.state[*nx as usize][*ny as usize] == CellState::Flagged)
                    .collect();

                if flags.len() == num as usize {
                    for (nx, ny) in self.field.surrounding_fields(x, y, None) {
                        self.reveal_cell(nx, ny);
                    }
                }
                self.check_win();
            }
        }
    }

    fn try_chord(&mut self, x: u32, y: u32) {
        if self.game_over {
            return;
        }

        let cell = self.field.get_cell(x, y);
        let required_flags = cell.get_number();

        let mut flag_count = 0;
        let mut hidden_count = 0;
        for (nx, ny) in self.field.surrounding_fields(x, y, None) {
            match self.state[nx as usize][ny as usize] {
                CellState::Flagged => flag_count += 1,
                CellState::Hidden => hidden_count += 1,
                CellState::Revealed => {}
            }
        }

        // Case 1: If all remaining unflagged cells need to be mines, auto-flag them
        if hidden_count + flag_count == required_flags && hidden_count > 0 {
            for (nx, ny) in self.field.surrounding_fields(x, y, None) {
                self.flag_cell(nx, ny);
            }
            return;
        }

        // Case 2: If flag count matches the number, reveal all non-flagged neighbors
        if flag_count != required_flags {
            return;
        }

        let neighbors_to_reveal: Vec<(u32, u32)> = self
            .field
            .surrounding_fields(x, y, None)
            .filter(|(nx, ny)| self.state[*nx as usize][*ny as usize] == CellState::Hidden)
            .collect();

        for (nx, ny) in neighbors_to_reveal {
            self.reveal_cell(nx, ny);
            if self.game_over {
                break; // Stop if we hit a mine
            }
        }
    }

    fn toggle_flag(&mut self, x: u32, y: u32) {
        if self.game_over {
            return;
        }

        match self.state[x as usize][y as usize] {
            CellState::Hidden => {
                self.flag_cell(x, y);
            }
            CellState::Flagged => {
                self.unflag_cell(x, y);
            }
            CellState::Revealed => {}
        }
    }

    fn flag_cell(&mut self, x: u32, y: u32) {
        if self.state[x as usize][y as usize] == CellState::Hidden {
            self.state[x as usize][y as usize] = CellState::Flagged;
            self.flags_placed += 1;
        }
    }

    fn unflag_cell(&mut self, x: u32, y: u32) {
        if self.state[x as usize][y as usize] == CellState::Flagged {
            self.state[x as usize][y as usize] = CellState::Hidden;
            self.flags_placed -= 1;
        }
    }

    fn check_win(&mut self) {
        let mut flag_count = 0;
        let mut hidden_count = 0;
        for (nx, ny) in self.field.sorted_fields() {
            match self.state[nx as usize][ny as usize] {
                CellState::Flagged => flag_count += 1,
                CellState::Hidden => hidden_count += 1,
                CellState::Revealed => {}
            }
        }

        if flag_count + hidden_count != self.field.get_mines() {
            return;
        }

        self.game_over = true;
        self.won = true;
        self.message = Some("CONGRATULATIONS! You won!".to_string());
    }

    fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide
        )?;

        writeln!(
            stdout,
            "Arrow keys: move | SPACE: reveal/chord | F: flag/unflag | Q: quit"
        )?;
        writeln!(
            stdout,
            "Mines: {} | Flags placed: {}",
            self.field.get_mines(),
            self.flags_placed
        )?;

        if let Some(ref msg) = self.message {
            if self.game_over {
                if self.won {
                    execute!(stdout, SetForegroundColor(Color::Green))?;
                } else {
                    execute!(stdout, SetForegroundColor(Color::Red))?;
                }
                writeln!(stdout, "{}", msg)?;
                execute!(stdout, SetForegroundColor(Color::Reset))?;
            } else {
                writeln!(stdout, "{}", msg)?;
            }
        }

        writeln!(stdout)?;

        write!(stdout, "╔")?;
        for _ in 0..self.field.get_width() {
            write!(stdout, "══")?;
        }
        writeln!(stdout, "╗")?;

        for y in 0..self.field.get_height() {
            write!(stdout, "║")?;
            for x in 0..self.field.get_width() {
                let is_cursor = x == self.cursor_x && y == self.cursor_y;
                let is_start = (x, y) == self.field.get_start_cell();
                let state = &self.state[x as usize][y as usize];

                // Background color priority: cursor (yellow) > start cell (green)
                if is_cursor {
                    execute!(stdout, SetBackgroundColor(Color::DarkYellow))?;
                } else if is_start && *state == CellState::Hidden {
                    execute!(stdout, SetBackgroundColor(Color::DarkGreen))?;
                }

                match state {
                    CellState::Hidden => {
                        write!(stdout, "? ")?;
                    }
                    CellState::Flagged => {
                        execute!(stdout, SetForegroundColor(Color::Red))?;
                        write!(stdout, "F ")?;
                        execute!(stdout, SetForegroundColor(Color::Reset))?;
                    }
                    CellState::Revealed => {
                        let cell = self.field.get_cell(x, y);
                        let colored_cell = cell.get_colored();
                        write!(stdout, "{} ", colored_cell)?;
                    }
                }

                if is_cursor || (is_start && *state == CellState::Hidden) {
                    execute!(stdout, SetBackgroundColor(Color::Reset))?;
                }
            }
            writeln!(stdout, "║")?;
        }

        write!(stdout, "╚")?;
        for _ in 0..self.field.get_width() {
            write!(stdout, "══")?;
        }
        writeln!(stdout, "╝")?;

        stdout.flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        let new_x = (self.cursor_x as i32 + dx)
            .max(0)
            .min(self.field.get_width() as i32 - 1);
        let new_y = (self.cursor_y as i32 + dy)
            .max(0)
            .min(self.field.get_height() as i32 - 1);
        self.cursor_x = new_x as u32;
        self.cursor_y = new_y as u32;
    }

    fn reveal_all(&mut self) {
        if !self.game_over {
            return;
        }

        for x in 0..self.field.get_width() {
            for y in 0..self.field.get_height() {
                self.state[x as usize][y as usize] = CellState::Revealed;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filepath = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Interactive Minesweeper Player");
        println!("==============================\n");
        print!("Enter field file path: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    let field = DefinedField::from_file(&filepath)?;

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut player = InteractivePlayer::new(field);
    player.draw()?;

    'main: loop {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind != KeyEventKind::Press {
                continue;
            }

            match code {
                KeyCode::Up => {
                    player.move_cursor(0, -1);
                }
                KeyCode::Down => {
                    player.move_cursor(0, 1);
                }
                KeyCode::Left => {
                    player.move_cursor(-1, 0);
                }
                KeyCode::Right => {
                    player.move_cursor(1, 0);
                }
                KeyCode::Char(' ') => {
                    if !player.game_over {
                        let state =
                            player.state[player.cursor_x as usize][player.cursor_y as usize];

                        if state == CellState::Revealed {
                            player.try_chord(player.cursor_x, player.cursor_y);
                        } else {
                            player.reveal_cell(player.cursor_x, player.cursor_y);
                        }
                    }
                }
                KeyCode::Char('f') | KeyCode::Char('F') => {
                    player.toggle_flag(player.cursor_x, player.cursor_y);
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    player.reveal_all();
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    break 'main;
                }
                _ => {}
            }
            player.draw()?;
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    if player.won {
        println!("Congratulations! You successfully completed the field!");
    } else if player.game_over {
        println!("Game Over! Better luck next time!");
    } else {
        println!("Thanks for playing!");
    }

    Ok(())
}
