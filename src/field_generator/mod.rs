mod minesweeper_field_iterator;
mod surrounding_fields_iterator;

pub mod minesweeper_field;
pub mod minesweeper_cell;
pub use minesweeper_field::MineSweeperField;
pub use minesweeper_cell::MineSweeperCell;

pub fn minesweeper_field(width: usize, height: usize, percentage: f32) -> MineSweeperField {
    MineSweeperField::new(width, height, percentage)
}

pub fn minesweeper_field_fixed_mines(width: usize, height: usize, mines: u64) -> MineSweeperField {
    MineSweeperField::new_field(width, height, mines)
}