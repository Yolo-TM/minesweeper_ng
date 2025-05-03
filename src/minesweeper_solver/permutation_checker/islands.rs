use crate::field_generator::minesweeper_field::MineSweeperField;
use crate::field_generator::minesweeper_cell::MineSweeperCell;
use crate::minesweeper_solver::box_logic::boxes::Box;
use crate::minesweeper_solver::MineSweeperSolver;
use crate::minesweeper_solver::MineSweeperCellState;
use colored::Colorize;
use core::panic;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, thread, vec};


pub fn search_for_islands(game: &MineSweeperSolver) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; game.field.height]; game.field.width];
    let mut islands = vec![];

    for (x, y) in game.field.sorted_fields() {
        if visited[x][y] || game.state[x][y] != MineSweeperCellState::Hidden {
            continue;
        }

        let mut fields = vec![];
        recursive_search(x, y, &mut fields, &mut visited, game);
        if fields.len() > 0 {
            islands.push(fields);
        }
    }

    islands
}

fn recursive_search(x: usize, y: usize, fields: &mut Vec<(usize,usize)>, visited : &mut Vec<Vec<bool>>, game: &MineSweeperSolver) {
    visited[x][y] = true;
    fields.push((x, y));

    for (new_x, new_y) in game.field.surrounding_fields(x, y) {
        if !visited[new_x][new_y] && game.state[new_x][new_y] == MineSweeperCellState::Hidden {
            recursive_search(new_x, new_y, fields, visited, game);
        }
    }
}

pub fn merge_islands(islands: Vec<Vec<(usize, usize)>>, max_distance: usize, max_size: usize) -> Vec<Vec<(usize, usize)>> {
    let mut merged_islands: Vec<Vec<(usize, usize)>> = vec![];
    let mut visited = vec![false; islands.len()];

    for i in 0..islands.len() {
        if visited[i] {
            continue;
        }

        let mut new_island = islands[i].clone();
        visited[i] = true;

        for j in 0..islands.len() {
            if i == j || visited[j] {
                continue;
            }

            // Check if the islands are within reach
            if are_islands_within_reach(&new_island, &islands[j], max_distance) {
                // Merge the islands if the new size does not exceed the max size
                if new_island.len() + islands[j].len() <= max_size {
                    new_island.extend(&islands[j]);
                    visited[j] = true;
                }
            }
        }

        merged_islands.push(new_island);
    }

    merged_islands
}

// Helper function to check if two islands are within the max distance
fn are_islands_within_reach(island1: &Vec<(usize, usize)>, island2: &Vec<(usize, usize)>, max_distance: usize) -> bool {
    for &(x1, y1) in island1 {
        for &(x2, y2) in island2 {
            if manhattan_distance((x1, y1), (x2, y2)) <= max_distance {
                return true;
            }
        }
    }
    false
}

// Helper function to calculate Manhattan distance between two points
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}