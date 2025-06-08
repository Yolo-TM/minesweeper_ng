mod minesweeper_cell;
mod minesweeper_field;

mod minesweeper_field_creation;
mod minesweeper_field_iterator;
mod surrounding_fields_iterator;

mod random_field;
mod mine_field;

pub use minesweeper_cell::MineSweeperCell;
pub use minesweeper_field::MineSweeperField;
pub use minesweeper_field_creation::MineSweeperFieldCreation;
pub use minesweeper_field_iterator::MineSweeperFieldIterator;
pub use surrounding_fields_iterator::SurroundingFieldsIterator;
pub use random_field::RandomField;
pub use mine_field::MineField;
pub use mine_field::{get_evil_ng_field, get_small_test_field};

pub fn minesweeper_field(width: u32, height: u32, mines: MineSweeperFieldCreation) -> RandomField {
    RandomField::new(width, height, mines)
}