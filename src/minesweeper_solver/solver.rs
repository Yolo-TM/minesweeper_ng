use super::{MineSweeperCellState, MineSweeperSolver, SolverSolution};
use crate::*;
use colored::Colorize;
use std::collections::HashMap;

impl<M> MineSweeperSolver<M>
where
    M: MineSweeperField,
{
    pub fn new(field: M) -> Self {
        let state = vec![
            vec![MineSweeperCellState::Hidden; field.get_height() as usize];
            field.get_width() as usize
        ];

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
            println!(
                "{}: Starting solver with field size {}x{} and {} mines.",
                "Solver started".bold(),
                self.field.get_width(),
                self.field.get_height(),
                self.field.get_mines()
            );
            self.field.show();
            println!(
                "Revealing Start field: ({}, {})",
                self.field.get_start_field().0,
                self.field.get_start_field().1
            );
        }

        self.reveal_field(
            self.field.get_start_field().0,
            self.field.get_start_field().1,
        );

        return self.continue_solving(enable_output);
    }

    pub fn continue_solving(&mut self, enable_output: bool) -> SolverSolution {
        loop {
            if self.is_solved() {
                if enable_output {
                    println!(
                        "{}: Took {} steps.",
                        "Solver finished".bold(),
                        self.step_count
                    );
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
                        println!(
                            "{}: Applied logic level {}",
                            "Solver Step".bold(),
                            logic_level
                        );
                    }

                    *self.logic_levels.entry(logic_level).or_insert(0) += 1;
                }
                None => {
                    if enable_output {
                        println!(
                            "{}: No further logic could be applied.",
                            "Solver Step".bold()
                        );
                    }

                    self.solution = SolverSolution::NoSolution(
                        self.step_count,
                        self.remaining_mines,
                        self.hidden_count,
                        self.state.clone(),
                    );
                    return self.solution.clone();
                }
            }

            self.step_count += 1;
        }
    }

    fn is_solved(&self) -> bool {
        self.hidden_count == 0 || (self.flag_count + self.hidden_count) == self.field.get_mines()
    }

    pub(super) fn print(&self) {
        for (x, y) in self.field.sorted_fields() {
            match self.get_state(x, y) {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.get_cell(x as u32, y as u32) {
                    MineSweeperCell::Empty => print!("  "),
                    MineSweeperCell::Mine => print!("{} ", "X".red()),
                    MineSweeperCell::Number(_n) => {
                        print!("{} ", self.field.get_cell(x as u32, y as u32).get_colored())
                    }
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

    fn do_solving_step(&mut self) -> Option<u8> {
        match self.do_basic_neighbour_check() {
            Some(_) => {
                return Some(1);
            }
            None => {}
        }

        match self.apply_basic_box_logic() {
            Some(_) => {
                return Some(2);
            }
            None => {}
        }

        match self.apply_extended_box_logic() {
            Some(_) => {
                return Some(3);
            }
            None => {}
        }

        match self.apply_permutation_checks() {
            Some(_) => {
                return Some(4);
            }
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
        if self.get_state(x, y) == MineSweeperCellState::Revealed
            || self.get_state(x, y) == MineSweeperCellState::Flagged
        {
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
            panic!(
                "Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}",
                x, y, flag_count, number
            );
        }

        number - flag_count
    }

    pub(super) fn has_informations(&self, x: u32, y: u32) -> bool {
        self.get_state(x, y) == MineSweeperCellState::Revealed
            && matches!(self.field.get_cell(x, y), MineSweeperCell::Number(_))
            && self.has_unrevealed_neighbours(x, y)
    }

    pub(super) fn do_basic_neighbour_check(&mut self) -> Option<()> {
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

    #[test]
    fn test_solver_creation() {
        let field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(3));
        let solver = MineSweeperSolver::new(field.clone());

        assert_eq!(solver.field.get_width(), 5);
        assert_eq!(solver.field.get_height(), 5);
        assert_eq!(solver.flag_count, 0);
        assert_eq!(solver.hidden_count, 25);
        assert_eq!(solver.remaining_mines, 3);
    }

    #[test]
    fn test_flag_cell() {
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
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
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        solver.reveal_field(1, 1);

        assert_eq!(solver.get_state(1, 1), MineSweeperCellState::Revealed);
        assert!(solver.hidden_count == 0);
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_reveal_mine_panics() {
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(1, 1)]);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(1, 1); // Should panic
    }

    #[test]
    fn test_get_surrounding_unrevealed_count() {
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
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
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
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
        let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(1));
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
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));

        field.initialize(vec![(0, 0)]);
        field.set_cell(1, 1, MineSweeperCell::Empty);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(1, 1); // Should trigger reveal_surrounding_cells and hit the mine
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_corner_field() {
        // Test hitting a mine in a corner position
        let mut field = MineField::new(4, 4, MineSweeperFieldCreation::FixedCount(1));

        field.initialize(vec![(0, 0)]);
        field.set_cell(1, 1, MineSweeperCell::Empty);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(3, 3); // Should panic
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_solver_hits_mine_with_incorrect_numbers() {
        // Test a malformed field where numbers don't match mine placement
        let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(2));
        field.set_start_field(4, 4);

        field.initialize(vec![(0, 0), (1, 0), (0, 1), (3, 2)]);

        // Cell 1 1 is a 4, make it a 1
        field.set_cell(1, 1, MineSweeperCell::Number(1));

        MineSweeperSolver::new(field).start(true);
    }

    // ========== COMPREHENSIVE do_basic_neighbour_check TESTS ==========

    #[test]
    fn test_do_basic_neighbour_check_flag_all_remaining() {
        // Test scenario where all remaining unrevealed neighbors should be flagged
        let mut field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));

        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the number cell (1,1) - should be Number(1)
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Reveal safe neighbors to leave only the mine unrevealed
        solver.set_state(0, 1, MineSweeperCellState::Revealed);
        solver.set_state(1, 0, MineSweeperCellState::Revealed);

        // Now only (0,0) is unrevealed around (1,1), and needed_mines == unrevealed_count == 1
        let result = solver.do_basic_neighbour_check();

        // Should flag the remaining mine
        assert!(result.is_some());
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged);
    }

    #[test]
    fn test_do_basic_neighbour_check_reveal_all_safe() {
        // Test scenario where do_basic_neighbour_check reveals all safe cells around a number
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));

        // Place a mine at corner (0,0)
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the cell at (1,1) - it should be a Number(1)
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Flag the mine at (0,0)
        solver.flag_cell(0, 0);

        // Now needed_mines = 1 - 1 = 0, so all surrounding cells should be revealed
        let result = solver.do_basic_neighbour_check();

        assert!(result.is_some()); // Should have done something

        // Check that safe neighbors were revealed
        assert_eq!(solver.get_state(0, 1), MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(1, 0), MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(2, 0), MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(2, 1), MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(2, 2), MineSweeperCellState::Revealed);
        // The mine at (0,0) should still be flagged
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged);
    }

    #[test]
    fn test_do_basic_neighbour_check_no_action_needed() {
        // Test scenario where do_basic_neighbour_check finds nothing to do
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));

        field.initialize(vec![(0, 0), (2, 2)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the center cell (1,1) - it should be a Number(2)
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Don't flag any mines, so needed_mines = 2, but unrevealed_count = 8
        // Neither condition (needed_mines == unrevealed_count nor needed_mines == 0) is met
        let result = solver.do_basic_neighbour_check();

        assert!(result.is_none()); // Should have done nothing
    }

    #[test]
    fn test_do_basic_neighbour_check_multiple_cells() {
        // Test scenario with multiple revealed numbers that can trigger logic
        let mut field = MineField::new(4, 4, MineSweeperFieldCreation::FixedCount(4));

        // Create a pattern with mines at corners
        field.initialize(vec![(0, 0), (0, 3), (3, 0), (3, 3)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal numbers at (1,1) and (2,2)
        solver.set_state(1, 1, MineSweeperCellState::Revealed);
        solver.set_state(2, 2, MineSweeperCellState::Revealed);

        // Flag some mines
        solver.flag_cell(0, 0);
        solver.flag_cell(3, 3);

        let result = solver.do_basic_neighbour_check();

        assert!(result.is_some()); // Should have done something
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_do_basic_neighbour_check_reveals_mine_malformed_field() {
        // Create a malformed field where the logic will reveal a mine
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));

        // Place mines at (0,1) and (2,2)
        field.initialize(vec![(0, 1), (2, 2)]);

        // Manually set wrong number at (1,1) - should be 2 but we'll make it 1
        field.set_cell(1, 1, MineSweeperCell::Number(1));

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the malformed cell
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Flag one mine correctly to make needed_mines = 0
        solver.flag_cell(2, 2);

        // This should trigger reveal_surrounding_cells, hitting the mine at (0,1)
        solver.do_basic_neighbour_check();
    }

    #[test]
    #[should_panic(expected = "Flag count is greater than number")]
    fn test_do_basic_neighbour_check_too_many_flags() {
        // Test scenario that triggers the panic in get_reduced_count
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(8));

        // Place many mines to avoid remaining_mines underflow
        field.initialize(vec![
            (0, 0),
            (2, 2),
            (0, 2),
            (2, 0),
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
        ]);

        // Manually set a lower number to create the flag_count > number condition
        // This simulates a malformed field scenario
        field.set_cell(1, 1, MineSweeperCell::Number(6)); // Will flag 7 but number is 6

        let mut solver = MineSweeperSolver::new(field);

        // Reveal a number cell (1,1) - should be Number(6)
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Flag 7 neighbors around (1,1) - more than the number 6
        solver.flag_cell(0, 0);
        solver.flag_cell(2, 2);
        solver.flag_cell(0, 2);
        solver.flag_cell(2, 0);
        solver.flag_cell(0, 1);
        solver.flag_cell(1, 0);
        solver.flag_cell(1, 2);
        // Leave (2, 1) unrevealed so has_informations returns true

        // This should trigger the panic in get_reduced_count because flag_count (7) > number (6)
        solver.do_basic_neighbour_check();
    }

    #[test]
    fn test_do_basic_neighbour_check_already_solved_area() {
        // Test area where all neighbors are already revealed or flagged
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));

        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the number cell
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        // Manually reveal and flag all neighbors
        solver.flag_cell(0, 0); // Mine
        solver.set_state(0, 1, MineSweeperCellState::Revealed);
        solver.set_state(1, 0, MineSweeperCellState::Revealed);
        solver.set_state(2, 0, MineSweeperCellState::Revealed);
        solver.set_state(2, 1, MineSweeperCellState::Revealed);
        solver.set_state(2, 2, MineSweeperCellState::Revealed);
        solver.set_state(1, 2, MineSweeperCellState::Revealed);
        solver.set_state(0, 2, MineSweeperCellState::Revealed);

        // Now (1,1) has no unrevealed neighbors, so has_informations should return false
        let result = solver.do_basic_neighbour_check();

        assert!(result.is_none()); // Should do nothing since no cells have unrevealed neighbors
    }

    #[test]
    #[should_panic(expected = "Game Over! The Solver hit a mine")]
    fn test_do_basic_neighbour_check_chain_reaction_hits_mine() {
        // Test where basic neighbor check reveals a mine due to malformed field
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));

        // Place mine at (2, 2)
        field.initialize(vec![(2, 2)]);

        // Manually set the center cell to be Empty to create malformed scenario
        field.set_cell(1, 1, MineSweeperCell::Empty);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal the center cell - since it's Empty, it should trigger reveal_surrounding_cells
        // which will hit the mine at (2,2)
        solver.reveal_field(1, 1);
    }

    // ========== CORE LOGIC METHOD TESTS ==========
    #[test]
    fn test_continue_solving_found_solution() {
        // Test continue_solving when a solution is found
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Create a solvable scenario by revealing safe cells first
        solver.reveal_field(2, 2); // This should reveal multiple cells and make progress toward solution

        let result = solver.continue_solving(false);
        assert!(matches!(result, SolverSolution::FoundSolution(_, _)));
        if let SolverSolution::FoundSolution(steps, _) = result {
            // Steps is u32, so always >= 0, just verify it's a valid number
            let _ = steps; // Use the variable to prevent warning
        }
    }

    #[test]
    fn test_continue_solving_no_solution() {
        // Test continue_solving when no solution can be found
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(4));
        field.initialize(vec![(0, 0), (1, 1), (2, 2), (0, 2)]);

        let mut solver = MineSweeperSolver::new(field);

        // Create a scenario where the solver gets stuck - reveal only one safe cell
        solver.reveal_field(1, 0); // Safe cell that won't lead to full solution

        let result = solver.continue_solving(false);

        assert!(matches!(result, SolverSolution::NoSolution(_, _, _, _)));
    }

    #[test]
    fn test_continue_solving_with_output() {
        // Test continue_solving with output enabled (checking it doesn't panic)
        let mut field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(1, 1); // This should create a solvable scenario

        let result = solver.continue_solving(true);

        // Either solution found or no solution, but should not panic
        assert!(
            matches!(result, SolverSolution::FoundSolution(_, _))
                || matches!(result, SolverSolution::NoSolution(_, _, _, _))
        );
    }

    #[test]
    fn test_is_solved_all_revealed() {
        // Test is_solved when all non-mine cells are revealed
        let mut field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Initially not solved
        assert!(!solver.is_solved());

        // Reveal all safe cells
        solver.reveal_field(0, 1);
        solver.reveal_field(1, 0);
        solver.reveal_field(1, 1);

        // Should be solved (hidden_count == 1, which is just the mine)
        assert!(solver.is_solved());
    }

    #[test]
    fn test_is_solved_all_flagged() {
        // Test is_solved when all mines are flagged and remaining cells revealed
        let mut field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Flag the mine and reveal safe cells
        solver.flag_cell(0, 0);
        solver.reveal_field(0, 1);
        solver.reveal_field(1, 0);
        solver.reveal_field(1, 1);

        // Should be solved (flag_count + hidden_count == mines)
        assert!(solver.is_solved());
    }

    #[test]
    fn test_is_solved_partial_progress() {
        // Test is_solved with partial progress
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        field.initialize(vec![(0, 0), (2, 2)]);

        let mut solver = MineSweeperSolver::new(field);

        // Reveal some cells but not enough
        solver.reveal_field(1, 1);
        solver.reveal_field(0, 1);

        assert!(!solver.is_solved());
    }

    #[test]
    fn test_do_solving_step_basic_logic() {
        // Test do_solving_step when basic neighbor check succeeds
        let mut field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        field.initialize(vec![(0, 0)]);

        let mut solver = MineSweeperSolver::new(field);

        // Set up a scenario where basic logic can work
        solver.set_state(1, 1, MineSweeperCellState::Revealed);
        solver.set_state(0, 1, MineSweeperCellState::Revealed);
        solver.set_state(1, 0, MineSweeperCellState::Revealed);

        let result = solver.do_solving_step();

        assert_eq!(result, Some(1)); // Basic neighbor check level
    }

    #[test]
    fn test_do_solving_step_no_logic_applicable() {
        // Test do_solving_step when no logic can be applied
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(3));
        field.initialize(vec![(0, 0), (1, 1), (2, 2)]);

        let mut solver = MineSweeperSolver::new(field);

        // Create a scenario where no logic can help
        solver.reveal_field(0, 1); // Just reveal one safe cell

        let result = solver.do_solving_step();

        assert_eq!(result, None);
    }

    #[test]
    fn test_flag_all_hidden_cells() {
        // Test flag_all_hidden_cells functionality
        let field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(3));
        let mut solver = MineSweeperSolver::new(field);

        // Reveal one cell to reduce hidden count
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        let initial_flag_count = solver.flag_count;

        solver.flag_all_hidden_cells();

        // Should have flagged all remaining hidden cells (3 cells)
        assert_eq!(solver.flag_count, initial_flag_count + 3);
        assert_eq!(solver.get_state(0, 1), MineSweeperCellState::Flagged);
        assert_eq!(solver.get_state(1, 0), MineSweeperCellState::Flagged);
        assert_eq!(solver.get_state(1, 1), MineSweeperCellState::Flagged);
    }

    // ========== HELPER METHOD TESTS ==========

    #[test]
    fn test_flag_surrounding_cells() {
        // Test flag_surrounding_cells functionality
        let field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(12));
        let mut solver = MineSweeperSolver::new(field);

        // Flag some cells already to test it doesn't double-flag
        solver.flag_cell(0, 0);
        solver.set_state(2, 2, MineSweeperCellState::Revealed);
        solver.hidden_count -= 1; // Manually adjust since we used set_state directly
        let initial_flag_count = solver.flag_count;
        let initial_remaining_mines = solver.remaining_mines;

        solver.flag_surrounding_cells(1, 1);

        // Should have flagged all hidden neighbors (8 total - 1 already flagged - 1 already revealed = 6)
        assert_eq!(solver.flag_count, initial_flag_count + 6);

        // Check specific cells
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged); // Was already flagged
        assert_eq!(solver.get_state(0, 1), MineSweeperCellState::Flagged); // Newly flagged
        assert_eq!(solver.get_state(1, 0), MineSweeperCellState::Flagged); // Newly flagged
        assert_eq!(solver.get_state(2, 2), MineSweeperCellState::Revealed); // Was already revealed

        // remaining_mines should have decremented but not below 0
        assert!(solver.remaining_mines <= initial_remaining_mines);
    }

    #[test]
    fn test_has_unrevealed_neighbours_true() {
        // Test has_unrevealed_neighbours when there are unrevealed neighbors
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // All neighbors are initially hidden
        assert!(solver.has_unrevealed_neighbours(1, 1));

        // Reveal some but not all neighbors
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        solver.set_state(0, 1, MineSweeperCellState::Revealed);

        assert!(solver.has_unrevealed_neighbours(1, 1));
    }

    #[test]
    fn test_has_unrevealed_neighbours_false() {
        // Test has_unrevealed_neighbours when all neighbors are revealed/flagged
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Reveal or flag all neighbors of (1,1)
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        solver.set_state(0, 1, MineSweeperCellState::Revealed);
        solver.set_state(0, 2, MineSweeperCellState::Revealed);
        solver.set_state(1, 0, MineSweeperCellState::Flagged);
        solver.set_state(1, 2, MineSweeperCellState::Flagged);
        solver.set_state(2, 0, MineSweeperCellState::Revealed);
        solver.set_state(2, 1, MineSweeperCellState::Revealed);
        solver.set_state(2, 2, MineSweeperCellState::Revealed);

        assert!(!solver.has_unrevealed_neighbours(1, 1));
    }

    #[test]
    fn test_has_unrevealed_neighbours_corner_cell() {
        // Test has_unrevealed_neighbours for a corner cell
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Corner cell (0,0) has only 3 neighbors
        assert!(solver.has_unrevealed_neighbours(0, 0));

        // Reveal all 3 neighbors
        solver.set_state(0, 1, MineSweeperCellState::Revealed);
        solver.set_state(1, 0, MineSweeperCellState::Revealed);
        solver.set_state(1, 1, MineSweeperCellState::Revealed);

        assert!(!solver.has_unrevealed_neighbours(0, 0));
    }

    #[test]
    fn test_has_revealed_neighbours_true() {
        // Test has_revealed_neighbours when there are revealed neighbors
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Initially no revealed neighbors
        assert!(!solver.has_revealed_neighbours(1, 1));

        // Reveal one neighbor
        solver.set_state(0, 0, MineSweeperCellState::Revealed);

        assert!(solver.has_revealed_neighbours(1, 1));
    }

    #[test]
    fn test_has_revealed_neighbours_false() {
        // Test has_revealed_neighbours when no neighbors are revealed
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Flag some neighbors but don't reveal any
        solver.set_state(0, 0, MineSweeperCellState::Flagged);
        solver.set_state(0, 1, MineSweeperCellState::Flagged);

        assert!(!solver.has_revealed_neighbours(1, 1));
    }

    #[test]
    fn test_get_surrounding_unrevealed() {
        // Test get_surrounding_unrevealed returns correct coordinates
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Initially all 8 neighbors of (1,1) should be unrevealed
        let unrevealed = solver.get_surrounding_unrevealed(1, 1);
        assert_eq!(unrevealed.len(), 8);

        // Check it contains all expected coordinates
        let expected: Vec<(u32, u32)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];

        for coord in expected {
            assert!(unrevealed.contains(&coord));
        }

        // Reveal some cells and check again
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        solver.set_state(2, 2, MineSweeperCellState::Flagged);

        let unrevealed_after = solver.get_surrounding_unrevealed(1, 1);
        assert_eq!(unrevealed_after.len(), 6);
        assert!(!unrevealed_after.contains(&(0, 0))); // Was revealed
        assert!(!unrevealed_after.contains(&(2, 2))); // Was flagged
    }

    #[test]
    fn test_get_surrounding_unrevealed_corner() {
        // Test get_surrounding_unrevealed for corner cell
        let field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));
        let solver = MineSweeperSolver::new(field);

        // Corner cell (0,0) should have 3 unrevealed neighbors
        let unrevealed = solver.get_surrounding_unrevealed(0, 0);
        assert_eq!(unrevealed.len(), 3);

        let expected: Vec<(u32, u32)> = vec![(0, 1), (1, 0), (1, 1)];

        for coord in expected {
            assert!(unrevealed.contains(&coord));
        }
    }

    #[test]
    fn test_set_state() {
        // Test set_state functionality directly
        let field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Initially all cells are hidden
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Hidden);

        // Set to revealed
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Revealed);

        // Set to flagged
        solver.set_state(0, 0, MineSweeperCellState::Flagged);
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged);

        // Set back to hidden
        solver.set_state(0, 0, MineSweeperCellState::Hidden);
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Hidden);
    }

    #[test]
    fn test_flag_cell_already_flagged() {
        // Test flag_cell when cell is already flagged (should do nothing)
        let field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Flag the cell once
        solver.flag_cell(0, 0);
        let flag_count_after_first = solver.flag_count;
        let hidden_count_after_first = solver.hidden_count;
        let remaining_mines_after_first = solver.remaining_mines;

        // Try to flag again
        solver.flag_cell(0, 0);

        // Counts should not change
        assert_eq!(solver.flag_count, flag_count_after_first);
        assert_eq!(solver.hidden_count, hidden_count_after_first);
        assert_eq!(solver.remaining_mines, remaining_mines_after_first);
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Flagged);
    }

    #[test]
    fn test_flag_cell_already_revealed() {
        // Test flag_cell when cell is already revealed (should do nothing)
        let field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Reveal the cell first
        solver.set_state(0, 0, MineSweeperCellState::Revealed);
        solver.hidden_count -= 1; // Manually adjust since we used set_state directly

        let flag_count_before = solver.flag_count;
        let hidden_count_before = solver.hidden_count;
        let remaining_mines_before = solver.remaining_mines;

        // Try to flag the revealed cell
        solver.flag_cell(0, 0);

        // Counts should not change
        assert_eq!(solver.flag_count, flag_count_before);
        assert_eq!(solver.hidden_count, hidden_count_before);
        assert_eq!(solver.remaining_mines, remaining_mines_before);
        assert_eq!(solver.get_state(0, 0), MineSweeperCellState::Revealed);
    }

    #[test]
    fn test_reveal_field_already_revealed() {
        // Test reveal_field when cell is already revealed (should do nothing)
        let field = MineField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        let mut solver = MineSweeperSolver::new(field);

        // Reveal the cell once
        solver.reveal_field(1, 1);
        let hidden_count_after_first = solver.hidden_count;

        // Try to reveal again
        solver.reveal_field(1, 1);

        // Hidden count should not change further
        assert_eq!(solver.hidden_count, hidden_count_after_first);
        assert_eq!(solver.get_state(1, 1), MineSweeperCellState::Revealed);
    }

    #[test]
    fn test_reveal_field_number_with_correct_flags() {
        // Test reveal_field on a number cell when correct number of flags are around it
        let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        field.initialize(vec![(0, 0), (2, 2)]);

        let mut solver = MineSweeperSolver::new(field);

        // Flag the mines correctly
        solver.flag_cell(0, 0);
        solver.flag_cell(2, 2);

        // Reveal the number cell - it should trigger reveal_surrounding_cells
        let hidden_count_before = solver.hidden_count;
        solver.reveal_field(1, 1);

        // Should have revealed the center cell plus triggered surrounding reveals
        assert!(solver.hidden_count < hidden_count_before);
        assert_eq!(solver.get_state(1, 1), MineSweeperCellState::Revealed);
    }
}
