use super::super::MineSweeperSolver;
use crate::field_generator::MineSweeperField;

impl<M> MineSweeperSolver<M>
where
    M: MineSweeperField + Clone,
{
    pub fn apply_basic_box_logic(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                // TODO: Is 5 a good number here or are we doint unnecessary calculations?
                for (new_x, new_y) in self.field.surrounding_fields(x, y, Some(5)) {
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