#[macro_use]
mod r#macro;
use super::{CellState, Solver};
use crate::Cell;

mod simple;
mod complex;

define_strategies! {
    Simple => simple,
    Complex => complex,
}