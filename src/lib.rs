mod ng_generator;
pub(crate) mod minesweeper_solver;
pub(crate) mod field_generator;

// Re-export commonly used types for cleaner imports
pub use field_generator::{
    MineSweeperField,
    MineSweeperFieldCreation,
    MineSweeperCell,
    minesweeper_field,
    MineField,
    get_evil_ng_field,
    get_small_test_field,
    RandomField
};

pub use ng_generator::minesweeper_ng_field;
pub use minesweeper_solver::{MineSweeperSolver, SolverSolution, MineSweeperCellState, solve};
