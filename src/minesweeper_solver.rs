use crate::minesweeper_field::{
    MineSweeperField,
    MineSweeperCell,
    MineSweeperCellState,
};
use crate::minesweeper_game::MinesweeperGame;
use colored::Colorize;

pub fn minesweeper_solver(field: MineSweeperField) {

    let game = MinesweeperGame {
        state: vec![vec![MineSweeperCellState::Hidden; field.width as usize]; field.height as usize],
        field: field,
        game_over: false,
        time: 0,
    };
    println!("Solving the field...");
    println!("Width: {}, Height: {}, Mines: {}", game.field.width.to_string().green(), game.field.height.to_string().green(), game.field.mines.to_string().red());

    // Get an Empty cell to start with
    let mut empty_cell = None;
    for (i, row) in game.field.board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == MineSweeperCell::Empty {
                empty_cell = Some((i, j));
                break;
            }
        }
        if empty_cell.is_some() {
            break;
        }
    }

    println!("Starting from cell: {:?}", empty_cell.unwrap());
}