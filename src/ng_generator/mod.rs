mod test_ng_field;
mod no_guess_field;

use no_guess_field::NoGuessField;
use crate::field_generator::*;

pub use test_ng_field::{get_evil_ng_field, get_small_test_field};

pub fn minesweeper_ng_field(
    width: u32,
    height: u32,
    mines: MineSweeperFieldCreation,
) -> NoGuessField {
    NoGuessField::new(width, height, mines)
}