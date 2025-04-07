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
}

impl MinesweeperGame {
    pub fn reveal_field(&mut self, x: usize, y: usize) {
        if self.game_over {
            return;
        }

        if self.state[y][x] == MineSweeperCellState::Revealed {
            return;
        }

        self.state[y][x] = MineSweeperCellState::Revealed;

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
        } else {
            self.state[y][x] = MineSweeperCellState::Flagged;
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
                        MineSweeperCell::Mine => print!("X "),
                        MineSweeperCell::Number(n) => print!("{} ", self.field.get_colored_number(&n)),
                    },
                }
            }
            println!();
        }
    }
}
