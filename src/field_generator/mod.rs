pub mod minesweeper_field;
pub mod minesweeper_cell;

pub fn minesweeper_field(width: usize, height: usize, percentage: f32) -> minesweeper_field::MineSweeperField {
    minesweeper_field::MineSweeperField::new(width, height, percentage)
}

pub fn minesweeper_field_fixed_mines(width: usize, height: usize, mines: u64) -> minesweeper_field::MineSweeperField {
    minesweeper_field::MineSweeperField::new_field(width, height, mines)
}