#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]

mod field_generator;
mod ng_generator;
mod minesweeper_solver;

fn main() {
    let start = std::time::Instant::now();
    let field = field_generator::minesweeper_field(45, 26, 0.22);
    minesweeper_solver::minesweeper_solver(field);

    let ng_field = ng_generator::get_evil_field();
    minesweeper_solver::minesweeper_solver(ng_field);

    let small_field = ng_generator::get_small_test_field();
    minesweeper_solver::minesweeper_solver(small_field);
    println!("Time elapsed: {:?}", start.elapsed());
}
