use crate::minesweeper_field::{
    MineSweeperField,
    MineSweeperCell,
    MineSweeperCellState,
};
use crate::minesweeper_game::MinesweeperGame;
use colored::Colorize;

enum SolverSolution {
    Error,
    NoSolution,
    FoundSolution(MinesweeperGame),
}

pub fn minesweeper_solver(field: MineSweeperField) {

    let mut game = MinesweeperGame {
        state: vec![vec![MineSweeperCellState::Hidden; field.width as usize]; field.height as usize],
        field: field,
        game_over: false,
        time: 0,
    };
    println!("Solving the field...");
    println!("Width: {}, Height: {}, Mines: {}", game.field.width.to_string().green(), game.field.height.to_string().green(), game.field.mines.to_string().red());

    let empty_cell = get_empty_cell(&game.field);
    if empty_cell.is_none() {
        println!("No empty cell found.");
        return;
    }

    match solver_start(game, empty_cell.unwrap().0, empty_cell.unwrap().1) {
        SolverSolution::Error => {
            println!("Error: No solution found.");
        }
        SolverSolution::NoSolution => {
            println!("No solution found.");
        }
        SolverSolution::FoundSolution(game) => {
            println!("Found a solution!");
            game.print();
        }
    }
}

fn get_empty_cell(field: &MineSweeperField) -> Option<(usize, usize)> {
    for (i, row) in field.board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == MineSweeperCell::Empty {
                return Some((j, i));
            }
        }
    }
    return None
}

fn solver_start(mut game: MinesweeperGame, start_x: usize, start_y: usize) -> SolverSolution {
    let step_count: u64 = 1;
    game.reveal_field(start_x, start_y);

    return solver_recursive(game, step_count);
}

fn solver_recursive(mut game: MinesweeperGame, mut step_count: u64) -> SolverSolution {
    if game.game_over {
        return SolverSolution::Error;
    }
    println!("Solving Step: {}", step_count.to_string().green());
    game.print();
    step_count += 1;


    return SolverSolution::NoSolution;
}