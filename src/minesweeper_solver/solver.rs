use crate::field_generator::{MineSweeperCell, MineSweeperField};
use super::SolverSolution;
use super::MineSweeperCellState;
use super::box_logic::Box;
use super::permutation_checker::search_for_islands;
use super::MineSweeperSolver;
use colored::Colorize;
use core::panic;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, thread, vec};

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
        for (x, y) in self.field.sorted_fields() {
            match self.state[x][y] {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.board[x][y] {
                    MineSweeperCell::Empty => print!("  "),
                    MineSweeperCell::Mine => print!("{} ", "X".red()),
                    MineSweeperCell::Number(_n) => print!("{} ", self.field.board[x][y].get_colored()),
                },
            }

            if x == self.field.width - 1 {
                println!();
            }
        }
    }

    pub fn do_solving_step(&mut self) -> Option<()>{
        //match self.do_basic_neighbour_check(){
        //    Some(_) => {
        //        println!("Revealed or Flagged Fields based on basic count logic.");
        //        return Some(());
        //    },
        //    None => {}
        //}

        //match self.apply_basic_box_logic() {
        //    Some(_) => {
        //        println!("Revealed or Flagged Fields based on box logic.");
        //        return Some(());
        //    },
        //    None => {}
        //}

        //match self.apply_extended_box_logic() {
        //    Some(_) => {
        //        println!("Revealed or Flagged Fields based on extended box logic.");
        //        return Some(());
        //    },
        //    None => {}
        //}

        match self.apply_permutation_checks() {
            Some(_) => {
                println!("Revealed or Flagged Fields based on tested permutations.");
                return Some(());
            },
            None => {}
        }
        None
    }

    pub fn flag_all_hidden_cells(&mut self) {
        for (x, y) in self.field.sorted_fields() {
            if self.state[x][y] == MineSweeperCellState::Hidden {
                self.flag_cell(x, y);
            }
        }
    }

    #[track_caller]
    pub fn reveal_field(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed {
            return;
        }

        self.state[x][y] = MineSweeperCellState::Revealed;
        self.hidden_count -= 1;

        match self.field.board[x][y] {
            MineSweeperCell::Mine => {
                panic!("Game Over! The Solver hit a mine at ({}, {})", x, y);
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

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed || self.state[x][y] == MineSweeperCellState::Flagged {
            println!("Cell ({}, {}) is already revealed or flagged.", x, y);
            return;
        }

        self.state[x][y] = MineSweeperCellState::Flagged;
        self.flag_count += 1;
        self.hidden_count -= 1;
        self.remaining_mines -= 1;
    }

    #[track_caller]
    pub fn reveal_surrounding_cells(&mut self, x: usize, y: usize) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                self.reveal_field(new_x, new_y);
            }
        }
    }

    pub fn flag_surrounding_cells(&mut self, x: usize, y: usize) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                self.flag_cell(new_x, new_y);
            }
        }
    }

    pub fn has_unrevealed_neighbours(&self, x: usize, y: usize) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                return true;
            }
        }

        false
    }

    pub fn has_revealed_neighbours(&self, x: usize, y: usize) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Revealed {
                return true;
            }
        }

        false
    }

    pub fn get_surrounding_flag_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Flagged {
                count += 1;
            }
        }

        count
    }

    pub fn get_surrounding_unrevealed_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                count += 1;
            }
        }

        count
    }

    pub fn get_surrounding_unrevealed(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub fn get_reduced_count(&self, x: usize, y: usize) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = self.field.board[x][y].get_number();

        if flag_count > number {
            panic!("Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}", x, y, flag_count, number);
        }

        number - flag_count
    }

    pub fn has_informations(&self, x: usize, y: usize) -> bool {
        self.state[x][y] == MineSweeperCellState::Revealed
        && matches!(self.field.board[x][y], MineSweeperCell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }

    pub fn do_basic_neighbour_check(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                let needed_mines = self.get_reduced_count(x, y);
                if needed_mines == self.get_surrounding_unrevealed_count(x, y) {
                    self.flag_surrounding_cells(x, y);
                    did_something = true;
                }
                if needed_mines == 0 {
                    self.reveal_surrounding_cells(x, y);
                    did_something = true;
                }
            }
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }
}

pub fn start(field: MineSweeperField) {
    let mut game = MineSweeperSolver::new(field);

    println!("\nSolving the field...");
    println!("Width: {}, Height: {}, Mines: {}", game.field.width.to_string().green(), game.field.height.to_string().green(), game.field.mines.to_string().red());

    let handle = thread::Builder::new()
    .stack_size(32 * 1024 * 1024) // 32 MB
    .spawn(|| {
        let mut step_count: u64 = 0;
        game.reveal_field(game.field.start_field.0, game.field.start_field.1);

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
    let mut nothing_count = 0;

    loop {
        (*step_count) += 1;
        println!("Solving Step: {}", step_count.to_string().green());
        game.print();
        if game.hidden_count == 0 {
            println!("All cells revealed. Game solved!");
            game.print();
            return SolverSolution::FoundSolution;
        }

        if (game.flag_count + game.hidden_count) == game.field.mines {
            println!("All non mine cells revealed and all mines flagged. Game solved!");
            game.flag_all_hidden_cells();
            game.print();
            return SolverSolution::FoundSolution;
        }

        match game.do_solving_step() {
            Some(_) => {
                nothing_count = 0;
            },
            None => {
                nothing_count += 1;
                if nothing_count >= 3 {
                    println!("Nothing found in 3 steps. Stopping solver.");
                    game.print();
                    println!("Hidden Count: {}", game.hidden_count.to_string().red());
                    println!("Island Count: {}", search_for_islands(&game).len());
                    //println!("islands: {:?}", search_for_islands(&game));
                    return SolverSolution::NoSolution;
                }
            }
        }
    }
}

