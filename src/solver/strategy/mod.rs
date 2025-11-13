#[macro_use]
mod r#macro;
use super::{Solver, Finding};

mod permutations;
mod reduction;
mod simple;

define_strategies! {
    Simple => simple,
    Reduction => reduction,
    Permutations => permutations,
}
