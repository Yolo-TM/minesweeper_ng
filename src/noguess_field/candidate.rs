use super::failed_moves::{FailedDoubleMoves, FailedMoves};
use super::frontier::Frontier;
use crate::solver::CellState;
use crate::{Cell, DefinedField, MineSweeperField};
use rayon::prelude::*;

pub(super) struct CandidatePicker;

impl CandidatePicker {
    /// Returns up to `batch_size` (remove, place) candidates.
    /// Heuristic candidates come first; brute-force fills the remainder if needed.
    /// Returns `None` only when every possible pair is exhausted (deadlock).
    pub(super) fn pick(
        frontiers: &[Frontier],
        field: &DefinedField,
        failed: &FailedMoves,
        solver_grid: &Vec<Vec<CellState>>,
        batch_size: usize,
    ) -> Option<Vec<((u32, u32), (u32, u32))>> {
        let multi = frontiers.len() > 1;

        // Collect all frontier mine cells ranked by revealed-number-neighbor count
        // (most constrained first — changing these mines affects the most deductions)
        let mut removal_candidates: Vec<(u32, u32)> = frontiers
            .iter()
            .flat_map(|f| f.hidden_cells().iter().copied())
            .filter(|&(x, y)| matches!(field.get_cell(x, y), Cell::Mine))
            .collect();

        removal_candidates.sort_by_key(|&(x, y)| {
            std::cmp::Reverse(revealed_number_neighbor_count(x, y, solver_grid))
        });

        // Placement target pool differs by frontier count
        let placement_pool: Vec<(u32, u32)> = if multi {
            // For each removal frontier, target the frontier with the fewest mines
            // We build a full placement list ordered by target-frontier mine count
            placement_multi_frontier(frontiers, field)
        } else {
            // Single frontier: interior cells first, same-frontier cells as fallback
            placement_single_frontier(frontiers, field, solver_grid)
        };

        // Build heuristic candidate list (skipping already-failed pairs)
        let mut candidates: Vec<((u32, u32), (u32, u32))> = Vec::new();

        'outer: for &remove in &removal_candidates {
            for &place in &placement_pool {
                if place == remove {
                    continue;
                }
                if failed.contains(&(remove, place)) {
                    continue;
                }
                candidates.push((remove, place));
                if candidates.len() >= batch_size {
                    break 'outer;
                }
            }
        }

        if candidates.len() >= batch_size {
            return Some(candidates);
        }

        // Brute-force fallback: enumerate all remaining pairs in parallel
        let all_mines: Vec<(u32, u32)> = frontiers
            .iter()
            .flat_map(|f| f.hidden_cells().iter().copied())
            .filter(|&(x, y)| matches!(field.get_cell(x, y), Cell::Mine))
            .collect();

        let all_targets: Vec<(u32, u32)> = all_non_mine_hidden_cells(field, solver_grid);

        let remaining: Vec<((u32, u32), (u32, u32))> = all_mines
            .par_iter()
            .flat_map(|&remove| {
                all_targets
                    .iter()
                    .filter(move |&&place| place != remove && !failed.contains(&(remove, place)))
                    .map(move |&place| (remove, place))
                    .collect::<Vec<_>>()
            })
            .collect();

        // Remove pairs already in the heuristic candidates
        let already: std::collections::HashSet<_> = candidates.iter().copied().collect();
        let mut extra: Vec<_> = remaining
            .into_iter()
            .filter(|p| !already.contains(p))
            .collect();

        if candidates.is_empty() && extra.is_empty() {
            return None; // deadlock
        }

        let needed = batch_size.saturating_sub(candidates.len());
        extra.truncate(needed);
        candidates.extend(extra);
        Some(candidates)
    }

    /// Returns up to `batch_size` double-move candidates: two independent (remove, place) pairs
    /// applied simultaneously. Used when all single-move options are exhausted.
    /// Returns `None` only when every double pair is also exhausted (true deadlock).
    pub(super) fn pick_double(
        frontiers: &[Frontier],
        field: &DefinedField,
        failed: &FailedDoubleMoves,
        solver_grid: &Vec<Vec<CellState>>,
        batch_size: usize,
    ) -> Option<Vec<((u32, u32), (u32, u32), (u32, u32), (u32, u32))>> {
        // Collect all frontier mine cells, most-constrained first
        let mut mines: Vec<(u32, u32)> = frontiers
            .iter()
            .flat_map(|f| f.hidden_cells().iter().copied())
            .filter(|&(x, y)| matches!(field.get_cell(x, y), Cell::Mine))
            .collect();
        mines.sort_by_key(|&(x, y)| {
            std::cmp::Reverse(revealed_number_neighbor_count(x, y, solver_grid))
        });
        mines.dedup();

        let targets: Vec<(u32, u32)> = all_non_mine_hidden_cells(field, solver_grid);

        if mines.len() < 2 || targets.len() < 2 {
            return None;
        }

        let mut candidates = Vec::new();

        'outer: for (i, &r1) in mines.iter().enumerate() {
            for &p1 in &targets {
                if p1 == r1 {
                    continue;
                }
                for &r2 in mines[i + 1..].iter() {
                    if r2 == r1 {
                        continue;
                    }
                    for &p2 in &targets {
                        if p2 == r2 || p2 == p1 || p2 == r1 {
                            continue;
                        }
                        if failed.contains(&(r1, p1, r2, p2)) {
                            continue;
                        }
                        candidates.push((r1, p1, r2, p2));
                        if candidates.len() >= batch_size {
                            break 'outer;
                        }
                    }
                }
            }
        }

        if candidates.is_empty() {
            None
        } else {
            Some(candidates)
        }
    }
}

fn revealed_number_neighbor_count(x: u32, y: u32, grid: &Vec<Vec<CellState>>) -> usize {
    let width = grid.len() as i32;
    let height = if width > 0 { grid[0].len() as i32 } else { 0 };
    let mut count = 0;
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width && ny < height {
                if matches!(
                    &grid[nx as usize][ny as usize],
                    CellState::Revealed(Cell::Number(_))
                ) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn placement_multi_frontier(frontiers: &[Frontier], field: &DefinedField) -> Vec<(u32, u32)> {
    // Sort target frontiers by mine count ascending (least-mined benefits most from a new mine)
    let mut sorted: Vec<&Frontier> = frontiers.iter().collect();
    sorted.sort_by_key(|f| f.mine_count());

    sorted
        .iter()
        .flat_map(|f| f.hidden_cells().iter().copied())
        .filter(|&(x, y)| !matches!(field.get_cell(x, y), Cell::Mine))
        .collect()
}

fn placement_single_frontier(
    frontiers: &[Frontier],
    field: &DefinedField,
    grid: &Vec<Vec<CellState>>,
) -> Vec<(u32, u32)> {
    let width = grid.len() as u32;
    let height = if width > 0 { grid[0].len() as u32 } else { 0 };

    // Interior = hidden cells not adjacent to any revealed number
    let mut interior: Vec<(u32, u32)> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if !matches!(&grid[x as usize][y as usize], CellState::Hidden(_)) {
                continue;
            }
            if matches!(field.get_cell(x, y), Cell::Mine) {
                continue;
            }
            if !has_revealed_number_neighbor(x, y, grid) {
                interior.push((x, y));
            }
        }
    }

    if !interior.is_empty() {
        return interior;
    }

    // Fallback: non-mine hidden cells on the frontier itself
    frontiers
        .iter()
        .flat_map(|f| f.hidden_cells().iter().copied())
        .filter(|&(x, y)| !matches!(field.get_cell(x, y), Cell::Mine))
        .collect()
}

fn has_revealed_number_neighbor(x: u32, y: u32, grid: &Vec<Vec<CellState>>) -> bool {
    let width = grid.len() as i32;
    let height = if width > 0 { grid[0].len() as i32 } else { 0 };
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width && ny < height {
                if matches!(
                    &grid[nx as usize][ny as usize],
                    CellState::Revealed(Cell::Number(_))
                ) {
                    return true;
                }
            }
        }
    }
    false
}

fn all_non_mine_hidden_cells(field: &DefinedField, grid: &Vec<Vec<CellState>>) -> Vec<(u32, u32)> {
    let width = grid.len() as u32;
    let height = if width > 0 { grid[0].len() as u32 } else { 0 };
    let mut result = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if matches!(&grid[x as usize][y as usize], CellState::Hidden(_))
                && !matches!(field.get_cell(x, y), Cell::Mine)
            {
                result.push((x, y));
            }
        }
    }
    result
}
