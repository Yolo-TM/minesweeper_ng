mod minesweeper_field_iterator;
mod surrounding_fields_iterator;

mod minesweeper_cell;
mod minesweeper_field;
mod random_generation_field;

pub use minesweeper_cell::MineSweeperCell;
pub use minesweeper_field::{MineSweeperField, MineSweeperFieldCreation};
pub use random_generation_field::RandomGenerationField;
pub use minesweeper_field_iterator::MineSweeperFieldIterator;
pub use surrounding_fields_iterator::SurroundingFieldsIterator;

pub fn minesweeper_field(
    width: u32,
    height: u32,
    mines: MineSweeperFieldCreation,
) -> RandomGenerationField {
    RandomGenerationField::new(width, height, mines)
}