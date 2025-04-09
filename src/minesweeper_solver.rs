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
    for (y, row) in field.board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == MineSweeperCell::Empty {
                return Some((x, y));
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
        game.flag_all_hidden_cells();
        return SolverSolution::FoundSolution(game);
    }

    for y in 0..game.field.height as usize {
        for x in 0..game.field.width as usize {
            if game.state[y][x] == MineSweeperCellState::Revealed
            && matches!(game.field.board[y][x], MineSweeperCell::Number(_))
            && game.has_unrevealed_neighbours(x, y) {
                let flag_count = game.get_surrounding_flag_count(x, y);
                let number = game.field.board[y][x].get_number().unwrap_or(0);
                let unrevealed_count = game.get_surrounding_unrevealed_count(x, y);

                let needed_mines = number - flag_count as u8;
                if needed_mines == unrevealed_count {
                    game.flag_surrounding_cells(x, y);
                }
                if needed_mines == 0 {
                    game.reveal_surrounding_cells(x, y);
                }
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
            println!("All non mine cells revealed and all mines flagged. Game solved!");
            return true;
    }

    return false
}