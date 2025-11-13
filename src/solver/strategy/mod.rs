#[macro_use]
mod r#macro;
use super::{Finding, Solver};

mod permutations;
mod reduction;
mod simple;

define_strategies! {
    Simple => simple,
    Complex => reduction,
    Permutations => permutations,
}
