use std::vec;

use crate::*;
use crate::minesweeper_solver::search_for_islands;
use colored::Colorize;

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
        let mut solver = MineSweeperSolver::new(self.clone());
        let mut iteration: u32 = 0;
        solver.reveal_field(self.get_start_field().0, self.get_start_field().1);

        loop {
            iteration += 1;
            match solver.continue_solving(true) {
                SolverSolution::NoSolution(_steps, mines, hidden, states) => {
                    eprintln!("No solution found, trying to move a mine. (Iteration: {}, Status: {}% solved)", iteration, format!("{:.3}", (100_f64 - hidden as f64 / (self.width * self.height) as f64 * 100_f64)).blue());

                    self.show();
                    self.make_solvable(&mut solver, mines, hidden, states);
                    self.show();

                    continue;
                }
                SolverSolution::FoundSolution(step_count, complexity) => {
                    // concatenate complexity levels into a string
                    let complexity_str: String = complexity.iter()
                        .map(|(k, v)| format!("{}: {}", k.to_string().blue(), v.to_string().green()))
                        .collect::<Vec<String>>()
                        .join(", ");

                    // Calculate average
                    let mut average: f64 = 0.0;
                    for (level, count) in &complexity {
                        average += *count as f64 * level.to_number() as f64;
                    }
                    average /= step_count as f64;
                    println!("Complexity: {}", complexity_str);
                    println!("Average: {}", format!("{:.4}", average).yellow());
                }
                SolverSolution::NeverStarted => {
                    unreachable!("Solver never started, this shouldn't happen!");
                }
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
            start_field: random_field.get_start_field(),
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

    fn get_start_field(&self) -> (u32, u32) {
        self.start_field
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
        assert_eq!(field.get_start_field(), (0, 0)); // Assuming the start field is always (0, 0)
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

        assert!(matches!(solution, SolverSolution::FoundSolution(_, _)));
    }

}