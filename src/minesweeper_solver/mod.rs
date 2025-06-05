use std::collections::HashMap;
use crate::field_generator::*;
use colored::Colorize;

mod box_logic;
mod permutation_checker;
mod solver;
mod islands;

pub use islands::{search_for_islands, merge_islands};

#[derive(Clone, PartialEq, Debug)]
pub enum SolverSolution {
    NeverStarted,
    NoSolution(u32, u32, u32, Vec<Vec<MineSweeperCellState>>),
    FoundSolution(u32, HashMap<u8, u32>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
pub struct MineSweeperSolver<M: MineSweeperField> {
    field:  M,
    state: Vec<Vec<MineSweeperCellState>>,
    flag_count: u32,
    hidden_count: u32,
    remaining_mines: u32,
    solution: SolverSolution,
    step_count: u32,
    logic_levels: HashMap<u8, u32>
}

pub fn solve(field: impl MineSweeperField, print_steps: bool) {
    let mut solver = MineSweeperSolver::new(field);

    match solver.start(print_steps) {
        SolverSolution::NoSolution(step_count, remaining_mines, hidden_count, _states) => {
            println!("No solution found. Stopped after {} steps. (Remaining Mines: {}, Hidden Fields: {})", step_count.to_string().red(), remaining_mines.to_string().red(), hidden_count.to_string().blue());
        }
        SolverSolution::FoundSolution(step_count, complexity) => {
            println!("Found a solution after {} steps.", step_count.to_string().green());

            // concatenate complexity levels into a string
            let complexity_str: String = complexity.iter()
                .map(|(k, v)| format!("{}: {}", k.to_string().blue(), v.to_string().green()))
                .collect::<Vec<String>>()
                .join(", ");

            // Calculate average
            let mut average: f64 = 0.0;
            for (level, count) in &complexity {
                average += *count as f64 * *level as f64;
            }
            average /= step_count as f64;

            println!("Complexity: {}", complexity_str);
            println!("Average: {:.6}", average.to_string().yellow());
        }
        SolverSolution::NeverStarted => unreachable!(),
    }
}