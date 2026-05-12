use super::cell::Cell;
use super::cell_state::{CellState, InvalidTransition};
use super::iterators::SurroundingCells;

/// Result of revealing a single cell on a grid.
#[derive(Clone, Debug)]
pub enum RevealResult {
    /// Cell was already revealed — no-op.
    AlreadyRevealed,
    /// Cell is flagged — must unflag first.
    Flagged,
    /// Player hit a mine.
    Mine,
    /// Successfully revealed one or more cells (with recursive cascade).
    Revealed {
        /// All cells that were revealed (clicked cell first).
        cells: Vec<(u32, u32)>,
        /// Recursive cascade depth layers (each inner Vec is one expansion layer).
        cascades: Vec<Vec<(u32, u32)>>,
    },
    /// Chord action on an already-revealed numbered cell.
    Chord {
        /// Cells that were auto-flagged (when hidden + flags == number).
        flagged: Vec<(u32, u32)>,
        /// Cells that were revealed (when flags == number).
        cells: Vec<(u32, u32)>,
        /// Recursive cascade depth layers from the revealed cells.
        cascades: Vec<Vec<(u32, u32)>>,
    },
}

/// Reveal the cell at `(x, y)` on the grid.
///
/// - **Hidden cell**: reveals it and cascades (empty cells and numbered cells
///   whose flag count matches their number expand into neighbours).
/// - **Revealed numbered cell** (chord):
///   - If `hidden + flags == number`, auto-flags all hidden neighbours.
///   - If `flags == number`, reveals all hidden neighbours (may hit a mine).
/// - **Revealed non-numbered / flagged**: no-op.
///
/// Returns `RevealResult::Mine` on mine hit instead of panicking.
pub fn reveal_cell(
    grid: &mut [Vec<CellState>],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
) -> RevealResult {
    let cs = &grid[x as usize][y as usize];

    if cs.is_revealed() {
        return chord(grid, width, height, x, y);
    }
    if cs.is_flagged() {
        return RevealResult::Flagged;
    }

    let cell = *cs.cell();

    if cell == Cell::Mine {
        let _ = grid[x as usize][y as usize].reveal();
        return RevealResult::Mine;
    }

    let mut cascades: Vec<Vec<(u32, u32)>> = Vec::new();
    reveal_recursive(grid, width, height, x, y, &mut cascades, 0);

    // Flatten cascades into a single cells list (clicked cell first)
    let mut cells = vec![(x, y)];
    for wave in &cascades {
        for &coord in wave {
            cells.push(coord);
        }
    }

    RevealResult::Revealed { cells, cascades }
}

/// Flag the cell at `(x, y)`. Returns `Ok(())` or the invalid transition error.
pub fn flag_cell(grid: &mut [Vec<CellState>], x: u32, y: u32) -> Result<(), InvalidTransition> {
    grid[x as usize][y as usize].flag()
}

/// Unflag the cell at `(x, y)`. Returns `Ok(())` or the invalid transition error.
pub fn unflag_cell(grid: &mut [Vec<CellState>], x: u32, y: u32) -> Result<(), InvalidTransition> {
    grid[x as usize][y as usize].unflag()
}

// --- Chord ---

fn chord(grid: &mut [Vec<CellState>], width: u32, height: u32, x: u32, y: u32) -> RevealResult {
    let cell = *grid[x as usize][y as usize].cell();
    let n = match cell {
        Cell::Number(n) => n,
        _ => return RevealResult::AlreadyRevealed,
    };

    let flag_count = surrounding_flag_count(grid, width, height, x, y);
    let hidden_count = surrounding_hidden_count(grid, width, height, x, y);

    // Auto-flag: if hidden + flags == number, all hidden neighbours must be mines
    if hidden_count > 0 && hidden_count + flag_count == n {
        let to_flag: Vec<(u32, u32)> = surrounding(x, y, width, height)
            .filter(|&(sx, sy)| grid[sx as usize][sy as usize].is_hidden())
            .collect();
        let mut flagged = Vec::with_capacity(to_flag.len());
        for (fx, fy) in to_flag {
            let _ = grid[fx as usize][fy as usize].flag();
            flagged.push((fx, fy));
        }
        return RevealResult::Chord {
            flagged,
            cells: vec![],
            cascades: vec![],
        };
    }

    // Standard chord: if flags == number, reveal all hidden neighbours
    if flag_count != n {
        return RevealResult::AlreadyRevealed;
    }

    let to_reveal: Vec<(u32, u32)> = surrounding(x, y, width, height)
        .filter(|&(sx, sy)| grid[sx as usize][sy as usize].is_hidden())
        .collect();

    if to_reveal.is_empty() {
        return RevealResult::AlreadyRevealed;
    }

    let mut cascades: Vec<Vec<(u32, u32)>> = Vec::new();
    let mut cells = Vec::new();

    for (rx, ry) in to_reveal {
        // Mine hit during chord — player flagged wrong cells
        if *grid[rx as usize][ry as usize].cell() == Cell::Mine {
            let _ = grid[rx as usize][ry as usize].reveal();
            return RevealResult::Mine;
        }
        cells.push((rx, ry));
        reveal_recursive(grid, width, height, rx, ry, &mut cascades, 0);
    }

    RevealResult::Chord {
        flagged: vec![],
        cells,
        cascades,
    }
}

// --- Internal recursive reveal ---

fn reveal_recursive(
    grid: &mut [Vec<CellState>],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    cascades: &mut Vec<Vec<(u32, u32)>>,
    depth: usize,
) {
    if grid[x as usize][y as usize].is_revealed() {
        return;
    }
    let cell = *grid[x as usize][y as usize].cell();

    match cell {
        Cell::Mine => {
            // Should not be reached — caller checks for mines before entering recursion.
            // If it somehow happens, just reveal and return (don't panic).
            let _ = grid[x as usize][y as usize].reveal();
        }
        Cell::Number(n) => {
            grid[x as usize][y as usize]
                .reveal()
                .expect("reveal_recursive called on non-hidden cell");

            if surrounding_flag_count(grid, width, height, x, y) == n {
                reveal_surrounding(grid, width, height, x, y, cascades, depth);
            }
        }
        Cell::Empty => {
            grid[x as usize][y as usize]
                .reveal()
                .expect("reveal_recursive called on non-hidden cell");

            reveal_surrounding(grid, width, height, x, y, cascades, depth);
        }
    }
}

fn reveal_surrounding(
    grid: &mut [Vec<CellState>],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    cascades: &mut Vec<Vec<(u32, u32)>>,
    depth: usize,
) {
    while cascades.len() <= depth {
        cascades.push(vec![]);
    }

    let neighbors: Vec<(u32, u32)> = surrounding(x, y, width, height)
        .filter(|&(sx, sy)| grid[sx as usize][sy as usize].is_hidden())
        .collect();

    for (sx, sy) in neighbors {
        cascades[depth].push((sx, sy));
        reveal_recursive(grid, width, height, sx, sy, cascades, depth + 1);
    }
}

// --- Helpers ---

fn surrounding(x: u32, y: u32, width: u32, height: u32) -> SurroundingCells {
    SurroundingCells {
        x,
        y,
        width,
        height,
        range: 1,
        dx: -1,
        dy: -1,
    }
}

fn surrounding_flag_count(grid: &[Vec<CellState>], width: u32, height: u32, x: u32, y: u32) -> u8 {
    surrounding(x, y, width, height)
        .filter(|&(sx, sy)| grid[sx as usize][sy as usize].is_flagged())
        .count() as u8
}

fn surrounding_hidden_count(
    grid: &[Vec<CellState>],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
) -> u8 {
    surrounding(x, y, width, height)
        .filter(|&(sx, sy)| grid[sx as usize][sy as usize].is_hidden())
        .count() as u8
}
