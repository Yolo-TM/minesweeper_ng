mod test_fields;
mod no_guess_field;

use no_guess_field::NoGuessField;
use crate::field_generator::*;

pub fn minesweeper_ng_field(width: u32, height: u32, mines: MineSweeperFieldCreation)
-> NoGuessField {
    NoGuessField::new(width, height, mines)
}