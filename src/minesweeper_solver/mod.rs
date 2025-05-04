use crate::field_generator::MineSweeperField;

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
struct MineSweeperSolver{
    pub field: MineSweeperField,
    pub state: Vec<Vec<MineSweeperCellState>>,
    pub flag_count: u64,
    pub hidden_count: u64,
    pub remaining_mines: u64
}

pub fn minesweeper_solver(field: MineSweeperField) {
    solver::start(field);
}