#[macro_use]
mod r#macro;
use super::{Solver, Finding};

mod reduction;
mod simple;

define_strategies! {
    Simple => simple,
    Reduction => reduction,
}
