use crate::field_generator::minesweeper_field::MineSweeperField;
use crate::field_generator::minesweeper_cell::MineSweeperCell;
use super::sort::sort_by_min_distance;
use crate::minesweeper_solver::box_logic::boxes::Box;
use crate::minesweeper_solver::MineSweeperCellState;
use crate::minesweeper_solver::MineSweeperSolver;
use crate::minesweeper_solver::permutation_checker::islands::{search_for_islands, merge_islands};
use colored::Colorize;
use core::panic;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, thread, vec};


impl MineSweeperSolver {
    pub fn apply_permutation_checks(&mut self) -> Option<()> {
        let mut did_something = false;
        let max_mines = self.field.mines - self.flag_count;
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
            let mut possible_permutations: u64 = 0;
            let mut wrong_permutations: u64 = 0;
            let mut permutation_field: HashMap<(usize, usize), u64> = HashMap::new();
            let mut permutation_vector: Vec<((usize, usize), bool)> = vec![];
            
            for &(x, y) in island {
                if self.has_revealed_neighbours(x, y) {
                    permutation_field.insert((x, y), 0);
                    continue;
                }
            }

            for (&(x, y), _) in &permutation_field {
                permutation_vector.push(((x, y), false));
            }

            // useless !?
            sort_by_min_distance(&mut permutation_vector);
            println!("\n\nRunning for island with {} cells and {} testable fields with max_mines {}", island.len().to_string().green(), permutation_vector.len().to_string().green(), max_mines.to_string().red());

            self.recursively_apply_permutations(&mut permutation_vector.clone(), 0, max_mines, &mut permutation_field, &mut possible_permutations, &mut wrong_permutations);

            // apply found informations to the map
            println!("Possible Permutations: {}", possible_permutations.to_string().green());
            println!("Wrong Permutations: {}", wrong_permutations.to_string().red());
            //println!("Permutation Field: {:?}", permutation_field);
            //continue; // For now, dont

            if possible_permutations == 0 {
                continue; // No possible permutations found, skip this island
            }

            for ((x, y), permutation_mines) in permutation_field {
                if permutation_mines == 0 {
                    // Field is in every possible way empty
                    self.reveal_field(x, y);
                    did_something = true;
                }

                if permutation_mines == possible_permutations {
                    // Field is in every possible way a mine
                    self.flag_cell(x, y);
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

    fn recursively_apply_permutations(
        &mut self,
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

        // new method is overflagged? -> check if this makes sense what we are doing here ??? -> could be obsolete becase of the previous loop

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
        let mut possible_mine: u8 = 0;
        let mine_count = self.field.board[x][y].get_number();

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Flagged {
                flag_count += 1;
            } else if let Some(field) = permutation_vector.iter().find(|&&((x, y), _)| x == new_x && y == new_y) {
                if field.1 {
                    flag_count += 1;
                }
            }
            else if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                // This is a hidden field which is not in the current permutation vector
                // assume this can be a mine
                possible_mine += 1;
            }
        }

        if mine_count == flag_count {
            // all mines would be flagged, this is a valid permutation
            return true;
        //} else if mine_count > flag_count && mine_count <= (flag_count + possible_mine) {
        //    // there are not enough mines flagged, but there are also still hidden fields which would satisfy the number if set to a mine
        //    return true;
        } else {
            return false;
        }
    }

    fn insert_if_valid(
        &mut self,
        permutation_vector: &Vec<((usize, usize), bool)>,
        permutation_field: &mut HashMap<(usize, usize), u64>,
        possible_permutations: &mut u64,
        wrong_permutations: &mut u64
    ) {
        // This function rejects to much currently. (eg 2 1 cell islands neighbouring each other)
        // Get all neighbouring information fields to the permutation vector and check if they are satisfied
        for &((x, y), _mine) in permutation_vector {
            for (new_x, new_y) in self.field.surrounding_fields(x, y) {
                if self.has_informations(new_x, new_y) {
                    // check if the number has more hidden field neighbours which are / are not in the permutation vector and only check if the number is not overflagged
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