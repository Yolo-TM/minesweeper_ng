mod components;
mod constraint_builder;
mod sat_solving;
mod solve;

#[cfg(test)]
mod tests;

#[cfg(test)]
use components::find_independent_components;

use super::{Finding, Solver};
pub use solve::solve;
