mod solving_api;
mod solving_helpers;
mod solving_utils;
mod cell_state;
mod strategy;

use strategy::SolvingStrategy;
use cell_state::CellState;
use super::MineSweeperField;

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

// Solves the given MineSweeperField and prints the steps taken to reach the solution.
pub fn solve_field(field: &impl MineSweeperField) {
    let mut solver = create_solver(field, 10);
    solver.solve();
}

// Checks if the given MineSweeperField is solvable. (No Output to stdout)
pub fn is_solvable(field: &impl MineSweeperField) -> bool {
    let mut solver = create_solver(field, 0);
    solver.solve();
    solver.is_solved()
}

pub fn create_solver(field: &impl MineSweeperField, verbosity: u8) -> Solver {
    Solver::new(field, verbosity)
}
