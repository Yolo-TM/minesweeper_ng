mod generator;
mod normal_field;
mod solver;
//mod noguess_field;

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

pub use solver::{
    solve_field,
    is_solvable,
    create_solver,
};
pub use generator::generate;

//pub use noguess_field::minesweeper_ng_field;
pub fn minesweeper_ng_field(width: u32, height: u32, mines: Mines) -> impl MineSweeperField {
    RandomField::new(width, height, mines)
}