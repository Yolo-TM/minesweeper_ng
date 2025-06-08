use crate::*;

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    pub(in crate::minesweeper_solver) fn apply_basic_box_logic(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                for (new_x, new_y) in self.field.surrounding_fields(x, y, Some(3)) {
                    if self.has_informations(new_x, new_y) {
                        let reduced_count = self.get_reduced_count(x, y);
                        let reduced_count2 = self.get_reduced_count(new_x, new_y);
                        let surrounding_hidden_fields = self.get_surrounding_unrevealed(x, y);
                        let surrounding_hidden_fields2 = self.get_surrounding_unrevealed(new_x, new_y);

                        let mut shared_fields = vec![];
                        let mut not_shared_fields = vec![];
                        for hidden_cell in &surrounding_hidden_fields2 {
                            if surrounding_hidden_fields.contains(hidden_cell) {
                                shared_fields.push(*hidden_cell);
                            } else {
                                not_shared_fields.push(*hidden_cell);
                            }
                        }

                        if surrounding_hidden_fields.len() == shared_fields.len() {
                            // Found two numbers which share the same unrevealed fields.
                            // Now we can check if we can solve other neighbouring fields of new_x and new_y with this extra informations
                            let reduced_diff = reduced_count2 - reduced_count;

                            if reduced_count == reduced_count2{
                                for cell in &not_shared_fields {
                                    self.reveal_field(cell.0, cell.1);
                                    did_something = true;
                                }
                            } else if reduced_diff == (self.get_surrounding_unrevealed_count(new_x, new_y) - shared_fields.len() as u8) {
                                for cell in &not_shared_fields {
                                    self.flag_cell(cell.0, cell.1);
                                    did_something = true;
                                }
                            }
                        } else if reduced_count > reduced_count2 {
                            let mut rest_fields = vec![];
                            for cell in &surrounding_hidden_fields {
                                if !shared_fields.contains(cell) {
                                    rest_fields.push(*cell);
                                }
                            }
                            let reduced_diff = (reduced_count - reduced_count2) as usize;
                            if reduced_diff == rest_fields.len() {
                                for cell in &rest_fields {
                                    self.flag_cell(cell.0, cell.1);
                                    did_something = true;
                                }
                            }
                        }
                    }
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
    use crate::*;
    use crate::MineSweeperFieldCreation::FixedCount;

    #[test]
    fn test_one_one_pattern() {
        let mut field = MineField::new(5, 5, FixedCount(4));
        /*
        0001M
        00011
        00011
        1112M
        1M12M
        */
        field.initialize(vec![(4, 0), (4, 3), (1, 4), (4, 4)]);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(0, 0);

        assert_eq!(solver.do_basic_neighbour_check(), None);
        
        assert_eq!(solver.apply_basic_box_logic(), Some(()));

        assert_eq!(solver.get_state(4, 2), MineSweeperCellState::Revealed);
        assert_eq!(solver.get_state(2, 4), MineSweeperCellState::Revealed);
    }

    #[test]
    fn test_one_two_one_pattern() {
        let mut field = MineField::new(3, 3, FixedCount(2));
        /*
        01M
        022
        01M
        */
        field.initialize(vec![(2, 0), (2, 2)]);

        let mut solver = MineSweeperSolver::new(field);
        solver.reveal_field(0, 0);

        assert_eq!(solver.do_basic_neighbour_check(), None);
        assert_ne!(solver.get_state(2, 1), MineSweeperCellState::Revealed);

        assert_eq!(solver.apply_basic_box_logic(), Some(()));

        assert_eq!(solver.get_state(2, 0), MineSweeperCellState::Flagged);
        assert_eq!(solver.get_state(2, 2), MineSweeperCellState::Flagged);

        assert_eq!(solver.do_basic_neighbour_check(), Some(()));
        assert_eq!(solver.get_state(2, 1), MineSweeperCellState::Revealed);
    }
}