mod minesweeper_field;
mod minesweeper_solver;
mod minesweeper_game;
use minesweeper_field::{MineSweeperField, get_ng_minesweeper_field};

fn main() {
    //let field = MineSweeperField::new(10, 10, 20);
    //field.println();
    //println!();

    let field2 = get_ng_minesweeper_field();
    field2.println();
    minesweeper_solver::minesweeper_solver(field2);
}