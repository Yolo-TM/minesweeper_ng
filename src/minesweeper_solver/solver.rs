use std::collections::HashMap;

use crate::field_generator::{MineSweeperCell, MineSweeperField};
use super::{SolverSolution, MineSweeperCellState, MineSweeperSolver};
use colored::Colorize;

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    fn new(field: M) -> Self {
        let state = vec![vec![MineSweeperCellState::Hidden; field.get_height() as usize]; field.get_width() as usize];

        MineSweeperSolver {
            state,
            flag_count: 0,
            hidden_count: (field.get_width() * field.get_height()),
            remaining_mines: field.get_mines(),
            field,
        }
    }

    fn print(&self) {
        for (x, y) in self.field.sorted_fields() {
            match self.get_state(x, y) {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.get_cell(x as u32, y as u32) {
                    MineSweeperCell::Empty => print!("  "),
                    MineSweeperCell::Mine => print!("{} ", "X".red()),
                    MineSweeperCell::Number(_n) => print!("{} ", self.field.get_cell(x as u32, y as u32).get_colored()),
                },
            }

            if x == self.field.get_width() - 1 {
                println!();
            }
        }
    }

    pub fn get_state(&self, x: u32, y: u32) -> MineSweeperCellState {
        self.state[x as usize][y as usize].clone()
    }

    fn set_state(&mut self, x: u32, y: u32, state: MineSweeperCellState) {
        self.state[x as usize][y as usize] = state;
    }

    fn do_solving_step(&mut self) -> Option<u8>{
        match self.do_basic_neighbour_check(){
            Some(_) => {
                return Some(1);
            },
            None => {}
        }

        match self.apply_basic_box_logic() {
            Some(_) => {
                return Some(2);
            },
            None => {}
        }

        match self.apply_extended_box_logic() {
            Some(_) => {
                return Some(3);
            },
            None => {}
        }

        match self.apply_permutation_checks() {
            Some(_) => {
                return Some(4);
            },
            None => {}
        }
        None
    }

    fn flag_all_hidden_cells(&mut self) {
        for (x, y) in self.field.sorted_fields() {
            if self.get_state(x, y) == MineSweeperCellState::Hidden {
                self.flag_cell(x, y);
            }
        }
    }

    #[track_caller]
    pub fn reveal_field(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) == MineSweeperCellState::Revealed {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Revealed);
        self.hidden_count -= 1;

        match self.field.get_cell(x as u32, y as u32) {
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

    pub fn flag_cell(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) == MineSweeperCellState::Revealed || self.get_state(x, y) == MineSweeperCellState::Flagged {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Flagged);
        self.flag_count += 1;
        self.hidden_count -= 1;
        self.remaining_mines -= 1;
    }

    #[track_caller]
    pub fn reveal_surrounding_cells(&mut self, x: u32, y: u32) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                self.reveal_field(new_x, new_y);
            }
        }
    }

    pub fn flag_surrounding_cells(&mut self, x: u32, y: u32) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                self.flag_cell(new_x, new_y);
            }
        }
    }

    pub fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                return true;
            }
        }

        false
    }

    pub fn has_revealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Revealed {
                return true;
            }
        }

        false
    }

    pub fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Flagged {
                count += 1;
            }
        }

        count
    }

    pub fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                count += 1;
            }
        }

        count
    }

    pub fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = self.field.get_cell(x as u32, y as u32).get_number();

        if flag_count > number {
            panic!("Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}", x, y, flag_count, number);
        }

        number - flag_count
    }

    pub fn has_informations(&self, x: u32, y: u32) -> bool {
        self.get_state(x, y) == MineSweeperCellState::Revealed
        && matches!(self.field.get_cell(x, y), MineSweeperCell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }

    fn do_basic_neighbour_check(&mut self) -> Option<()> {
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

pub fn start<M: MineSweeperField>(field: M, enable_output: bool) -> SolverSolution {
    let mut game = MineSweeperSolver::new(field);

    game.reveal_field(game.field.get_start_field().0, game.field.get_start_field().1);

    return continue_solving(game, enable_output);
}

pub fn continue_solving<M: MineSweeperField>(mut game: MineSweeperSolver<M>, enable_output: bool) -> SolverSolution {
    let mut logic_levels_used: HashMap<u8, u32> = HashMap::new();
    let mut step_count = 0;

    loop {
        step_count += 1;

        if enable_output {
            println!("{}: {}", "Solver Step".bold(), step_count);
            game.print();
        }

        if game.hidden_count == 0 || (game.flag_count + game.hidden_count) == game.field.get_mines() {
            game.flag_all_hidden_cells();
            return SolverSolution::FoundSolution(step_count, logic_levels_used);
        }

        match game.do_solving_step() {
            Some(logic_level) => {
                *logic_levels_used.entry(logic_level).or_insert(0) += 1;
                continue;
            },
            None => {
                return SolverSolution::NoSolution(step_count, game.remaining_mines, game.hidden_count, game.state.clone());
            }
        }
    }
}