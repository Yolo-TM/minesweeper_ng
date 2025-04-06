mod minesweeper_field;
mod minesweeper_solver;
mod minesweeper_game;
use minesweeper_field::MineSweeperField;

fn main() {
    let field = MineSweeperField::new(10, 10, 20);
    field.println();
    minesweeper_solver::minesweeper_solver(field);
}