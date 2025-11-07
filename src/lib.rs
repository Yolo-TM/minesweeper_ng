mod generator;
mod normal_field;
mod solver;
//mod noguess_field;

// Re-export commonly used types for cleaner imports
pub use normal_field::{
    Cell, DefinedField, MineSweeperField, Mines, RandomField, get_evil_ng_field,
    get_small_test_field,
};

pub use generator::generate;
pub use solver::{create_solver, is_solvable, solve_field};

//pub use noguess_field::minesweeper_ng_field;
pub fn minesweeper_ng_field(width: u32, height: u32, mines: Mines) -> impl MineSweeperField {
    RandomField::new(width, height, mines)
}
