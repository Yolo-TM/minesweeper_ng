#[macro_use]
mod r#macro;
use super::Solver;

mod simple;
mod reduction;
mod boxes;

define_strategies! {
    Simple => simple,
    Reduction => reduction,
    Boxes => boxes,
}