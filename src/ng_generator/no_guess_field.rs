use crate::*;
use crate::minesweeper_solver::search_for_islands;
use std::vec;
use colored::Colorize;

#[derive(Clone)]
pub struct NoGuessField {
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),
    board: Vec<Vec<MineSweeperCell>>,
}

impl NoGuessField {
    fn initialize(&mut self) {
        let mut solver = MineSweeperSolver::new(self.clone());
        let mut iteration: u32 = 0;

        solver.reveal_field(self.get_start_cell().0, self.get_start_cell().1);

        loop {
            iteration += 1;
            match solver.continue_solving(true) {
                SolverSolution::NoSolution(_steps, mines, hidden, states) => {
                    eprintln!("No solution found, editing field... (Iteration: {}, Status: {}% solved)", iteration.to_string().cyan(), format!("{:.3}", (100_f64 - hidden as f64 / (self.width * self.height) as f64 * 100_f64)).blue());
                    self.make_solvable(&mut solver, mines, hidden, states);
                    break;
                }
                SolverSolution::FoundSolution(_steps) => {
                    // Dont show anything here, the solving steps are probably not the fastest solution, bc it was not one continuous solving process
                    break;
                }
                SolverSolution::NeverStarted => {
                    unreachable!("Solver never started, this shouldn't happen!");
                }
            }
        }

        // From this Point on, the field should be solvable without any guesses.
        // just to make sure ...
        match MineSweeperSolver::new(self.clone()).start(false) {
            SolverSolution::FoundSolution(steps) => {
                println!("Found a solution after {} steps and {} iterations", steps.get_steps().to_string().green(), iteration.to_string().cyan());
                println!("Complexity: {}", steps.get_complexity().blue());
                println!("Average: {}", format!("{:.4}", steps.get_average()).yellow());
            }
            _ => {
                // Solver to dumb or not solvable ...
                panic!("The field is currently not solvable, something went wrong!");
            }
        }
        
    }

    fn make_solvable(&mut self, solver: &mut MineSweeperSolver<NoGuessField>, mines: u32, hidden: u32, states: Vec<Vec<MineSweeperCellState>>) {
        let islands = search_for_islands(self.width, self.height, &self.board, &states);

        solver.update_field(vec![]);
        solver.update_states(vec![]);
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
            start_cell: random_field.get_start_cell(),
            board: random_field.get_field(),
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

    fn get_start_cell(&self) -> (u32, u32) {
        self.start_cell
    }

    fn get_field(&self) -> Vec<Vec<MineSweeperCell>> {
        self.board.clone()
    }

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell {
        self.board[x as usize][y as usize].clone()
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell) {
        self.board[x as usize][y as usize] = cell;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_generator::MineSweeperFieldCreation;

    #[test]
    #[ignore]
    fn test_no_guess_field_creation() {
        let field = NoGuessField::new(10, 10, MineSweeperFieldCreation::FixedCount(20));
        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 10);
        assert_eq!(field.get_mines(), 20);
        assert_eq!(field.get_start_cell(), (0, 0)); // Assuming the start field is always (0, 0)
    }

    #[test]
    #[ignore]
    fn test_no_guess_field_initialization() {
        let field = NoGuessField::new(5, 5, MineSweeperFieldCreation::Percentage(0.2));
        assert_eq!(field.get_width(), 5);
        assert_eq!(field.get_height(), 5);
        assert_eq!(field.get_mines(), 5); // 20% of 25 cells
    }

    #[test]
    #[ignore]
    fn test_no_guess_field_is_solvable() {
        let field = NoGuessField::new(8, 8, MineSweeperFieldCreation::FixedCount(10));

        let solution = MineSweeperSolver::new(field).start(true);

        assert!(matches!(solution, SolverSolution::FoundSolution(_)));
    }

}