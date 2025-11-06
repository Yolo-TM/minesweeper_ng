use core::panic;

use crate::{MineSweeperField, Cell};
use super::{CellState, SolvingStrategy};
use crate::normal_field::{SortedCells, SurroundingCells};

pub struct Solver {
    verbosity: u8,
    state: Vec<Vec<CellState>>,
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),

    // TODO: Later add tracking of how hard the step was / which logic was used
    solving_steps: Vec<(Vec<(u32, u32)>, Vec<(u32, u32)>)>,
}

impl Solver {
    pub fn new(field: &impl MineSweeperField, verbosity: u8) -> Self {
        let state = (0..field.get_width()).map(|x| {
            (0..field.get_height())
                .map(|y| CellState::Hidden(field.get_cell(x, y)))
                .collect()
            }).collect();

        Solver {
            verbosity,
            state,
            width: field.get_width(),
            height: field.get_height(),
            mines: field.get_mines(),
            start_cell: field.get_start_cell(),
            solving_steps: Vec::new(),
        }
    }

    pub fn is_solved(&self) -> bool {
        let mut flag_counter = 0;

        for (x, y) in self.sorted_fields() {
            match self.get_state(x, y) {
                CellState::Revealed(_) => continue,
                CellState::Hidden(_) => continue,
                CellState::Flagged(_) => flag_counter += 1,
            }
        }

        if flag_counter != self.mines {
            return false;
        }

        true
    }

    pub fn solve(&mut self) {
        if self.verbosity > 8 {
            println!("Starting solving process...");
            println!("Field dimensions: {}x{}, Mines: {}", self.width, self.height, self.mines);
            println!("Start cell: {:?}", self.start_cell);
            println!("Opening start cell...");
        }

        self.open_start_cell();

        if self.verbosity > 9 {
            self.print_field_state();
        }

        let mut step_count = 0;
        while step_count == self.solving_steps.len() {
            step_count += 1;
            self.do_solving_step();

            if self.verbosity > 9 {
                self.print_field_state();
            }
        }

        if self.verbosity > 8 {
            if self.is_solved() {
                println!("Field solved in {} steps!", step_count);
            } else {
                println!("Could not solve the field after {} steps.", step_count);
            }
        }
    }

    fn do_solving_step(&mut self) {
        let mut revealed_cells: Vec<(u32, u32)> = Vec::new();
        let mut flagged_cells: Vec<(u32, u32)> = Vec::new();

        for strategy in SolvingStrategy::iter() {
            let (rev, flag) = strategy.execute(self);

            if !rev.is_empty() || !flag.is_empty() {
                if self.verbosity > 6 {
                    println!("Strategy {:?} made progress: Revealed {}, Flagged {}", strategy, rev.len(), flag.len());
                }

                revealed_cells.extend(rev);
                flagged_cells.extend(flag);

                // Only apply one strategy per step
                break;
            }
        }

        if revealed_cells.is_empty() && flagged_cells.is_empty() {
            if self.verbosity > 7 {
                println!("No progress made in this step.");
            }
        } else {
            for (x, y) in &revealed_cells {
                self.reveal_cell(*x, *y);
            }

            for (x, y) in &flagged_cells {
                self.flag_cell(*x, *y);
            }

            self.solving_steps.push((revealed_cells, flagged_cells));
        }
    }

    fn open_start_cell(&mut self) {
        if self.get_state(self.start_cell.0, self.start_cell.1).get_cell() != &Cell::Empty {
            panic!("The Start Cell is directly bordering a Mine! The Solver expects the start cell to be empty.");
        }

        self.reveal_cell(self.start_cell.0, self.start_cell.1);
    }

    fn flag_cell(&mut self, x: u32, y: u32) {
        // Don't bother checking if it's actually a mine here, if its no mine, were definitely hitting one in the next step
        let cell = self.get_state(x, y).get_cell();
        self.state[x as usize][y as usize] = CellState::Flagged(cell.clone());
    }

    fn reveal_cell(&mut self, x: u32, y: u32) {
        let cell = self.get_state(x, y).get_cell().clone();

        match cell {
            Cell::Mine => {
                if self.verbosity > 2 {
                    println!("Stepped on a mine at ({}, {})! Solver failed.", x, y);
                    self.print_field_state();
                }
                panic!("Solver hit a mine!");
            }
            Cell::Number(n) => {
                self.state[x as usize][y as usize] = CellState::Revealed(cell);

                if self.get_surrounding_flag_count(x, y) == n {
                    self.reveal_surrounding_cells(x, y);
                }
            }
            Cell::Empty => {
                self.state[x as usize][y as usize] = CellState::Revealed(cell);

                self.reveal_surrounding_cells(x, y);
            }
        }
    }

    pub fn get_state(&self, x: u32, y: u32) -> &CellState {
        &self.state[x as usize][y as usize]
    }

    fn reveal_surrounding_cells(&mut self, x: u32, y: u32) {
        for (sx, sy) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(sx, sy) {
                self.reveal_cell(sx, sy);
            }
        }
    }

    fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        let mut flag_count = 0;

        for (sx, sy) in self.surrounding_fields(x, y, None) {
            if let CellState::Flagged(_) = self.get_state(sx, sy) {
                flag_count += 1;
            }
        }

        flag_count
    }

    pub fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                return true;
            }
        }

        false
    }

    fn has_revealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Revealed(_) = self.get_state(new_x, new_y) {
                return true;
            }
        }

        false
    }

    fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                count += 1;
            }
        }

        count
    }

    pub fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = match self.get_state(x, y).get_cell() {
            Cell::Number(n) => n,
            _ => panic!("get_reduced_count called on non-number cell at ({}, {})", x, y),
        };

        if flag_count > *number {
            panic!(
                "Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}",
                x, y, flag_count, number
            );
        }

        number - flag_count
    }

    fn print_field_state(&self) {
        print!("╔═");
        for _ in 0..self.width {
            print!("══");
        }
        println!("╗");

        print!("║");
        for (x, y) in self.sorted_fields() {
            print!(" {}", self.get_state(x, y).get_colored());

            if x == self.width - 1 {
                print!(" ║");
                println!();

                if y != self.height - 1 {
                    print!("║");
                }
            }
        }

        print!("╚═");
        for _ in 0..self.width {
            print!("══");
        }
        println!("╝");
    }

    pub fn sorted_fields(&self) -> SortedCells {
        SortedCells {
            width: self.width,
            height: self.height,
            current_x: 0,
            current_y: 0,
        }
    }

    fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingCells {
        let range = range.unwrap_or(1);
        SurroundingCells { x, y, width: self.width, height: self.height, range, dx: -(range as i8), dy: -(range as i8) }
    }
}