use crate::solver::CellState;
use crate::{Cell, MineSweeperField};
use std::collections::{HashSet, VecDeque};

pub(super) struct Frontier {
    hidden_cells: Vec<(u32, u32)>,
    mine_count: u32,
}

impl Frontier {
    /// Groups hidden border cells into frontiers by BFS over revealed number cells.
    /// Each connected group of revealed numbers defines one frontier; all hidden cells
    /// adjacent to that group belong to it. Mine count is read from the actual board.
    pub(super) fn identify_all(
        grid: &Vec<Vec<CellState>>,
        board: &impl MineSweeperField,
    ) -> Vec<Frontier> {
        let width = grid.len() as u32;
        let height = if width > 0 {
            grid[0].len() as u32
        } else {
            return vec![];
        };

        let is_revealed_number = |x: u32, y: u32| -> bool {
            matches!(
                &grid[x as usize][y as usize],
                CellState::Revealed(Cell::Number(_))
            )
        };

        let is_hidden = |x: u32, y: u32| -> bool {
            matches!(&grid[x as usize][y as usize], CellState::Hidden(_))
        };

        let neighbors = |x: u32, y: u32| -> Vec<(u32, u32)> {
            let mut result = Vec::with_capacity(8);
            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        result.push((nx as u32, ny as u32));
                    }
                }
            }
            result
        };

        // BFS over revealed number cells to find connected groups.
        // Two revealed number cells are connected if they share a hidden neighbor
        // (same approach as the SAT solver's component detection).
        let mut visited_revealed: HashSet<(u32, u32)> = HashSet::new();
        let mut frontiers: Vec<Frontier> = Vec::new();

        for sx in 0..width {
            for sy in 0..height {
                if !is_revealed_number(sx, sy) || visited_revealed.contains(&(sx, sy)) {
                    continue;
                }

                // BFS to collect this connected group of revealed number cells
                let mut group: Vec<(u32, u32)> = Vec::new();
                let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
                queue.push_back((sx, sy));
                visited_revealed.insert((sx, sy));

                while let Some((cx, cy)) = queue.pop_front() {
                    group.push((cx, cy));

                    // Connect revealed number cells that share a hidden neighbor
                    let hidden_of_cx: Vec<(u32, u32)> = neighbors(cx, cy)
                        .into_iter()
                        .filter(|&(nx, ny)| is_hidden(nx, ny))
                        .collect();

                    for (nx, ny) in neighbors(cx, cy) {
                        if !is_revealed_number(nx, ny) || visited_revealed.contains(&(nx, ny)) {
                            continue;
                        }
                        // Connect if they share at least one hidden neighbor
                        let shares_hidden = neighbors(nx, ny)
                            .into_iter()
                            .any(|nb| hidden_of_cx.contains(&nb));
                        if shares_hidden {
                            visited_revealed.insert((nx, ny));
                            queue.push_back((nx, ny));
                        }
                    }
                }

                // Collect all hidden cells adjacent to this group (deduped)
                let mut hidden_set: HashSet<(u32, u32)> = HashSet::new();
                for &(gx, gy) in &group {
                    for (nx, ny) in neighbors(gx, gy) {
                        if is_hidden(nx, ny) {
                            hidden_set.insert((nx, ny));
                        }
                    }
                }

                let hidden_cells: Vec<(u32, u32)> = hidden_set.into_iter().collect();
                let mine_count = hidden_cells
                    .iter()
                    .filter(|&&(hx, hy)| matches!(board.get_cell(hx, hy), Cell::Mine))
                    .count() as u32;

                frontiers.push(Frontier {
                    hidden_cells,
                    mine_count,
                });
            }
        }

        frontiers
    }

    pub(super) fn hidden_cells(&self) -> &[(u32, u32)] {
        &self.hidden_cells
    }

    pub(super) fn mine_count(&self) -> u32 {
        self.mine_count
    }
}
