use crate::minesweeper_solver::{MineSweeperSolver, MineSweeperCellState};
use crate::field_generator::{MineSweeperFieldIterator, SurroundingFieldsIterator, MineSweeperField, MineSweeperCell};

pub fn search_for_islands(width: u32, height: u32, field: &Vec<Vec<MineSweeperCell>>, state: &Vec<Vec<MineSweeperCellState>>) -> Vec<Vec<(u32, u32)>> {
    let mut visited = vec![vec![false; height as usize]; width as usize];
    let mut islands = vec![];

    for (x, y) in (MineSweeperFieldIterator {
            width,
            height,
            current_x: 0,
            current_y: 0,
        }) {
        if visited[x as usize][y as usize] || state[x as usize][y as usize] != MineSweeperCellState::Hidden {
            continue;
        }

        let mut fields = vec![];
        recursive_search(x, y, &mut fields, &mut visited, width, height, field, state);
        if fields.len() > 0 {
            islands.push(fields);
        }
    }

    islands
}

fn recursive_search(x: u32, y: u32, fields: &mut Vec<(u32,u32)>, visited : &mut Vec<Vec<bool>>, width: u32, height: u32, field: &Vec<Vec<MineSweeperCell>>, state: &Vec<Vec<MineSweeperCellState>>) {
    visited[x as usize][y as usize] = true;
    fields.push((x, y));

    for (new_x, new_y) in (SurroundingFieldsIterator {
            x,
            y,
            width,
            height,
            range: 1,
            dx: -1,
            dy: -1,
        }) {
        if !visited[new_x as usize][new_y as usize] && state[new_x as usize][new_y as usize] == MineSweeperCellState::Hidden {
            recursive_search(new_x, new_y, fields, visited, width, height, field, state);
        }
    }
}

pub fn merge_islands(islands: Vec<Vec<(u32, u32)>>, max_distance: u32, max_size: usize) -> Vec<Vec<(u32, u32)>> {
    let mut merged_islands: Vec<Vec<(u32, u32)>> = vec![];
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
fn are_islands_within_reach(island1: &Vec<(u32, u32)>, island2: &Vec<(u32, u32)>, max_distance: u32) -> bool {
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
fn manhattan_distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as u32
}