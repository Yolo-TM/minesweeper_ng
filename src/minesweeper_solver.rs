pub mod simple_solver;

enum SolverSolution {
    NoSolution,
    FoundSolution,
}

#[derive(Clone, PartialEq)]
pub enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}
