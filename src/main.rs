#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(unused_imports))]

pub mod field_generator;
pub mod ng_generator;

use field_generator::minesweeper_field;
use ng_generator::{get_evil_field, minesweeper_solver};

fn main() {
    //let field = minesweeper_field(45, 26, 0.22);
    //field.print();
    //minesweeper_solver(field);

    //let ng_field = get_evil_field();
    //ng_field.print();
    //minesweeper_solver(ng_field);

    let small_field = ng_generator::get_small_test_field();
    small_field.print();
    minesweeper_solver(small_field);
}
