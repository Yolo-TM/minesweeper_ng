mod minesweeper_field;
mod solver;

use solver::minesweeper_solver;
use minesweeper_field::{MineSweeperField, get_ng_minesweeper_field};

fn main() {
    let field2 = MineSweeperField::new_percentage(31, 30, 0.2);
    field2.println();
    minesweeper_solver(field2);
}
