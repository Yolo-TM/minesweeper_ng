use crate::field_generator::*;
use crate::minesweeper_solver::{SolverSolution, MineSweeperSolver, MineSweeperCellState, search_for_islands};
use std::collections::HashMap;
use rand::{rng, prelude::IndexedRandom};

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

        loop {
            match solver.start(true) {
                SolverSolution::NoSolution(_steps, mines, hidden, states) => {
                    eprintln!("No solution found, trying to move a mine. (Iteration: {})", iteration);
                    iteration += 1;
                    self.show();

                    let islands = search_for_islands(self.width, self.height, &self.board, &states);
                    if islands.len() > 1 {
                        self.multiple_islands(islands, &states);
                    } else if islands.len() == 1 {
                        self.single_island(mines, hidden, islands[0].clone(), &states);
                        break;
                    } else {
                        unreachable!("A Game with no islands should be solved!");
                    }

                    self.assign_numbers();
                    self.show();
                    continue;
                }
                SolverSolution::FoundSolution(_, _) => {
                    println!("Solution found after {} iterations", iteration);
                    break;
                }
                SolverSolution::NeverStarted => {
                    unreachable!("Solver never started, this should not happen!");
                }
            }
        }
    }

    fn multiple_islands(&mut self, islands: Vec<Vec<(u32, u32)>>, states: &Vec<Vec<MineSweeperCellState>>) {
        // Remember which island we already edited
        let mut edited_islands = vec![false; islands.len()];

        // For each island, collect border fields (fields adjacent to a revealed cell)
        let mut mines_at_border: HashMap<usize, Vec<(u32, u32)>> = HashMap::new();

        // get all mines which are completely encapsulated by flags or map border
        let mut encapsulated_mines = vec![];

        for (i, island) in islands.iter().enumerate() {
            let mut border = vec![];
            let mut encapsulated = true;

            for &(x, y) in island {
                for (nx, ny) in (SurroundingFieldsIterator{
                    x,
                    y,
                    width: self.width,
                    height: self.height,
                    range: 1,
                    dx: -1,
                    dy: -1,
                }) {
                    match states[nx as usize][ny as usize] {
                        MineSweeperCellState::Revealed => {
                            border.push((x, y));
                            encapsulated = false;
                            break;
                        },
                        _ => {}
                    }
                }
            }

            if encapsulated {
                edited_islands[i] = true;
                for &(x, y) in island {
                    if self.get_cell(x, y) == MineSweeperCell::Mine {
                        encapsulated_mines.push((x, y));
                    }
                }
            } else {
                mines_at_border.insert(i, border);
            }
        }

        for (x, y) in encapsulated_mines {
            self.set_cell(x, y, MineSweeperCell::Empty);
            println!("Encapsulated mine at ({}, {}) removed", x, y);

            // Try move it to another island
            let mut moved_successfully = false;
            for (i, island) in islands.iter().enumerate() {
                if edited_islands[i] {
                    continue;
                }

                if let Some(&(bx, by)) = island.iter().find(|&&(bx, by)| self.get_cell(bx, by) != MineSweeperCell::Mine) {
                    println!("Moving mine to cell ({}, {})", bx, by);
                    self.set_cell(bx, by, MineSweeperCell::Mine);
                    moved_successfully = true;

                    if mines_at_border.contains_key(&i) && mines_at_border[&i].contains(&(bx, by)) {
                        println!("Mine moved to border cell, removing from border");
                        mines_at_border.remove(&i);
                        edited_islands[i] = true;
                    }
                    break;
                }
            }

            if !moved_successfully {
                // If no border found, just move the mine to a random empty cell
                let mut empty_cells = vec![];
                for x in 0..self.width {
                    for y in 0..self.height {
                        if self.get_cell(x, y) == MineSweeperCell::Empty {
                            empty_cells.push((x, y));
                        }
                    }
                }
                if let Some(&(x, y)) = empty_cells.choose(&mut rng()) {
                    println!("No place in an island found, moving to random empty cell ({}, {})", x, y);
                    self.set_cell(x, y, MineSweeperCell::Mine);
                }
            }
        }

        // Move a mine from one island to another one
        // or to another place at the border
        let unedited_count = edited_islands.iter().filter(|&&edited| !edited).count();
        if unedited_count > 0 {
            let mut moving: Option<(u32, u32, usize)> = None;

            for (i, bordering) in &mines_at_border {
                if edited_islands[*i] {
                    continue;
                }

                match moving {
                    Some((_x, _y, _)) => {
                        for &(dx, dy) in bordering {
                            if self.get_cell(dx, dy) != MineSweeperCell::Mine {
                                self.set_cell(dx, dy, MineSweeperCell::Mine);
                                moving = None;
                                println!("to border cell ({}, {})", dx, dy);
                                break;
                            }
                        }
                    }
                    None => {
                        for &(x, y) in bordering {
                            if self.get_cell(x, y) == MineSweeperCell::Mine {
                                self.set_cell(x, y, MineSweeperCell::Empty);
                                moving = Some((x, y, *i));
                                println!("Moving mine from border cell ({}, {}) : ", x, y);
                                break;
                            }
                        }
                    }
                }
            }

            if let Some((x, y, i)) = moving {
                for (dx, dy) in mines_at_border[&i].clone() {
                    if self.get_cell(dx, dy) != MineSweeperCell::Mine && !(dx == x && dy == y) {
                        self.set_cell(dx, dy, MineSweeperCell::Mine);
                        println!("to border cell ({}, {}).", dx, dy);
                        break;
                    }
                }
            }
        }


    }

    fn single_island(&mut self, _mines: u32, _hidden: u32, islands: Vec<(u32, u32)>, states: &Vec<Vec<MineSweeperCellState>>) {
        // check if there are bordering cells, move a mine to another bordering cell or another hidden cell
        let mut bordering_cells = vec![];

        for &(x, y) in &islands {
            for (nx, ny) in (SurroundingFieldsIterator{
                x,
                y,
                width: self.width,
                height: self.height,
                range: 1,
                dx: -1,
                dy: -1,
            }) {
                if states[nx as usize][ny as usize] == MineSweeperCellState::Revealed {
                    bordering_cells.push((x, y));
                    break;
                }
            }
        }

        if bordering_cells.is_empty() {
            // uhm thats bad ...
            // we need to move a found mine to open up the island
        } else {
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
    fn test_no_guess_field_creation() {
        let field = NoGuessField::new(10, 10, MineSweeperFieldCreation::FixedCount(20));
        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 10);
        assert_eq!(field.get_mines(), 20);
        assert_eq!(field.get_start_field(), (0, 0)); // Assuming the start field is always (0, 0)
    }

    #[test]
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