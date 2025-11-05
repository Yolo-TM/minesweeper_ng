mod box_logic;
//mod permutation_checker;
mod solver_framework;
mod islands;
mod solver_steps;

use crate::MineSweeperField;
use colored::Colorize;
pub use islands::{search_for_islands, merge_islands};
pub use solver_steps::{SolverStepCounter, SolverStep};

#[derive(Clone)]
pub struct MineSweeperSolver<M: MineSweeperField> {
    field:  M,
    state: Vec<Vec<MineSweeperCellState>>,
    flag_count: u32,
    hidden_count: u32,
    remaining_mines: u32,
    solution: SolverSolution,
    steps: SolverStepCounter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}


#[derive(Clone, PartialEq, Debug)]
pub enum SolverSolution {
    NeverStarted,
    NoSolution(u32, u32, u32, Vec<Vec<MineSweeperCellState>>),
    FoundSolution(SolverStepCounter),
}

pub fn solve(field: impl MineSweeperField, print_steps: bool) {
    let (width, height, mines) = field.get_dimensions();
    let mut solver = MineSweeperSolver::new(field);

    match solver.start(print_steps) {
        SolverSolution::NoSolution(step_count, remaining_mines, hidden_count, _states) => {
            println!("No solution found. Stopped after {} steps.\nRemaining Mines: {} ({} %)\nPercentage Solved: {} %", step_count.to_string().red(), remaining_mines.to_string().red(), format!("{:.3}", (remaining_mines as f64 / mines as f64 * 100_f64)).blue(), format!("{:.3}", (100_f64 - hidden_count as f64 / (width * height) as f64 * 100_f64)).blue());
        }
        SolverSolution::FoundSolution(steps) => {
            println!("Found a solution after {} steps.", steps.get_steps().to_string().green());
            println!("Complexity: {}", steps.get_complexity().blue());
            println!("Average: {}", format!("{:.4}", steps.get_average()).yellow());
        }
        SolverSolution::NeverStarted => {
            unreachable!("Solver never started, this shouldn't happen!");
        }
    }
}