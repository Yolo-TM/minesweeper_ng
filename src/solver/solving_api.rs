use super::cell_state::CellState;
use super::findings::Finding;
use super::strategy::SolvingStrategy;
use crate::{Cell, MineSweeperField};
use log::{debug, trace};

pub struct Solver {
    pub(super) state: Vec<Vec<CellState>>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) mines: u32,
    pub(super) start_cell: (u32, u32),
    pub(super) solving_steps: Vec<Finding>,
}

pub fn is_solvable(field: &impl MineSweeperField) -> bool {
    let mut solver = create_solver(field);
    solver.solve();
    solver.is_solved()
}

pub fn create_solver(field: &impl MineSweeperField) -> Solver {
    Solver::new(field)
}

impl Solver {
    pub fn new(field: &impl MineSweeperField) -> Self {
        let state = (0..field.get_width())
            .map(|x| {
                (0..field.get_height())
                    .map(|y| CellState::Hidden(field.get_cell(x, y).clone()))
                    .collect()
            })
            .collect();

        Solver {
            state,
            width: field.get_width(),
            height: field.get_height(),
            mines: field.get_mines(),
            start_cell: field.get_start_cell(),
            solving_steps: Vec::new(),
        }
    }

    pub(crate) fn revealed_count(&self) -> u32 {
        self.sorted_fields()
            .filter(|&(x, y)| matches!(self.get_state(x, y), CellState::Revealed(_)))
            .count() as u32
    }

    pub(crate) fn get_state_grid(&self) -> &Vec<Vec<CellState>> {
        &self.state
    }

    pub fn is_solved(&self) -> bool {
        self.sorted_fields()
            .filter(|&(x, y)| !matches!(self.get_state(x, y), CellState::Revealed(_)))
            .count() as u32
            == self.mines
    }

    pub fn get_solving_steps(&self) -> Vec<Finding> {
        self.solving_steps.clone()
    }

    pub fn solve(&mut self) {
        debug!("Starting solving process...");
        debug!(
            "Field dimensions: {}x{}, Mines: {}",
            self.width, self.height, self.mines
        );
        debug!("Start cell: {:?}", self.start_cell);
        debug!("Opening start cell...");

        self.open_start_cell();

        trace!("{}", self.format_field_state());

        let mut step_count = 0;
        while let Some(finding) = self.do_solving_step() {
            self.solving_steps.push(finding);
            step_count += 1;

            trace!("{}", self.format_field_state());
        }

        if self.is_solved() {
            debug!("Field solved in {} steps!", step_count);
        } else {
            debug!(
                "Solver could not solve the field after {} steps.",
                step_count
            );
        }
    }

    fn do_solving_step(&mut self) -> Option<Finding> {
        let mut step_solution: Option<Finding> = None;

        for strategy in SolvingStrategy::iter() {
            let finding: Finding = strategy.execute(&*self);

            if finding.success() {
                debug!(
                    "Strategy {:?} made progress: Revealed {:?}, Flagged {:?}",
                    strategy,
                    finding.get_safe_fields(),
                    finding.get_mine_fields()
                );

                step_solution = Some(finding);
                break;
            }
        }

        if step_solution.is_none() {
            debug!("No progress made in this step.");
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
