#![cfg_attr(debug_assertions, allow(unreachable_code))]

use super::Box;
use super::super::MineSweeperSolver;
use crate::field_generator::MineSweeperField;
use std::collections::HashMap;

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    pub fn apply_extended_box_logic(&mut self) -> Option<()> {

        let mut did_something = false;
        let mut boxes: Vec<Box> = vec![];
        
        // Create Boxes around fields with unrevealed neighbours
        for (x, y) in self.field.sorted_fields() {
            if !self.has_informations(x, y) {
                continue;
            }

            let surrounding_hidden_fields = self.get_surrounding_unrevealed(x, y);
            let mut new_box = Box::new(x, y, self.get_reduced_count(x, y));
            for cell in &surrounding_hidden_fields {
                new_box.add_field(cell.0, cell.1);
            }
            boxes.push(new_box);
        }

        // Create a Map of all fields with unrevealed neighbours and the boxes which are in their reach
        let mut field_map: HashMap<(u32, u32), Vec<Box>> = HashMap::new();
        for (x, y) in self.field.sorted_fields() {
            if !self.has_informations(x, y) {
                continue;
            }

            //
            todo!("Only add significant boxes to the field map")
            for box_ in &boxes {
                if box_.is_neighbouring(x, y) {
                    
                    field_map.entry((x, y)).or_insert(vec![]).push(box_.clone());
                }
            }
        }

        for ((x, y), boxes) in &field_map {
            let mut new_boxes = vec![];
            let mines = self.get_reduced_count(*x, *y);
            let fields = self.get_surrounding_unrevealed(*x, *y);
            print!("Box at ({}, {}) has ", x, y);
            for box_ in boxes {
                // Ignore boxes which dont help us (including the box we created for this field)
                // Boxes which hold the same mine count AND the same number of fields can be ignored (as some of the fields are shared)
                if mines == box_.get_mines() && fields.len() == box_.get_field_count() {
                    continue;
                }
                new_boxes.push(box_);
            }
            println!("New Boxes in Reach: {}", new_boxes.len());

            let mut field_tuples: Vec<(std::ops::RangeInclusive<u8>, Vec<(u32, u32)>)> = vec![];
            let mut safe_fields: Vec<(u32, u32)> = vec![];
            let mut mine_fields: Vec<(u32, u32)> = vec![];
            field_tuples.push((mines..=mines, fields.clone()));

            self.recursive_search(&fields, &mut field_tuples, &mut new_boxes, 0, &mut safe_fields, &mut mine_fields);
            for cell in &safe_fields {
                self.reveal_field(cell.0, cell.1);
                did_something = true;
            }
            for cell in &mine_fields {
                self.flag_cell(cell.0, cell.1);
                did_something = true;
            }
            println!("Field Tuples after search: {:?}\n\n", field_tuples);
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
        field_tuples: &mut Vec<(std::ops::RangeInclusive<u8>, Vec<(u32, u32)>)>,
        new_boxes: &mut Vec<&Box>,
        current_box_index: usize,
        safe_fields: &mut Vec<(u32, u32)>,
        mine_fields: &mut Vec<(u32, u32)>
    ) {
        if current_box_index == new_boxes.len() {
            return;
        }
        let box_ = new_boxes[current_box_index];
        {
            println!("Box: ID {} : {:?}", current_box_index, box_);
            println!("Len of field_tuples: {}", field_tuples.len());
            for i in 0..field_tuples.len() {
                let (shared, this_only, other_only) = box_.compare_to(&field_tuples[i].1);
                if shared.len() == 0 || (shared.len() as i8) < (this_only.len() as i8 - box_.get_mines() as i8) {
                    continue;
                }
                println!("Field Tuples bevor: {:?}", field_tuples);
                let box_mines = box_.get_mines();

                if this_only.len() == 0 && other_only.len() != 0 {
                    field_tuples.push((*field_tuples[i].0.start() - box_mines..=field_tuples[i].0.end() - box_mines, other_only));
                    field_tuples.push((box_mines..=box_mines, shared));
                    field_tuples.remove(i);
                } else if this_only.len() != 0 && other_only.len() != 0 {
                    let mut this_only_len = this_only.len();

                    if field_tuples.len() > 1 {
                        for cell in &this_only {
                            if original_fields.contains(cell) {
                                this_only_len += 1;
                            }
                        }
                        if this_only_len == 0 {
                            continue;
                        }
                    }

                    let lower_bound;
                    if this_only_len <= box_mines as usize {
                        lower_bound = box_mines - this_only_len as u8;
                    } else {
                        lower_bound = 0;
                    }

                    let mut upper_bound = shared.len() as u8;
                    if upper_bound > box_mines {
                        upper_bound = box_mines;
                    }
                    field_tuples.push((lower_bound..=upper_bound, shared));

                    let new_lower_bound;
                    if *field_tuples[i].0.start() < upper_bound as u8 {
                        new_lower_bound = 0;
                        println!("Bug a? New Lower Bound: {}", new_lower_bound);
                    } else {
                        new_lower_bound = *field_tuples[i].0.start() - upper_bound;
                        println!("New Lower Bound: {}", new_lower_bound);
                    }

                    let mut new_upper_bound;
                    if *field_tuples[i].0.end() < lower_bound as u8 {
                        new_upper_bound = 0;
                    } else {
                        new_upper_bound = field_tuples[i].0.end() - lower_bound;
                    }

                    if new_upper_bound > other_only.len() as u8 {
                        new_upper_bound = other_only.len() as u8;
                    }

                    field_tuples.push((new_lower_bound..=new_upper_bound, other_only));
                    field_tuples.remove(i);
                }
                println!("Field Tuples after: {:?}", field_tuples);
            }
        }
        self.recursive_search(original_fields, field_tuples, new_boxes, current_box_index + 1, safe_fields, mine_fields);
        {
            for i in 0..field_tuples.len() {
                let field_count = field_tuples[i].1.len() as u8;

                if field_count == 0 {
                    panic!("Field Count is 0. This should not happen.");
                }

                if field_tuples[i].0.start() == &0 {
                    continue;
                }

                if field_tuples[i].0.start() == field_tuples[i].0.end() {
                    continue;
                }

                if field_count < *field_tuples[i].0.end() {
                    let diff = *field_tuples[i].0.end() - field_count;
                    for j in 0..field_tuples.len() {
                        if i == j {
                            continue;
                        }
                        if field_tuples[j].0.start() == field_tuples[j].0.end() {
                            continue;
                        }

                        if !field_tuples[j].0.start() == 0 {
                            continue;
                        }
                        let start = field_tuples[j].0.start().clone() + diff;
                        field_tuples[j].0 = start..=*field_tuples[j].0.end();
                        break;
                    }
                    let start = field_tuples[i].0.start().clone();
                    field_tuples[i].0 = start..=field_count;
                    println!("Fixed : {:?}", field_tuples);
                }
            }

            for i in 0..field_tuples.len() {
                if field_tuples[i].0.start() != field_tuples[i].0.end() {
                    continue;
                }

                if field_tuples[i].0.start() == &0 {
                    for cell in &field_tuples[i].1 {
                        safe_fields.push(*cell);
                    }
                } else if *field_tuples[i].0.start() == field_tuples[i].1.len() as u8 {
                    for cell in &field_tuples[i].1 {
                        mine_fields.push(*cell);
                    }
                }
            }
        }
    }
}