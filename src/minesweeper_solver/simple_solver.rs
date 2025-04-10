use core::panic;
use std::thread;
use colored::Colorize;
use crate::minesweeper_field::{
    MineSweeperField,
    MineSweeperCell,
};
use crate::minesweeper_solver::{
    SolverSolution,
    MineSweeperCellState,
};

#[derive(Clone, PartialEq)]
pub struct MineSweeperSolver{
    pub field: MineSweeperField,
    pub state: Vec<Vec<MineSweeperCellState>>,
    pub flag_count: u64,
    pub hidden_count: u64,
    pub remaining_mines: u64
}

impl MineSweeperSolver {
    pub fn new(field : MineSweeperField) -> Self {
        let state = vec![vec![MineSweeperCellState::Hidden; field.height]; field.width];

        MineSweeperSolver {
            state,
            flag_count: 0,
            hidden_count: (field.width * field.height) as u64,
            remaining_mines: field.mines,
            field,
        }
    }

    pub fn print(&self) {
        for x in 0..self.field.width {
            for y in 0..self.field.height {
                match self.state[x][y] {
                    MineSweeperCellState::Hidden => print!("? "),
                    MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                    MineSweeperCellState::Revealed => match self.field.board[x][y] {
                        MineSweeperCell::Empty => print!("  "),
                        MineSweeperCell::Mine => print!("{} ", "X".red()),
                        MineSweeperCell::Number(n) => print!("{} ", self.field.get_colored_number(&n)),
                    },
                }
            }
            println!();
        }
    }

    pub fn get_empty_cell(&self) -> (usize, usize) {
        for x in 0..self.field.width {
            for y in 0..self.field.height {
                if self.field.board[x][y] == MineSweeperCell::Empty {
                    return (x, y);
                }
            }
        }

        panic!("No empty cell found on this game.");
    }

    pub fn do_solving_step(&mut self) {
        self.do_basic_neighbour_check();
    }

    pub fn flag_all_hidden_cells(&mut self) {
        for x in 0..self.field.width {
            for y in 0..self.field.height {
                if self.state[x][y] == MineSweeperCellState::Hidden {
                    self.flag_cell(x, y);
                }
            }
        }
    }

    pub fn reveal_field(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed {
            return;
        }

        self.state[x][y] = MineSweeperCellState::Revealed;
        self.hidden_count -= 1;

        match self.field.board[x][y] {
            MineSweeperCell::Mine => {
                panic!("Game Over! The Solver hit a mine!");
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

    fn flag_cell(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed {
            return;
        }

        if self.state[x][y] == MineSweeperCellState::Flagged {
            self.state[x][y] = MineSweeperCellState::Hidden;
            self.flag_count -= 1;
            self.hidden_count += 1;
            self.remaining_mines += 1;
        } else {
            self.state[x][y] = MineSweeperCellState::Flagged;
            self.flag_count += 1;
            self.hidden_count -= 1;
            self.remaining_mines -= 1;
        }
    }

    fn reveal_surrounding_cells(&mut self, x: usize, y: usize) {
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width && new_y < self.field.height {
                    if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                        self.reveal_field(new_x, new_y);
                    }
                }
            }
        }
    }

    fn flag_surrounding_cells(&mut self, x: usize, y: usize) {
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width && new_y < self.field.height {
                    if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                        self.flag_cell(new_x, new_y);
                    }
                }
            }
        }
    }

    fn has_unrevealed_neighbours(&self, x: usize, y: usize) -> bool {
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width && new_y < self.field.height {
                    if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn get_surrounding_flag_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width && new_y < self.field.height {
                    if self.state[new_x][new_y] == MineSweeperCellState::Flagged {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn get_surrounding_unrevealed_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let new_x = (x as isize + j) as usize;
                let new_y = (y as isize + i) as usize;
                if new_x < self.field.width as usize && new_y < self.field.height as usize {
                    if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn do_basic_neighbour_check(&mut self) {
        for x in 0..self.field.width {
            for y in 0..self.field.height {
                if self.state[x][y] == MineSweeperCellState::Revealed
                && matches!(self.field.board[x][y], MineSweeperCell::Number(_))
                && self.has_unrevealed_neighbours(x, y) {
                    let flag_count = self.get_surrounding_flag_count(x, y);
                    let number = self.field.board[x][y].get_number();
                    let unrevealed_count = self.get_surrounding_unrevealed_count(x, y);

                    let needed_mines = number - flag_count;
                    if needed_mines == unrevealed_count {
                        self.flag_surrounding_cells(x, y);
                    }
                    if needed_mines == 0 {
                        self.reveal_surrounding_cells(x, y);
                    }
                }
            }
        }
    }
}


pub fn simple_minesweeper_solver(field: MineSweeperField) {
    let mut game = MineSweeperSolver::new(field);

    println!("\nSolving the field...");
    println!("Width: {}, Height: {}, Mines: {}", game.field.width.to_string().green(), game.field.height.to_string().green(), game.field.mines.to_string().red());

    let handle = thread::Builder::new()
    .stack_size(32 * 1024 * 1024) // 32 MB
    .spawn(|| {
        let mut step_count: u64 = 0;
        let empty_cell = game.get_empty_cell();
        game.reveal_field(empty_cell.0, empty_cell.1);

        match solver(game, &mut step_count) {
            SolverSolution::NoSolution => {
                println!("No solution found. Stopped after {} steps.", step_count.to_string().red());
            }
            SolverSolution::FoundSolution => {
                println!("Found a solution after {} steps.", step_count.to_string().green());
            }
        }
    })
    .expect("Thread spawn failed");

    handle.join().unwrap();
}

fn solver(mut game: MineSweeperSolver, step_count: &mut u64) -> SolverSolution {

    while (*step_count) < 10 {
        (*step_count) += 1;
        println!("Solving Step: {}", step_count.to_string().green());
        game.print();

        if game.hidden_count == 0 {
            println!("All cells revealed. Game solved!");
            return SolverSolution::FoundSolution;
        }

        if (game.flag_count + game.hidden_count) == game.field.mines {
            println!("All non mine cells revealed and all mines flagged. Game solved!");
            game.flag_all_hidden_cells();
            game.print();
            return SolverSolution::FoundSolution;
        }

        game.do_solving_step();
    }

    return SolverSolution::NoSolution;
}