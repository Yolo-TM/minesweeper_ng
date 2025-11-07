mod cell;
mod defined_field;
mod iterators;
mod mines;
mod random_field;
mod r#trait;

pub use cell::Cell;
pub use defined_field::DefinedField;
pub use iterators::{SortedCells, SurroundingCells};
pub use mines::Mines;
pub use random_field::RandomField;
pub use r#trait::MineSweeperField;

#[allow(dead_code)]
pub fn minesweeper_field(width: u32, height: u32, mines: Mines) -> impl MineSweeperField {
    RandomField::new(width, height, mines)
}

#[allow(dead_code)]
pub fn get_evil_ng_field() -> impl MineSweeperField {
    DefinedField::from_file("./src/generated/testing/evil_ng_field.minesweeper").unwrap()
}

#[allow(dead_code)]
pub fn get_small_test_field() -> impl MineSweeperField {
    DefinedField::from_file("./src/generated/testing/hard.minesweeper").unwrap()
}
