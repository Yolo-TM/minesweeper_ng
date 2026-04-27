#[macro_use]
mod r#macro;
use super::{Finding, Solver};

mod mine_count;
mod reduction;
mod sat_solver;
mod simple;

define_strategies! {
    Simple => simple,
    Complex => reduction,
    Sat => sat_solver,
    MineCount => mine_count,
}

/*
Current State:

- Solver is not able to solve every solveable field
  - fields where islands which are not accessible / are completely surrounded by mines
  - MineCount strategy could be improved
  - fields with multiple 50/50 islands which are not bordering each other but are still solveable with minecount
  -> could be solved with so called meta ng strategies
- Solver could be optimized for even more for performance
  - gpu / shader computing for permutations / more parallelism
  - more strategies which can solve small islands possibly
  - strategy for pattern recognition (1-3-1 corner etc) so the sat doesnt have to solve them
*/
