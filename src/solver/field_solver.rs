use crate::MineSweeperField;
use super::CellState;
use crate::normal_field::SortedCells;

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
            match self.state[x as usize][y as usize] {
                CellState::Revealed(_) => continue,
                CellState::Flagged(_) => flag_counter += 1,
                CellState::Hidden(_) => return false,
            }
        }

        if flag_counter != self.mines {
            return false;
        }

        true
    }

    pub fn solve(&self) {
        if self.verbosity > 8 {
            println!("Starting solving process...");
            println!("Field dimensions: {}x{}, Mines: {}", self.width, self.height, self.mines);
            println!("Start cell: {:?}", self.start_cell);
        }

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

    fn do_solving_step(&self) {
        let revealed_cells: Vec<(u32, u32)> = Vec::new();
        let flagged_cells: Vec<(u32, u32)> = Vec::new();

        // TODO! Implement solving logic here

        if revealed_cells.is_empty() && flagged_cells.is_empty() {
            if self.verbosity > 7 {
                println!("No progress made in this step.");
            }
        } else {
            for (x, y) in &revealed_cells {
                self.reveal_cell(self, *x, *y);
            }

            for (x, y) in &flagged_cells {
                self.flag_cell(self, *x, *y);
            }

            self.solving_steps.push((revealed_cells, flagged_cells));
        }
    }

    fn flag_cell(&self, x: u32, y: u32) {
        // Don't bother checking if it's actually a mine here, if its no mine, were definitely hitting one in the next step
        let cell = self.state[x as usize][y as usize].get_cell();
        self.state[x as usize][y as usize] = CellState::Flagged(cell);
    }

    fn reveal_cell(&self, x: u32, y: u32) {
        let cell = self.state[x as usize][y as usize].get_cell();

        match cell {
            Cell::Mine => {
                if self.verbosity > 2 {
                    println!("Stepped on a mine at ({}, {})! Solver failed.", x, y);
                    self.print_field_state();
                }
                panic!("Solver hit a mine!");
            }
            _ => self.state[x as usize][y as usize] = CellState::Revealed(cell);
        }
    }

    fn print_field_state(&self) {
        print!("╔═");
        for _ in 0..self.width {
            print!("══");
        }
        println!("╗");

        print!("║");
        for (x, y) in self.sorted_fields() {
            print!(" {}", self.state[x as usize][y as usize].get_colored());

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

    fn sorted_fields(&self) -> SortedCells {
        SortedCells {
            width: self.width,
            height: self.height,
            current_x: 0,
            current_y: 0,
        }
    }
}