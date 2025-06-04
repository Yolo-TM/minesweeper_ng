mod minesweeper_cell;
mod minesweeper_field;

mod minesweeper_field_creation;
mod minesweeper_field_iterator;
mod surrounding_fields_iterator;

mod random_generation_field;

pub use minesweeper_cell::MineSweeperCell;
pub use minesweeper_field::MineSweeperField;
pub use minesweeper_field_creation::MineSweeperFieldCreation;
pub use minesweeper_field_iterator::MineSweeperFieldIterator;
pub use surrounding_fields_iterator::SurroundingFieldsIterator;
pub use random_generation_field::RandomGenerationField;

pub fn minesweeper_field(width: u32, height: u32, mines: MineSweeperFieldCreation) -> RandomGenerationField {
    RandomGenerationField::new(width, height, mines)
}