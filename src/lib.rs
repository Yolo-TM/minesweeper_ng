mod minesweeper_field;
mod noguess_field;
mod solver;

pub use minesweeper_field::{
    Cell, CellState, DefinedField, FieldError, InvalidTransition, MineSweeperField,
    MineSweeperFieldDisplay, MineSweeperFieldFileIO, MineSweeperFieldMetrics, Mines, RandomField, SortedCells,
    SurroundingCells, Visibility, game_ops,
};
pub use noguess_field::NoGuessField;

#[cfg(feature = "json")]
pub use minesweeper_field::MineSweeperFieldJson;
#[cfg(feature = "svg")]
pub use minesweeper_field::{MineSweeperFieldSvg, SVG_Mode};

pub use solver::{Finding, Solver, create_solver, is_solvable};
