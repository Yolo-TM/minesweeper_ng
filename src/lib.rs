mod minesweeper_field;
mod noguess_field;
mod solver;

pub use minesweeper_field::{
    Cell, DefinedField, FieldError, MineSweeperField, MineSweeperFieldDisplay,
    MineSweeperFieldFileIO, Mines, RandomField,
};
pub use noguess_field::NoGuessField;

#[cfg(feature = "json")]
pub use minesweeper_field::MineSweeperFieldJson;
#[cfg(feature = "svg")]
pub use minesweeper_field::{MineSweeperFieldSvg, SVG_Mode};

pub use solver::{create_solver, is_solvable};
