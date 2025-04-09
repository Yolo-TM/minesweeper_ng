mod minesweeper_field;
mod minesweeper_solver;

use minesweeper_solver::simple_solver::simple_minesweeper_solver;
use minesweeper_field::{MineSweeperField, get_ng_minesweeper_field};

fn main() {
    let field2 = get_ng_minesweeper_field();
    field2.println();
    simple_minesweeper_solver(field2);

    println!();
    println!();

    let field = MineSweeperField::new_percentage(10, 10, 0.2);
    field.println();
    simple_minesweeper_solver(field);
}