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

    let mut game = MinesweeperGame::new(field);

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
    if game.game_over || step_count > 10 {
        return SolverSolution::Error;
    }
    println!("Solving Step: {}", step_count.to_string().green());
    game.field.board[8][0] = MineSweeperCell::Number(1);
    game.print();
    step_count += 1;

    if check_game_state(&game) {
        return SolverSolution::FoundSolution(game);
    }

    for (i, row) in game.state.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == MineSweeperCellState::Revealed
                && matches!(game.field.board[i][j], MineSweeperCell::Number(_))
                && has_unrevealed_neighbours(i, j, &game) {
                let flag_count = game.get_surrounding_flag_count(j, i);
                let number = game.field.board[i][j].get_number().unwrap_or(0);
                let unrevealed_count = game.get_surrounding_unrevealed_count(j, i);
                
            }
        }
    }

    return solver_recursive(game, step_count);
}

fn check_game_state(game: &MinesweeperGame) -> bool {

    if game.flag_count < 0  {
            println!("Flag count is negative. Game over!");
            return true;
    }
    if game.hidden_count < 0 {
            println!("All cells revealed. Game solved!");
            return true;
    }
    if game.flag_count + game.hidden_count == game.field.mines {
            println!("All cells revealed and all mines flagged. Game solved!");
            return true;
    }

    return false
}

fn has_unrevealed_neighbours(y: usize, x: usize, game: &MinesweeperGame) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = (x as isize + j) as usize;
            let new_y = (y as isize + i) as usize;
            if new_x < game.field.width as usize && new_y < game.field.height as usize {
                if game.state[new_y][new_x] == MineSweeperCellState::Hidden {
                    return true;
                }
            }
        }
    }

    return false;
}