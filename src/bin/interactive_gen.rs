use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use minesweeper_ng_gen::*;
use std::io::{self, Write};

struct InteractiveField {
    field: DefinedField,
    cursor_x: u32,
    cursor_y: u32,
    start_cell_set: bool,
    error_message: Option<String>,
}

impl InteractiveField {
    fn new(width: u32, height: u32) -> Self {
        // Small Hack to create an empty field since creating an field with Mines::Count(0) is forbidden
        let mut field = DefinedField::new(width, height, Mines::Count(1));
        field.initialize(vec![(0, 0)]);
        field.remove_mine(0, 0);
        Self {
            field,
            cursor_x: 0,
            cursor_y: 0,
            start_cell_set: false,
            error_message: None,
        }
    }

    fn toggle_mine(&mut self) {
        let cell = self.field.get_cell(self.cursor_x, self.cursor_y);
        match cell {
            Cell::Mine => {
                self.field.remove_mine(self.cursor_x, self.cursor_y);
            }
            _ => {
                self.field.place_mine(self.cursor_x, self.cursor_y);
            }
        }
    }

    fn set_start_cell(&mut self) {
        self.field.set_start_cell(self.cursor_x, self.cursor_y);
        self.start_cell_set = true;
    }

    fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide
        )?;

        let total_cells = self.field.get_width() * self.field.get_height();
        let mine_count = self.field.get_mines();
        let density = if total_cells > 0 {
            (mine_count as f64 / total_cells as f64) * 100.0
        } else {
            0.0
        };

        writeln!(
            stdout,
            "Arrow keys to move, SPACE to place/remove mine, S to set start cell, Q to quit and save"
        )?;
        writeln!(
            stdout,
            "Mines placed: {} (Density: {:.2}%)",
            mine_count, density
        )?;
        writeln!(
            stdout,
            "Cursor: ({}, {}) | Start cell: {:?}",
            self.cursor_x,
            self.cursor_y,
            if self.start_cell_set {
                format!(
                    "({}, {})",
                    self.field.get_start_cell().0,
                    self.field.get_start_cell().1
                )
            } else {
                "Not set".to_string()
            }
        )?;

        if let Some(ref error) = self.error_message {
            use crossterm::style::{Color, ResetColor, SetForegroundColor};
            execute!(stdout, SetForegroundColor(Color::Red))?;
            writeln!(stdout, "ERROR: {}", error)?;
            execute!(stdout, ResetColor)?;
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
                let is_start_cell = self.start_cell_set && {
                    let (start_x, start_y) = self.field.get_start_cell();
                    x == start_x && y == start_y
                };
                let cell = self.field.get_cell(x, y);

                if is_cursor {
                    execute!(stdout, SetBackgroundColor(Color::DarkYellow))?;
                } else if is_start_cell {
                    execute!(stdout, SetBackgroundColor(Color::DarkGreen))?;
                }

                let colored_cell = cell.get_colored();
                write!(stdout, "{} ", colored_cell)?;

                if is_cursor || is_start_cell {
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
}

fn main() -> io::Result<()> {
    println!("Interactive Minesweeper Field Generator");
    println!("========================================\n");

    print!("Enter field width: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let width: u32 = input.trim().parse().expect("Invalid width");

    print!("Enter field height: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let height: u32 = input.trim().parse().expect("Invalid height");

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut interactive_field = InteractiveField::new(width, height);
    interactive_field.draw()?;

    // Main loop
    'main: loop {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            // ignore release and repeat
            if kind != KeyEventKind::Press {
                continue;
            }

            match code {
                KeyCode::Up => {
                    interactive_field.move_cursor(0, -1);
                }
                KeyCode::Down => {
                    interactive_field.move_cursor(0, 1);
                }
                KeyCode::Left => {
                    interactive_field.move_cursor(-1, 0);
                }
                KeyCode::Right => {
                    interactive_field.move_cursor(1, 0);
                }
                KeyCode::Char(' ') => {
                    interactive_field.toggle_mine();
                    interactive_field.error_message = None; // Clear error on action
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    interactive_field.set_start_cell();
                    interactive_field.error_message = None; // Clear error on action
                }
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    // Validate before quitting
                    if !interactive_field.start_cell_set {
                        interactive_field.error_message =
                            Some("Start cell not set! Press S to set it.".to_string());
                        interactive_field.draw()?;
                        continue;
                    }

                    let (start_x, start_y) = interactive_field.field.get_start_cell();
                    let start_cell = interactive_field.field.get_cell(start_x, start_y);
                    if start_cell != Cell::Empty {
                        interactive_field.error_message = Some(format!(
                            "Start cell at ({}, {}) is not empty! Move it to an empty cell.",
                            start_x, start_y
                        ));
                        interactive_field.draw()?;
                        continue;
                    }

                    break 'main;
                }
                _ => {}
            }
            interactive_field.draw()?;
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    print!("Enter filename to save (without extension): ");
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    let filename = input.trim();

    if !filename.is_empty() {
        let filepath = format!("{}.minesweeper", filename);
        interactive_field.field.to_file(&filepath)?;
        println!("Field saved to: {}", filepath);
    } else {
        println!("No filename provided, field not saved.");
    }

    Ok(())
}
