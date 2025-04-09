mod minesweeper_field;
mod minesweeper_solver;
mod minesweeper_game;
use minesweeper_field::{MineSweeperField, get_ng_minesweeper_field};

fn main() {
    let field = MineSweeperField::new_percentage(10, 10, 0.2);
    field.println();
    println!();

    let field2 = get_ng_minesweeper_field();
    field2.println();
    minesweeper_solver::minesweeper_solver(field2);

    minesweeper_solver::minesweeper_solver(field);
}