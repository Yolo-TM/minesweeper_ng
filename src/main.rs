#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]

pub mod field_generator;
pub mod ng_generator;
pub mod minesweeper_solver;

use field_generator::minesweeper_field;
use ng_generator::get_evil_field;
use minesweeper_solver::minesweeper_solver;

fn main() {
    //let field = minesweeper_field(45, 26, 0.22);
    //field.print();
    //minesweeper_solver(field);

    //let ng_field = get_evil_field();
    //ng_field.print();
    //minesweeper_solver(ng_field);

    /*
    Solver doesnt solve 1 or 2 cell islands ??
    optimisation needs to be added  ~8 min for 29 cells is too long
    */
    let start = std::time::Instant::now();
    let small_field = ng_generator::get_small_test_field();
    minesweeper_solver(small_field);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
