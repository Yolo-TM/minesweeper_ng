use std::collections::HashMap;
use crate::field_generator::*;
use colored::Colorize;

mod box_logic;
mod permutation_checker;
mod solver;

pub use solver::start;

pub enum SolverSolution {
    NoSolution(u32, u32, u32, Vec<Vec<(u32, u32)>>),
    FoundSolution(u32, HashMap<u8, u32>),
}

#[derive(Clone, PartialEq)]
pub enum MineSweeperCellState {
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
    match start(field) {
        SolverSolution::NoSolution(step_count, remaining_mines, hidden_count, _islands) => {
            println!("No solution found. Stopped after {} steps. (Remaining Mines: {}, Hidden Fields: {})", step_count.to_string().red(), remaining_mines.to_string().red(), hidden_count.to_string().blue());
        }
        SolverSolution::FoundSolution(step_count, _complexity) => {
            println!("Found a solution after {} steps.", step_count.to_string().green());
        }
    }
}