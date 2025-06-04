mod test_fields;
mod no_guess_field;

use no_guess_field::NoGuessField;
use crate::field_generator::{MineSweeperField, MineSweeperFieldCreation};

pub use test_fields::{TestField, get_evil_ng_field, get_small_test_field};

pub fn minesweeper_ng_field(width: u32, height: u32, mines: MineSweeperFieldCreation)
-> NoGuessField {
    NoGuessField::new(width, height, mines)
}