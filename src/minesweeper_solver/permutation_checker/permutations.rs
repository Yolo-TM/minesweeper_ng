use super::sort::sort_by_min_distance;
use crate::minesweeper_solver::MineSweeperCellState;
use crate::minesweeper_solver::MineSweeperSolver;
use super::{search_for_islands, merge_islands};
use colored::Colorize;
use num_cpus;
use std::{thread, collections::HashMap};

impl MineSweeperSolver {
    pub fn apply_permutation_checks(&mut self) -> Option<()> {
        let mut did_something = false;
        let max_mines: u64 = self.field.mines - self.flag_count;
        let islands = search_for_islands(self);
        
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

    fn try_permutation_solving(&mut self, island: &Vec<(usize, usize)>, max_mines: u64, did_something: &mut bool) {
        let mut all_possible_permutations: u64 = 0;
        let mut all_wrong_permutations: u64 = 0;
        let mut permutation_field: HashMap<(usize, usize), u64> = HashMap::new();
        let mut permutation_vector: Vec<((usize, usize), bool)> = vec![];
        let mut no_revealed_neighbours: u64 = 0;

        for &(x, y) in island {
            if self.has_revealed_neighbours(x, y) {
                permutation_field.insert((x, y), 0);
            } else {
                no_revealed_neighbours += 1;
            }
        }

        for (&(x, y), _) in &permutation_field {
            permutation_vector.push(((x, y), false));
        }

        // useless !? -> to be tested 
        sort_by_min_distance(&mut permutation_vector);
        println!("\n\nRunning for island with {} cells and {} testable fields with max_mines {}", island.len().to_string().green(), permutation_vector.len().to_string().green(), max_mines.to_string().red());

        if permutation_vector.len() >= 20 {
            // This would take way too long, start multiple threads for speed up
            self.start_permutation_threads(permutation_vector.clone(), max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
        } else {
            let mut permutations = permutation_vector.clone();
            self.recursively_apply_permutations(&mut permutations, 0, max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
        }

        println!("Possible Permutations: {}", all_possible_permutations.to_string().green());
        println!("Wrong Permutations: {}", all_wrong_permutations.to_string().red());

        if all_possible_permutations == 0 {
           return; // No possible permutations found, skip this island
        }

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

        if !*did_something && max_mines > no_revealed_neighbours {
            let mut all_possible_permutations: u64 = 0;
            let mut all_wrong_permutations: u64 = 0;
            let mut permutation_field: HashMap<(usize, usize), u64> = HashMap::new();

            // Edge Case, it could be solvable if all non information fields are mines and we give a reduced max mines to our permutation all_wrong_permutations
            let max_mines = max_mines - no_revealed_neighbours;
            if permutation_vector.len() >= 20 {
                // This would take way too long, start multiple threads for speed up
                self.start_permutation_threads(permutation_vector.clone(), max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
            } else {
                self.recursively_apply_permutations(&mut permutation_vector.clone(), 0, max_mines, &mut permutation_field, &mut all_possible_permutations, &mut all_wrong_permutations);
            }

            println!("Edge Case Possible Permutations: {}", all_possible_permutations.to_string().green());
            println!("Edge Case Wrong Permutations: {}", all_wrong_permutations.to_string().red());

            if all_possible_permutations == 0 {
               return; // No possible permutations found, skip this island
            }

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

    fn start_permutation_threads(
        &mut self,
        permutation_vector: Vec<((usize, usize), bool)>,
        max_remaining_mines: u64,
        permutation_field: &mut HashMap<(usize, usize), u64>,
        possible_permutations: &mut u64,
        wrong_permutations: &mut u64
        ) {
        // run on gpu ??
        let cores = num_cpus::get();
        let threads = (cores * 2) as u64;
        let mut thread_pool = vec![];
        let mut start_index = 0;

        // Calculate the start index for each thread, the start index is the index of the last one (+1) in the bitwise number threads
        // This is done to ensure that each thread has a unique set of permutations to work on
        let mask = collect_bits(threads - 1);
        for i in 0..mask.len() {
            if mask[i] == 1 {
                start_index = i;
            }
        }
        start_index += 1; // Start at the next index

        println!("Starting {} threads for 2^{} permutations", threads.to_string().green(), permutation_vector.len().to_string().green());

        for bit_mask in 0..threads {
            let mut permutation_vector_clone = permutation_vector.clone();
            let mut permutation_field_clone = permutation_field.clone();
            let new_self: MineSweeperSolver = self.clone(); // Clone the current instance of self

            let mask = collect_bits(bit_mask);
            // implement a check if this mask is already a valid permutation and if not skip it
            for i in 0..permutation_vector_clone.len() {
                if mask[i] == 1 {
                    permutation_vector_clone[i].1 = true;
                } else {
                    permutation_vector_clone[i].1 = false;
                }
            }

            // Create a thread for each core
            let handle = thread::spawn(move || {
                let mut local_possible_permutations: u64 = 0;
                let mut local_wrong_permutations: u64 = 0;

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

    fn recursively_apply_permutations(
        &self,
        permutation_vector: &mut Vec<((usize, usize), bool)>,
        index: usize,
        max_remaining_mines: u64,
        permutation_field: &mut HashMap<(usize, usize), u64>,
        possible_permutations: &mut u64,
        wrong_permutations: &mut u64
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
        for (new_x, new_y) in self.field.surrounding_fields(permutation_vector[index].0.0, permutation_vector[index].0.1) {
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

    fn is_number_satisfied(&self, x: usize, y: usize, permutation_vector: &Vec<((usize, usize), bool)>) -> bool {
        let mut flag_count: u8 = 0;
        let mine_count = self.field.board[x][y].get_number();

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Flagged {
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
        permutation_vector: &Vec<((usize, usize), bool)>,
        permutation_field: &mut HashMap<(usize, usize), u64>,
        possible_permutations: &mut u64,
        wrong_permutations: &mut u64
        ) {
        for &((x, y), _mine) in permutation_vector {
            for (new_x, new_y) in self.field.surrounding_fields(x, y) {
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

fn collect_bits(number: u64) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..64 {
        let bit = ((number >> i) & 1) as u8;
        bits.push(bit);
    }
    bits
}
