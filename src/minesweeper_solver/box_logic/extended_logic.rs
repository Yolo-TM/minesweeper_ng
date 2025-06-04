use super::Box;
use super::super::MineSweeperSolver;
use crate::field_generator::MineSweeperField;
use std::collections::HashMap;

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    #[allow(unreachable_code)]
    pub(in crate::minesweeper_solver) fn apply_extended_box_logic(&mut self) -> Option<()> {
        let mut did_something = false;
        let mut boxes: Vec<Box> = vec![];

        // Create Boxes around fields with unrevealed neighbours
        for (x, y) in self.field.sorted_fields() {
            if !self.has_informations(x, y) {
                continue;
            }

            let count = self.get_reduced_count(x, y) as usize;
            let surrounding_hidden_fields = self.get_surrounding_unrevealed(x, y);
            let mut new_box = Box::new(x, y, count..=count);
            for cell in &surrounding_hidden_fields {
                new_box.add_field(cell.0, cell.1);
            }
            boxes.push(new_box);
        }

        // Map Boxes to other Boxes which could be important
        let mut possible_cases: Vec<(Box, Vec<Box>)> = vec![];
        for i in 0..boxes.len() {
            // Search through all boxes to find possible important adjacent boxes
            let box_ = &boxes[i];
            let mut possible_fields: Vec<Box> = vec![];

            for j in 0..boxes.len() {
                if i == j {
                    continue;
                }
                let other_box = &boxes[j];
                let (shared, _this, other) = box_.compare_to(&other_box.get_fields());

                // If this box could hold important informations
                if shared.len() > 0 && shared.len() >= other.len() && !(box_.covers_same_fields(other_box) && box_.has_same_range(other_box)) {

                    // check if we dont already have it
                    let mut allow_insert = true;
                    for fieldn in &possible_fields {
                        if fieldn.covers_same_fields(other_box) && fieldn.has_same_range(other_box) {
                            allow_insert = false;
                            break;
                        }
                    }

                    if allow_insert {
                        possible_fields.push(other_box.clone());
                    }
                }
            }

            if !possible_fields.is_empty() {
                let mut allow_insert = true;

                // go through all possible fields and check that there is atleast one range which is different to box_
                let range = box_.get_mines();
                if possible_fields.iter().all(|field| field.get_mines() == range) {
                    allow_insert = false;
                }

                if allow_insert {
                    possible_cases.push((box_.clone(), possible_fields));
                }
            }
        }

        for (original, adjacent) in possible_cases {
            println!("\nBox: {:?}\n\tAdjacent: {:?}", original, adjacent);

            // by collection all fields now, the permutation checker could be used to solve this small part and it should give valid results in every case
        }


        if did_something {
            return Some(());
        } else {
            return None;
        }
    }

    fn recursive_search(
        &mut self,
        original_fields: &Vec<(u32, u32)>,
        boxes: &mut Vec<&Box>,
        current_box_index: usize,
        safe_fields: &mut Vec<(u32, u32)>,
        mine_fields: &mut Vec<(u32, u32)>
    ) {
        if current_box_index == boxes.len() {
            return;
        }
        let box_ = boxes[current_box_index];
        {

        }
        self.recursive_search(original_fields, boxes, current_box_index + 1, safe_fields, mine_fields);
        {

        }
    }
}