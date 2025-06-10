use std::collections::HashMap;
use crate::MineSweeperField;
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
    FoundSolution(u32, HashMap<SolverStep, u32>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SolverStep{
    Basic,
    Reduction,
    Complex,
    Permutations
}

impl SolverStep {
    pub fn to_string(&self) -> String {
        match self {
            SolverStep::Basic => "Basic Count".to_string(),
            SolverStep::Reduction => "Basic Reduction".to_string(),
            SolverStep::Complex => "Extended Reduction".to_string(),
            SolverStep::Permutations => "Permutations".to_string(),
        }
    }

    pub fn to_number(&self) -> u8 {
        match self {
            SolverStep::Basic => 1,
            SolverStep::Reduction => 2,
            SolverStep::Complex => 3,
            SolverStep::Permutations => 4,
        }
    }
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
    logic_levels: HashMap<SolverStep, u32>
}

pub fn solve(field: impl MineSweeperField, print_steps: bool) {
    let (width, height, mines) = field.get_dimensions();
    let mut solver = MineSweeperSolver::new(field);

    match solver.start(print_steps) {
        SolverSolution::NoSolution(step_count, remaining_mines, hidden_count, _states) => {
            println!("No solution found. Stopped after {} steps.\nRemaining Mines: {} ({} %)\nPercentage Solved: {} %", step_count.to_string().red(), remaining_mines.to_string().red(), format!("{:.3}", (remaining_mines as f64 / mines as f64 * 100_f64)).blue(), format!("{:.3}", (100_f64 - hidden_count as f64 / (width * height) as f64 * 100_f64)).blue());
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
                average += *count as f64 * level.to_number() as f64;
            }
            average /= step_count as f64;

            println!("Complexity: {}", complexity_str);
            println!("Average: {:.4}", average.to_string().yellow());
        }
        SolverSolution::NeverStarted => {
            unreachable!("Solver never started, this shouldn't happen!");
        }
    }
}