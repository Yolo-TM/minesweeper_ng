#![cfg_attr(debug_assertions, allow(dead_code))]

pub mod field_generator;
pub mod ng_generator;

use field_generator::minesweeper_field;
use ng_generator::{get_evil_field, minesweeper_solver};

fn main() {
    let field = minesweeper_field(45, 26, 0.22);
    field.print();
    minesweeper_solver(field);

    let ng_field = get_evil_field();
    ng_field.print();
    minesweeper_solver(ng_field);
}
