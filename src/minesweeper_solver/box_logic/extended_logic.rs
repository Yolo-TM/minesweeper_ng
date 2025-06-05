use super::Box;
use super::super::MineSweeperSolver;
use crate::field_generator::MineSweeperField;
use std::collections::HashMap;
use std::vec;

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
            println!("\n\nBox ({}, {}) with range {:?} and fields: {:?}\n", original.get_owner().0, original.get_owner().1, original.get_mines(), original.get_fields());
            let mut field_tuples: Vec<(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)> = vec![(original.get_mines(), original.get_fields().clone())];
            let mut safe = vec![];
            let mut mines = vec![];

            self.recursive_search(original.get_fields(), &mut field_tuples, &adjacent, 0, &mut safe, &mut mines);
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
        field_tuples: &mut Vec<(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)>,
        boxes: &Vec<Box>,
        current_box_index: usize,
        safe_fields: &mut Vec<(u32, u32)>,
        mine_fields: &mut Vec<(u32, u32)>
    ) {
        if current_box_index == boxes.len() {
            return;
        }
        let box_ = boxes[current_box_index].clone();
        {
            println!("Entry Box ({}, {}) with range {:?} and fields: {:?}", box_.get_owner().0, box_.get_owner().1, box_.get_mines(), box_.get_fields());
            println!("Tuples: {:?}", field_tuples);            // Overlay the field tuples with our current information in the current box
            let mut new_tuples = Vec::new();
            let mut processed_indices = Vec::new();
            
            for i in 0..field_tuples.len() {
                let (range, fields) = &field_tuples[i];
                let (shared, this_only, other_only) = box_.compare_to(fields);
                let second_range = box_.get_mines().clone();

                if shared.len() == 0 || other_only.len() == 0 {
                    // No overlap, keep the tuple as is
                    continue;
                }

                let new_shared_range = second_range.start().saturating_sub(this_only.len())..=*second_range.end().min(&shared.len());
                let new_other_range = range.start().saturating_sub(*new_shared_range.end())..=range.end().saturating_sub(*new_shared_range.start()).min(other_only.len() as usize);

                if new_shared_range.end() > new_other_range.end() {
                    // If the new shared range is larger than the new other range, we cannot split
                    continue;
                }

                if shared.len() == 1 && new_shared_range.start() != new_shared_range.end() {
                    continue; // If the shared range is only one field, we cannot split it
                }

                if other_only.len() == 1 && new_other_range.start() != new_other_range.end() {
                    continue; // If the other range is only one field, we cannot split it
                }

                // Mark this tuple for removal since it will be split
                processed_indices.push(i);

                println!("New Shared Range: {:?}\nNew Other Range: {:?}", new_shared_range, new_other_range);

                new_tuples.push((
                    new_other_range,
                    other_only.clone()
                ));
                
                new_tuples.push((
                    new_shared_range,
                    shared.clone()
                ));
                
            }

            for &index in processed_indices.iter().rev() {
                field_tuples.remove(index);
            }
            field_tuples.append(&mut new_tuples);

            println!("Tuples: {:?}", field_tuples);

        }
        self.recursive_search(original_fields, field_tuples, boxes, current_box_index + 1, safe_fields, mine_fields);
        {
            println!("Past Box ({}, {}) with id: {}", box_.get_owner().0, box_.get_owner().1, current_box_index);
        }
    }
}