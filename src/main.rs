#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]

mod field_generator;
mod ng_generator;
mod minesweeper_solver;

use field_generator::*;

fn main() {
    let start = std::time::Instant::now();
    //let field = ng_generator::minesweeper_ng_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    let field = minesweeper_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    minesweeper_solver::solve(field, false);
    let field = ng_generator::get_evil_ng_field();
    minesweeper_solver::solve(field, false);
    let field = ng_generator::get_small_test_field();
    minesweeper_solver::solve(field, false);

    println!("Time elapsed: {:?}", start.elapsed());
}
