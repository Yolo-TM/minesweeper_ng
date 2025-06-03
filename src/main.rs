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
    field.show();
    minesweeper_solver::solve(field, true);

    println!("Time elapsed: {:?}", start.elapsed());
}
