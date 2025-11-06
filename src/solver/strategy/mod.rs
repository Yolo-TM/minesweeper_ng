#[macro_use]
mod r#macro;
use super::Solver;

mod simple;
mod complex;

define_strategies! {
    Simple => simple,
    Complex => complex,
}