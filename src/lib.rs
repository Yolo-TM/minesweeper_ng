mod generator;
pub(crate) mod normal_field;
pub(crate) mod solver;
mod noguess_field;

// Re-export commonly used types for cleaner imports
pub use normal_field::{
    MineSweeperField,
    Mines,
    Cell,
    RandomField,
    DefinedField,
    get_evil_ng_field,
    get_small_test_field,
};

pub use solver::{MineSweeperSolver, SolverSolution, MineSweeperCellState, solve};
pub use noguess_field::minesweeper_ng_field;
pub use generator::generate;