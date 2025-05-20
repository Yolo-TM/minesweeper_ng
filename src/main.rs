#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]

mod field_generator;
mod ng_generator;
mod minesweeper_solver;

use field_generator::*;

fn main() {
    let start = std::time::Instant::now();
    let field = minesweeper_field(45, 26, MineSweeperFieldCreation::Percentage(0.2));
    field.show();
    //minesweeper_solver::minesweeper_solver(field);

    let ng_field = ng_generator::get_evil_ng_field();
    ng_field.show();
    //minesweeper_solver::minesweeper_solver(ng_field);

    let small_field = ng_generator::get_small_test_field();
    small_field.show();
    minesweeper_solver::minesweeper_solver(small_field);
    println!("Time elapsed: {:?}", start.elapsed());
}
