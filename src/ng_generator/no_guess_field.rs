use crate::field_generator::*;
use crate::minesweeper_solver::{SolverSolution, solver::start};

#[derive(Clone)]
pub struct NoGuessField {
    width: u32,
    height: u32,
    mines: u32,
    start_field: (u32, u32),
    board: Vec<Vec<MineSweeperCell>>,
}

impl NoGuessField {
    fn initialize(&mut self) {
        loop {
            match start(self.clone()) {
                SolverSolution::NoSolution(_) => {
                    println!("No solution found, trying to move a mine.");
// Create the NG Minesweeper Field

/*
if not solvable, check if there are multiple islands
    if yes, take a random mine from an island border and move it to another island border
        then try solving again? possible from the current sovler state theoretically
    if no, check minecount, unrevealed etc if it makes sense to just move the mine somewhere else (from the border away)
*/
                    continue;
                }
                SolverSolution::FoundSolution(_) => {
                    break;
                }
            }
        }
    }
}

impl MineSweeperField for NoGuessField {
    #[track_caller]
    fn new(width: u32, height: u32, mines: MineSweeperFieldCreation) -> Self {
        let random_field = minesweeper_field(width, height, mines.clone());

        let mut field = NoGuessField {
            width,
            height,
            mines: mines.get_fixed_count(width, height),
            start_field: random_field.get_start_field(),
            board: random_field.get_cells(),
        };


        field.initialize();
        field
    }

    fn get_mines(&self) -> u32 {
        self.mines
    }

    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_start_field(&self) -> (u32, u32) {
        self.start_field
    }

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell {
        self.board[x as usize][y as usize].clone()
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell) {
        self.board[x as usize][y as usize] = cell;
    }
}