use std::collections::HashMap;
use crate::field_generator::*;
use colored::Colorize;

mod box_logic;
mod permutation_checker;
mod solver;
mod islands;

pub use solver::{start, continue_solving};
pub use islands::{search_for_islands, merge_islands};

#[derive(PartialEq, Debug)]
pub enum SolverSolution {
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
    remaining_mines: u32
}

pub fn solve(field: impl MineSweeperField, print_steps: bool) {
    match start(field, print_steps) {
        SolverSolution::NoSolution(step_count, remaining_mines, hidden_count, _states) => {
            println!("No solution found. Stopped after {} steps. (Remaining Mines: {}, Hidden Fields: {})", step_count.to_string().red(), remaining_mines.to_string().red(), hidden_count.to_string().blue());
        }
        SolverSolution::FoundSolution(step_count, complexity) => {
            println!("Found a solution after {} steps.", step_count.to_string().green());

            let complexity_str: String = complexity.iter()
                .map(|(k, v)| format!("{}: {}", k.to_string().blue(), v.to_string().green()))
                .collect::<Vec<String>>()
                .join(", ");

            println!("Complexity: {}", complexity_str);
        }
    }
}