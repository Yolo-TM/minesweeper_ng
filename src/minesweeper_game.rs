use crate::minesweeper_field::{
    MineSweeperField,
    MineSweeperCell,
    MineSweeperCellState,
};
use colored::Colorize;

#[derive(Clone, PartialEq)]
pub struct MinesweeperGame {
    pub field: MineSweeperField,
    pub state: Vec<Vec<MineSweeperCellState>>,
    pub game_over: bool,
    pub time: u64,
    pub flag_count: u64,
    pub hidden_count: u64,
    pub remaining_mines: u64
}

impl MinesweeperGame {
    pub fn new(field : MineSweeperField) -> Self {
        let mut state = vec![vec![MineSweeperCellState::Hidden; field.width as usize]; field.height as usize];

        MinesweeperGame {
            state,
            game_over: false,
            time: 0,
            flag_count: 0,
            hidden_count: field.width * field.height,
            remaining_mines: field.mines,
            field,
        }
    }

    pub fn reveal_field(&mut self, x: usize, y: usize) {
        if self.game_over {
            return;
        }

        if self.state[y][x] == MineSweeperCellState::Revealed {
            return;
        }

        self.state[y][x] = MineSweeperCellState::Revealed;
        self.hidden_count -= 1;

        match self.field.board[y][x] {
            MineSweeperCell::Mine => {
                self.game_over = true;
            }
            MineSweeperCell::Empty => {
                self.reveal_surrounding_cells(x, y);
            }
            MineSweeperCell::Number(i) => {
                if i == self.get_surrounding_flag_count(x, y) {
                    self.reveal_surrounding_cells(x, y);
                }
            }
        }
    }

    fn reveal_surrounding_cells(&mut self, x: usize, y: usize) {
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width as usize && new_y < self.field.height as usize {
                    if self.state[new_y][new_x] == MineSweeperCellState::Hidden {
                        self.reveal_field(new_x, new_y);
                    }
                }
            }
        }
    }

    fn get_surrounding_flag_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width as usize && new_y < self.field.height as usize {
                    if self.state[new_y][new_x] == MineSweeperCellState::Flagged {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if self.game_over {
            return;
        }

        if self.state[y][x] == MineSweeperCellState::Revealed {
            return;
        }

        if self.state[y][x] == MineSweeperCellState::Flagged {
            self.state[y][x] = MineSweeperCellState::Hidden;
            self.flag_count -= 1;
            self.hidden_count += 1;
            self.remaining_mines += 1;
        } else {
            self.state[y][x] = MineSweeperCellState::Flagged;
            self.flag_count += 1;
            self.hidden_count -= 1;
            self.remaining_mines -= 1;
        }
    }

    pub fn print(&self) {
        for y in 0..self.field.height as usize {
            for x in 0..self.field.width as usize {
                match self.state[y][x] {
                    MineSweeperCellState::Hidden => print!("? "),
                    MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                    MineSweeperCellState::Revealed => match self.field.board[y][x] {
                        MineSweeperCell::Empty => print!("  "),
                        MineSweeperCell::Mine => print!("{} ", "X".red()),
                        MineSweeperCell::Number(n) => print!("{} ", self.field.get_colored_number(&n)),
                    },
                }
            }
            println!();
        }
    }
}
