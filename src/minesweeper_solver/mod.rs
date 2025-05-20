use crate::field_generator::*;
use colored::Colorize;

mod box_logic;
mod permutation_checker;
pub mod solver;

pub enum SolverSolution {
    NoSolution(u64),
    FoundSolution(u64),
}

#[derive(Clone, PartialEq)]
enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
struct MineSweeperSolver<M: MineSweeperField> {
    field:  M,
    state: Vec<Vec<MineSweeperCellState>>,
    flag_count: u32,
    hidden_count: u32,
    remaining_mines: u32
}

pub fn solve(field: impl MineSweeperField) {
    match solver::start(field) {
        SolverSolution::NoSolution(step_count) => {
            println!("No solution found. Stopped after {} steps.", step_count.to_string().red());
        }
        SolverSolution::FoundSolution(step_count) => {
            println!("Found a solution after {} steps.", step_count.to_string().green());
        }
    }
}