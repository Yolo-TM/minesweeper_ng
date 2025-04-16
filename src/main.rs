mod minesweeper_field;
mod solver;

use solver::minesweeper_solver;
use minesweeper_field::{MineSweeperField, get_ng_minesweeper_field};

fn main() {
    let field2 = MineSweeperField::new_percentage(45, 26, 0.22);
    field2.println();
    minesweeper_solver(field2);

    let ng_field = get_ng_minesweeper_field();
    ng_field.println();
    minesweeper_solver(ng_field);
}
