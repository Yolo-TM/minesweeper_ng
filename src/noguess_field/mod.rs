mod no_guess_field;

use no_guess_field::NoGuessField;
use crate::{MineSweeperField, Mines};

pub fn minesweeper_ng_field(width: u32, height: u32, mines: Mines) -> NoGuessField {
    NoGuessField::new(width, height, mines)
}