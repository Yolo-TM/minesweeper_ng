use super::{CellState, Solver, SolvingStrategy};
use crate::{Cell, MineSweeperField};
use core::panic;

impl Solver {
    pub fn new(field: &impl MineSweeperField, verbosity: u8) -> Self {
        let state = (0..field.get_width())
            .map(|x| {
                (0..field.get_height())
                    .map(|y| CellState::Hidden(field.get_cell(x, y)))
                    .collect()
            })
            .collect();

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
        let mut unrevealed = 0;

        for (x, y) in self.sorted_fields() {
            match self.get_state(x, y) {
                CellState::Hidden(_) | CellState::Flagged(_) => unrevealed += 1,
                CellState::Revealed(_) => continue,
            }
        }

        if unrevealed != self.mines {
            return false;
        }

        true
    }

    pub fn solve(&mut self) {
        self.println("Starting solving process...", 8);
        self.println(
            &format!(
                "Field dimensions: {}x{}, Mines: {}",
                self.width, self.height, self.mines
            ),
            8,
        );
        self.println(&format!("Start cell: {:?}", self.start_cell), 8);
        self.println("Opening start cell...", 8);

        self.open_start_cell();

        self.print_field(9);

        let mut step_count = 0;
        while step_count == self.solving_steps.len() {
            step_count += 1;
            self.do_solving_step();

            self.print_field(9);
        }

        if self.is_solved() {
            self.println(&format!("Field solved in {} steps!", step_count), 8);
        } else {
            self.println(
                &format!(
                    "Solver could not solve the field after {} steps.",
                    step_count
                ),
                8,
            );
        }
    }

    fn do_solving_step(&mut self) {
        let mut revealed_cells: Vec<(u32, u32)> = Vec::new();
        let mut flagged_cells: Vec<(u32, u32)> = Vec::new();

        for strategy in SolvingStrategy::iter() {
            let (rev, flag) = strategy.execute(self);

            if !rev.is_empty() || !flag.is_empty() {
                self.println(
                    &format!(
                        "Strategy {:?} made progress: Revealed {}, Flagged {}",
                        strategy,
                        rev.len(),
                        flag.len()
                    ),
                    7,
                );

                revealed_cells.extend(rev);
                flagged_cells.extend(flag);
                // Only apply one strategy per step
                break;
            }
        }

        if revealed_cells.is_empty() && flagged_cells.is_empty() {
            self.println("No progress made in this step.", 8);
        } else {
            for (x, y) in &revealed_cells {
                // Since Reveal Cell is recursive, not all cells will be in the solving_steps list
                self.reveal_cell(*x, *y);
            }
            for (x, y) in &flagged_cells {
                self.flag_cell(*x, *y);
            }

            self.solving_steps.push((revealed_cells, flagged_cells));
        }
    }

    fn open_start_cell(&mut self) {
        if self
            .get_state(self.start_cell.0, self.start_cell.1)
            .get_cell()
            != &Cell::Empty
        {
            panic!(
                "The Start Cell is directly bordering a Mine! The Solver expects the start cell to be empty."
            );
        }

        self.reveal_cell(self.start_cell.0, self.start_cell.1);
    }
}
