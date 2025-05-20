use crate::field_generator::*;

mod box_logic;
mod permutation_checker;
mod solver;

enum SolverSolution {
    NoSolution,
    FoundSolution,
}

#[derive(Clone, PartialEq)]
enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
pub struct MineSweeperSolver<M>
where
    M: MineSweeperField + Clone,
{
    field:  M,
    state: Vec<Vec<MineSweeperCellState>>,
    flag_count: u32,
    hidden_count: u32,
    remaining_mines: u32
}

pub fn minesweeper_solver(field: impl MineSweeperField + Clone) {
    solver::start(field);
}