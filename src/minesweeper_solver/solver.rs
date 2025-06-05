use colored::Colorize;
use std::collections::HashMap;
use crate::field_generator::{MineSweeperCell, MineSweeperField};
use super::{SolverSolution, MineSweeperCellState, MineSweeperSolver};

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    pub fn new(field: M) -> Self {
        let state = vec![vec![MineSweeperCellState::Hidden; field.get_height() as usize]; field.get_width() as usize];

        MineSweeperSolver {
            state,
            flag_count: 0,
            hidden_count: (field.get_width() * field.get_height()),
            remaining_mines: field.get_mines(),
            field,
            solution: SolverSolution::NeverStarted,
            step_count: 0,
            logic_levels: HashMap::new(),
        }
    }

    pub fn start(&mut self, enable_output: bool) -> SolverSolution {
        if enable_output {
            println!("{}: Starting solver with field size {}x{} and {} mines.", "Solver started".bold(), self.field.get_width(), self.field.get_height(), self.field.get_mines());
            self.field.show();
            println!("Revealing Start field: ({}, {})", self.field.get_start_field().0, self.field.get_start_field().1);
        }

        self.reveal_field(self.field.get_start_field().0, self.field.get_start_field().1);

        return self.continue_solving(enable_output);
    }

    pub fn continue_solving(&mut self, enable_output: bool) -> SolverSolution {
        loop {
            if self.is_solved() {
                if enable_output {
                    println!("{}: Took {} steps.", "Solver finished".bold(), self.step_count);
                }

                self.flag_all_hidden_cells();
                self.solution = SolverSolution::FoundSolution(self.step_count, self.logic_levels.clone());
                return self.solution.clone();

            } else if enable_output {
                println!("{}: {}", "Solver Step".bold(), self.step_count);
                self.print();
            }

            match self.do_solving_step() {
                Some(logic_level) => {
                    if enable_output {
                        println!("{}: Applied logic level {}", "Solver Step".bold(), logic_level);
                    }
                    *self.logic_levels.entry(logic_level).or_insert(0) += 1;
                },
                None => {
                    if enable_output {
                        println!("{}: No further logic could be applied.", "Solver Step".bold());
                    }

                    self.solution = SolverSolution::NoSolution(self.step_count, self.remaining_mines, self.hidden_count, self.state.clone());
                    return self.solution.clone();
                }
            }

            self.step_count += 1;
        }
    }

    fn is_solved(&self) -> bool {
        self.hidden_count == 0 || (self.flag_count + self.hidden_count) == self.field.get_mines()
    }

    fn print(&self) {
        for (x, y) in self.field.sorted_fields() {
            match self.get_state(x, y) {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.get_cell(x as u32, y as u32) {
                    MineSweeperCell::Empty => print!("  "),
                    MineSweeperCell::Mine => print!("{} ", "X".red()),
                    MineSweeperCell::Number(_n) => print!("{} ", self.field.get_cell(x as u32, y as u32).get_colored()),
                },
            }

            if x == self.field.get_width() - 1 {
                println!();
            }
        }
    }

    pub(super) fn get_state(&self, x: u32, y: u32) -> MineSweeperCellState {
        self.state[x as usize][y as usize].clone()
    }

    fn set_state(&mut self, x: u32, y: u32, state: MineSweeperCellState) {
        self.state[x as usize][y as usize] = state;
    }

    fn do_solving_step(&mut self) -> Option<u8>{
        match self.do_basic_neighbour_check(){
            Some(_) => {
                return Some(1);
            },
            None => {}
        }

        match self.apply_basic_box_logic() {
            Some(_) => {
                return Some(2);
            },
            None => {}
        }

        match self.apply_extended_box_logic() {
            Some(_) => {
                return Some(3);
            },
            None => {}
        }

        match self.apply_permutation_checks() {
            Some(_) => {
                return Some(4);
            },
            None => {}
        }
        None
    }

    fn flag_all_hidden_cells(&mut self) {
        for (x, y) in self.field.sorted_fields() {
            if self.get_state(x, y) == MineSweeperCellState::Hidden {
                self.flag_cell(x, y);
            }
        }
    }

    #[track_caller]
    pub(super) fn reveal_field(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) == MineSweeperCellState::Revealed {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Revealed);
        self.hidden_count -= 1;

        match self.field.get_cell(x as u32, y as u32) {
            MineSweeperCell::Mine => {
                panic!("Game Over! The Solver hit a mine at ({}, {})", x, y);
            }
            MineSweeperCell::Empty => {
                self.reveal_surrounding_cells(x, y);
            }
            MineSweeperCell::Number(i) => {
                if i == self.get_surrounding_flag_count(x, y) {
                    self.reveal_surrounding_cells(x, y);
                }
            }
        }
    }

    pub(super) fn flag_cell(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) == MineSweeperCellState::Revealed || self.get_state(x, y) == MineSweeperCellState::Flagged {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Flagged);
        self.flag_count += 1;
        self.hidden_count -= 1;
        self.remaining_mines -= 1;
    }

    #[track_caller]
    pub(super) fn reveal_surrounding_cells(&mut self, x: u32, y: u32) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                self.reveal_field(new_x, new_y);
            }
        }
    }

    pub(super) fn flag_surrounding_cells(&mut self, x: u32, y: u32) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                self.flag_cell(new_x, new_y);
            }
        }
    }

    pub(super) fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                return true;
            }
        }

        false
    }

    pub(super) fn has_revealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Revealed {
                return true;
            }
        }

        false
    }

    pub(super) fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Flagged {
                count += 1;
            }
        }

        count
    }

    pub(super) fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                count += 1;
            }
        }

        count
    }

    pub(super) fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub(super) fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = self.field.get_cell(x as u32, y as u32).get_number();

        if flag_count > number {
            panic!("Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}", x, y, flag_count, number);
        }

        number - flag_count
    }

    pub(super) fn has_informations(&self, x: u32, y: u32) -> bool {
        self.get_state(x, y) == MineSweeperCellState::Revealed
        && matches!(self.field.get_cell(x, y), MineSweeperCell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }

    fn do_basic_neighbour_check(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                let needed_mines = self.get_reduced_count(x, y);
                if needed_mines == self.get_surrounding_unrevealed_count(x, y) {
                    self.flag_surrounding_cells(x, y);
                    did_something = true;
                }
                if needed_mines == 0 {
                    self.reveal_surrounding_cells(x, y);
                    did_something = true;
                }
            }
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::field_generator::MineSweeperFieldCreation;
    use crate::ng_generator::TestField;

    #[test]
    fn test_solver_creation() {
        let field = TestField::new(5, 5, MineSweeperFieldCreation::FixedCount(3));
        let solver = MineSweeperSolver::new(field.clone());

        assert_eq!(solver.field.get_width(), 5);
        assert_eq!(solver.field.get_height(), 5);
        assert_eq!(solver.flag_count, 0);
        assert_eq!(solver.hidden_count, 25);
        assert_eq!(solver.remaining_mines, 3);
    }

    #[test]
    fn test_flag_cell() {
        let field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        let mut solver = MineSweeperSolver::new(field);

        let initial_flag_count = solver.flag_count;
        let initial_hidden_count = solver.hidden_count;
        let initial_remaining = solver.remaining_mines;

        solver.flag_cell(0, 0);

        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged);
        assert_eq!(solver.flag_count, initial_flag_count + 1);
        assert_eq!(solver.hidden_count, initial_hidden_count - 1);
        assert_eq!(solver.remaining_mines, initial_remaining - 1);
    }

    #[test]
    fn test_revealing_safe_cells() {
        let field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        solver.reveal_field(1, 1);

        assert_eq!(solver.get_state(1, 1), MineSweeperCellState::Revealed);
        assert!(solver.hidden_count == 0);
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_reveal_mine_panics() {
        let mut field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(1, 1)]);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(1, 1); // Should panic
    }

    #[test]
    fn test_get_surrounding_unrevealed_count() {
        let field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // All cells around (1,1) should be unrevealed initially
        assert_eq!(solver.get_surrounding_unrevealed_count(1, 1), 8);

        // Reveal some surrounding cells
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        solver.set_state(2, 2, MineSweeperCellState::Revealed);

        assert_eq!(solver.get_surrounding_unrevealed_count(1, 1), 6);
    }

    #[test]
    fn test_get_surrounding_flag_count() {
        let field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        let mut solver = MineSweeperSolver::new(field);

        // No flags initially
        assert_eq!(solver.get_surrounding_flag_count(1, 1), 0);

        // Flag some surrounding cells
        solver.set_state(0, 0, MineSweeperCellState::Flagged);
        solver.set_state(2, 2, MineSweeperCellState::Flagged);

        assert_eq!(solver.get_surrounding_flag_count(1, 1), 2);
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_at_start_field() {
        // Create a malformed field where the start field contains a mine
        let mut field = TestField::new(5, 5, MineSweeperFieldCreation::FixedCount(1));
        let start_field = (0, 0);
        field.set_start_field(start_field.0, start_field.1);
        field.initialize(vec![start_field]);

        let mut solver = MineSweeperSolver::new(field);
        solver.start(true);
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_during_reveal_surrounding() {
        // Create a field where revealing surrounding cells hits a mine
        let mut field = TestField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));

        field.initialize(vec![(0, 0)]);
        field.set_cell(1, 1, MineSweeperCell::Empty);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(1, 1); // Should trigger reveal_surrounding_cells and hit the mine
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_corner_field() {
        // Test hitting a mine in a corner position
        let mut field = TestField::new(4, 4, MineSweeperFieldCreation::FixedCount(1));

        field.initialize(vec![(0, 0)]);
        field.set_cell(1, 1, MineSweeperCell::Empty);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(3, 3); // Should panic
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_with_incorrect_numbers() {
        // Test a malformed field where numbers don't match mine placement
        let mut field = TestField::new(5, 5, MineSweeperFieldCreation::FixedCount(2));
        field.set_start_field(4, 4);

        field.initialize(vec![
            (0, 0), (1, 0), (0, 1), (3, 2)
        ]);

        // Cell 1 1 is a 4, make it a 1
        field.set_cell(1, 1, MineSweeperCell::Number(1));

        MineSweeperSolver::new(field).start(true);
    }
}