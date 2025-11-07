#[macro_use]
mod r#macro;
use super::Solver;

mod boxes;
mod reduction;
mod simple;

define_strategies! {
    Simple => simple,
    Reduction => reduction,
    Boxes => boxes,
}
