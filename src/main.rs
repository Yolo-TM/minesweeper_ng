#![cfg_attr(debug_assertions, allow(dead_code))]

mod ng_generator;
mod field_generator;

use field_generator::minesweeper_field;
use ng_generator::minesweeper_solver;
use ng_generator::get_evil_field;

fn main() {
    let field = minesweeper_field(45, 26, 0.22);
    field.print();
    minesweeper_solver(field);

    let ng_field = get_evil_field();
    ng_field.print();
    minesweeper_solver(ng_field);
}
