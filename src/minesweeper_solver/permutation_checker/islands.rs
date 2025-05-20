use crate::minesweeper_solver::{MineSweeperSolver, MineSweeperCellState};
use crate::field_generator::MineSweeperField;

pub fn search_for_islands<M: MineSweeperField>(game: &MineSweeperSolver<M>) -> Vec<Vec<(u32, u32)>> {
    let mut visited = vec![vec![false; game.field.get_height() as usize]; game.field.get_width() as usize];
    let mut islands = vec![];

    for (x, y) in game.field.sorted_fields() {
        if visited[x as usize][y as usize] || game.get_state(x, y) != MineSweeperCellState::Hidden {
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

fn recursive_search<M: MineSweeperField>(x: u32, y: u32, fields: &mut Vec<(u32,u32)>, visited : &mut Vec<Vec<bool>>, game: &MineSweeperSolver<M>) {
    visited[x as usize][y as usize] = true;
    fields.push((x, y));

    for (new_x, new_y) in game.field.surrounding_fields(x, y, None) {
        if !visited[new_x as usize][new_y as usize] && game.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
            recursive_search(new_x, new_y, fields, visited, game);
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