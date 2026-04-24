use super::{CellState, Finding, Solver, SolvingStrategy};
use crate::{Cell, MineSweeperField};
use core::panic;

impl Solver {
    pub fn new(field: &impl MineSweeperField, verbosity: u8) -> Self {
        let state = (0..field.get_width())
            .map(|x| {
                (0..field.get_height())
                    .map(|y| CellState::Hidden(field.get_cell(x, y).clone()))
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

    pub fn get_solving_steps(&self) -> Vec<Finding> {
        return self.solving_steps.clone();
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
        while let Some(finding) = self.do_solving_step() {
            self.solving_steps.push(finding);
            step_count += 1;

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

    fn do_solving_step(&mut self) -> Option<Finding> {
        let mut step_solution: Option<Finding> = None;

        for strategy in SolvingStrategy::iter() {
            let finding: Finding = strategy.execute(self);

            if finding.success() {
                self.println(
                    &format!(
                        "Strategy {:?} made progress: Revealed {:?}, Flagged {:?}",
                        strategy,
                        finding.get_safe_fields(),
                        finding.get_mine_fields()
                    ),
                    7,
                );

                step_solution = Some(finding);
                break;
            }
        }

        if step_solution.is_none() {
            self.println("No progress made in this step.", 8);
            return step_solution;
        }
        let mut step_solution: Finding = step_solution.unwrap();

        let mut recursive_revealed_fields: Vec<Vec<(u32, u32)>> = Vec::new();
        for (x, y) in step_solution.get_safe_fields() {
            self.reveal_cell(*x, *y, &mut recursive_revealed_fields, 0);
        }
        for (x, y) in step_solution.get_mine_fields() {
            self.flag_cell(*x, *y);
        }

        step_solution.add_recursive_informations(recursive_revealed_fields);
        Some(step_solution)
    }

    pub fn open_start_cell(&mut self) {
        if self
            .get_state(self.start_cell.0, self.start_cell.1)
            .get_cell()
            != &Cell::Empty
        {
            panic!(
                "The Start Cell is directly bordering a Mine! The Solver expects the start cell to be empty."
            );
        }

        let mut finding = Finding::new();
        finding.add_safe_field((self.start_cell.0, self.start_cell.1));

        let mut recursive_revealed_fields: Vec<Vec<(u32, u32)>> = Vec::new();

        self.reveal_cell(
            self.start_cell.0,
            self.start_cell.1,
            &mut recursive_revealed_fields,
            0,
        );

        finding.add_recursive_informations(recursive_revealed_fields);
        self.solving_steps.push(finding);
    }
}
