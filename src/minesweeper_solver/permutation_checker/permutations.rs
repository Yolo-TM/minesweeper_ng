use crate::minesweeper_solver::{MineSweeperCellState, MineSweeperSolver};
use crate::field_generator::MineSweeperField;
use super::{sort::sort_by_min_distance, super::search_for_islands, super::merge_islands, collect_bits, get_last_one_bit};
use std::{thread, collections::HashMap};
use num_cpus;

const MAXIMUM_PERMUTATIONS_IN_THREAD: usize = 18;

impl<M> MineSweeperSolver<M> where M: MineSweeperField {
    pub fn apply_permutation_checks(&mut self) -> Option<()> {
        let mut did_something = false;
        let max_mines: u32 = self.field.get_mines() - self.flag_count;
        let islands = search_for_islands(self.field.get_width(), self.field.get_height(), &self.field.get_field(), &self.state);
        
        if islands.len() == 0 {
            panic!("No islands found. This should not happen.");
        }
        
        if max_mines <= 0 {
            panic!("No remaining mines. This should not happen.");
        }
        
        // merge neighbouring small islands together
        let merged_islands = merge_islands(islands, 3, 16);
        
        for island in &merged_islands {
            self.try_permutation_solving(island, max_mines, &mut did_something);
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }

    fn try_permutation_solving(&mut self, island: &Vec<(u32, u32)>, max_mines: u32, did_something: &mut bool) {
        let mut all_possible_permutations: u32 = 0;
        let mut all_wrong_permutations: u32 = 0;
        let mut permutation_field: HashMap<(u32, u32), u32> = HashMap::new();
        let mut permutation_vector: Vec<((u32, u32), bool)> = vec![];
        let mut no_revealed_neighbours: u32 = 0;

        for &(x, y) in island {
            if self.has_revealed_neighbours(x, y) {
                permutation_vector.push(((x, y), false));
            } else {
                no_revealed_neighbours += 1;
            }
        }

        for &((x, y), _) in &permutation_vector {
            permutation_field.insert((x, y), 0);
        }

        sort_by_min_distance(&mut permutation_vector);

        if permutation_vector.len() >= 15 {
            // This would take way too long, start multiple threads for speed up
            self.start_permutation_threads(permutation_vector.clone(), max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
        } else {
            let mut permutations = permutation_vector.clone();
            self.recursively_apply_permutations(&mut permutations, 0, max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
        }

        if all_possible_permutations != 0 {
            for ((x, y), permutation_mines) in &permutation_field {
                if *permutation_mines == 0 {
                    // Field is in every possible way empty
                    self.reveal_field(*x, *y);
                    *did_something = true;
                }
                if *permutation_mines == all_possible_permutations {
                    // Field is in every possible way a mine
                    self.flag_cell(*x, *y);
                    *did_something = true;
                }
            }
        }

        if *did_something == false && max_mines > no_revealed_neighbours {
            let mut all_possible_permutations: u32 = 0;
            let mut all_wrong_permutations: u32 = 0;
            let mut permutation_field: HashMap<(u32, u32), u32> = HashMap::new();

            for &((x, y), _) in &permutation_vector {
                permutation_field.insert((x, y), 0);
            }

            // Edge Case, it could be solvable if all non information fields are mines and we give a reduced max mines to our permutation all_wrong_permutations
            let max_mines = max_mines - no_revealed_neighbours;
            if permutation_vector.len() >= 20 {
                // This would take way too long, start multiple threads for speed up
                self.start_permutation_threads(permutation_vector.clone(), max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
            } else {
                self.recursively_apply_permutations(&mut permutation_vector.clone(), 0, max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
            }

            if all_possible_permutations != 0 {
                for ((x, y), permutation_mines) in permutation_field {
                    if permutation_mines == 0 {
                        // Field is in every possible way empty
                        self.reveal_field(x, y);
                        *did_something = true;
                    }

                    if permutation_mines == all_possible_permutations {
                        // Field is in every possible way a mine
                        self.flag_cell(x, y);
                        *did_something = true;
                    }
                }
            }
        }
    }

    fn start_permutation_threads(
        &mut self,
        permutation_vector: Vec<((u32, u32), bool)>,
        max_remaining_mines: u32,
        permutation_field: &mut HashMap<(u32, u32), u32>,
        possible_permutations: &mut u32,
        wrong_permutations: &mut u32
        ) {
        // run on gpu ??
        let (thread_count, masks, start_index) = self.generate_start_masks(&permutation_vector);

        let mut thread_pool = vec![];
        for bit_mask in 0..thread_count {
            let mut permutation_vector_clone = permutation_vector.clone();
            let mut permutation_field_clone = permutation_field.clone();
            let new_self: MineSweeperSolver<M> = self.clone(); // Clone the current instance of self

            let mask = collect_bits(masks[bit_mask]);
            for i in 0..permutation_vector_clone.len() {
                if mask[i] == 1 {
                    permutation_vector_clone[i].1 = true;
                } else {
                    permutation_vector_clone[i].1 = false;
                }
            }

            let handle = thread::spawn(move || {
                let mut local_possible_permutations: u32 = 0;
                let mut local_wrong_permutations: u32 = 0;

                new_self.recursively_apply_permutations(&mut permutation_vector_clone, start_index, max_remaining_mines, &mut permutation_field_clone, &mut local_possible_permutations, &mut local_wrong_permutations);

                (permutation_field_clone, local_possible_permutations, local_wrong_permutations)
            });

            thread_pool.push(handle);
        }

        // Wait for all threads to finish
        for handle in thread_pool {
            match handle.join() {
                Ok((local_permutation_field, local_possible_permutations, local_wrong_permutations)) => {
                    for (key, value) in local_permutation_field {
                        *permutation_field.entry(key).or_insert(0) += value;
                    }
                    *possible_permutations += local_possible_permutations;
                    *wrong_permutations += local_wrong_permutations;
                }
                Err(e) => {
                    eprintln!("Thread panicked: {:?}", e);
                }
            }
        }
    }

    fn generate_start_masks(&self, permutation_vector: &Vec<((u32, u32), bool)>) -> (usize, Vec<u64>, usize) {
        // Atleast use as much as we can
        let min_threads = num_cpus::get() * 2;

        let mut numbers = vec![];
        let mut start_index: usize = 0;

        // we can calculate the masks for the threads in the main thread
        if permutation_vector.len() <= MAXIMUM_PERMUTATIONS_IN_THREAD {
            self.calculate_masks(0, 2^permutation_vector.len() as u64, &mut numbers, &mut start_index, permutation_vector, min_threads);
            return (numbers.len(), numbers, start_index);
        }

        // We have a lot of permutations for masks already, so calulate them also multithreaded
        start_index = permutation_vector.len() - MAXIMUM_PERMUTATIONS_IN_THREAD;

        // Maximum Number of possible bit patterns for masks
        let mut max_number = 1;
        for _ in 0..start_index {
            max_number *= 2;
        }

        // Thread Count for generating masks
        let mut thread_count = 1;
        for _ in (0..start_index).step_by(3) {
            thread_count *= 2;
        }

        let mut thread_pool = vec![];
        for i in 0..thread_count {
            let count_start = max_number * i / thread_count;
            let count_end = max_number * (i + 1) / thread_count - 1;

            let mut sindex = start_index.clone();
            let perm_vec = permutation_vector.clone();
            let new_self = self.clone();

            let handle = thread::spawn(move || {
                let mut valid_masks = vec![];
                new_self.calculate_masks(count_start, count_end, &mut valid_masks, &mut sindex, &perm_vec, 2);

                valid_masks
            });

            thread_pool.push(handle);
        }

        for handle in thread_pool {
            match handle.join() {
                Ok(valid_masks) => {
                    for mask in valid_masks {
                        numbers.push(mask);
                    }
                }
                Err(e) => {
                    eprintln!("Thread panicked: {:?}", e);
                }
            }
        }

        return(numbers.len(), numbers, start_index);
    }

    fn calculate_masks(&self, start_counter: u64, counter_maximum: u64, numbers: &mut Vec<u64>, start_index: &mut usize, permutation_vector: &Vec<((u32, u32), bool)>, min_threads: usize) {
        let mut counter = start_counter;
        while numbers.len() < min_threads || *start_index >= get_last_one_bit(counter) + 1 {
            let possible_new_start = get_last_one_bit(counter) + 1;
            if self.is_possible_start(counter, permutation_vector, possible_new_start) {
                numbers.push(counter);

                if *start_index < possible_new_start {
                    // we have a bigger start index, generate all possible masks for this startindex
                    // a bigger start index reduces the amount of permutations calculated in each thread
                    *start_index = possible_new_start;
                }
            }
            counter += 1;

            if counter == counter_maximum {
                break;
            }
        }
    }

    fn is_possible_start(&self, mask: u64, permutation_vector: &Vec<((u32, u32), bool)>, check_until: usize) -> bool {
        let bits = collect_bits(mask);
        let mut permutation_vector_clone = vec![];
        for i in 0..check_until {
            if bits[i] == 1 {
                permutation_vector_clone.push((permutation_vector[i].0, true));
            } else {
                permutation_vector_clone.push((permutation_vector[i].0, false));
            }
        }

        for i in 0..check_until {
            let (x, y) = permutation_vector_clone[i].0;

            for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
                if self.has_informations(new_x, new_y) {
                    if !self.can_number_be_satisfied(new_x, new_y, &permutation_vector_clone) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn can_number_be_satisfied(&self, x: u32, y: u32, permutation_vector: &Vec<((u32, u32), bool)>) -> bool {
        let mine_count = self.field.get_cell(x, y).get_number();
        let mut flag_count = 0;
        let mut unknown_count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Revealed {
                // revealed field, ignore
                continue;
            }

            if self.get_state(new_x, new_y) == MineSweeperCellState::Flagged {
                flag_count += 1;
            } else if let Some(field) = permutation_vector.iter().find(|&&((x, y), _)| x == new_x && y == new_y) {
                if field.1 {
                    flag_count += 1;
                }
            } else {
                // this field is not in the permutation vector, so it could be everything
                unknown_count += 1;
            }
        }

        if flag_count > mine_count {
            // Too many flags, this is not a valid permutation
            return false;
        }

        if (mine_count - flag_count) > unknown_count {
            // Not enough unknown fields to satisfy the number, this is not a valid permutation
            return false;
        }

        true
    }

    fn recursively_apply_permutations(
        &self,
        permutation_vector: &mut Vec<((u32, u32), bool)>,
        index: usize,
        max_remaining_mines: u32,
        permutation_field: &mut HashMap<(u32, u32), u32>,
        possible_permutations: &mut u32,
        wrong_permutations: &mut u32
    ) {
        // we have a permutation
        if index == permutation_vector.len() {
            self.insert_if_valid(&permutation_vector, permutation_field, possible_permutations, wrong_permutations);
            return;
        }

        // set all following fields to false as default state, this prevents collision with previous recursion calls
        for i in index..permutation_vector.len() {
            permutation_vector[i].1 = false;
        }

        // Check if we are allowed to place a mine here -> check surrounding numbers and if they are already satisfied
        let mut satisfied = false;
        for (new_x, new_y) in self.field.surrounding_fields(permutation_vector[index].0.0, permutation_vector[index].0.1, None) {
            if self.has_informations(new_x, new_y) && self.is_number_satisfied(new_x, new_y, permutation_vector) {
                // No more mines allowed here
                satisfied = true;
                break;
            }
        }

        // Check for case when here is no mine
        self.recursively_apply_permutations(permutation_vector, index + 1, max_remaining_mines, permutation_field, possible_permutations, wrong_permutations);

        // Is here even allowed to be a mine?
        if satisfied || max_remaining_mines == 0 {
            return;
        }

        permutation_vector[index].1 = true;
        self.recursively_apply_permutations(permutation_vector, index + 1, max_remaining_mines - 1, permutation_field, possible_permutations, wrong_permutations);
    }

    fn is_number_satisfied(&self, x: u32, y: u32, permutation_vector: &Vec<((u32, u32), bool)>) -> bool {
        let mut flag_count: u8 = 0;
        let mine_count = self.field.get_cell(x, y).get_number();

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y)  == MineSweeperCellState::Flagged {
                flag_count += 1;
            } else if let Some(field) = permutation_vector.iter().find(|&&((x, y), _)| x == new_x && y == new_y) {
                if field.1 {
                    flag_count += 1;
                }
            }
        }

        if mine_count == flag_count {
            // all mines would be flagged, this is a valid permutation
            return true;
        } else {
            return false;
        }
    }

    fn insert_if_valid(
        &self,
        permutation_vector: &Vec<((u32, u32), bool)>,
        permutation_field: &mut HashMap<(u32, u32), u32>,
        possible_permutations: &mut u32,
        wrong_permutations: &mut u32
        ) {
        for &((x, y), _mine) in permutation_vector {
            for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
                if self.has_informations(new_x, new_y) {
                    if !self.is_number_satisfied(new_x, new_y, permutation_vector) {
                        *wrong_permutations += 1;
                        return;
                    }
                }
            }
        }

        *possible_permutations += 1;
        for &((x, y), mine) in permutation_vector {
            if !mine {
                continue;
            }

            if let Some(count) = permutation_field.get_mut(&(x, y)) {
                *count += 1;
            } else {
                panic!("Field ({}, {}) not found in permutation field.", x, y);
            }
        }
    }
}