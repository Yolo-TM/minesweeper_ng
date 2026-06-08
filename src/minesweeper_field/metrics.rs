use super::{Cell, MineSweeperField};

pub trait MineSweeperFieldMetrics: MineSweeperField {
    /// Calculates the 3BV (Bechtel's Board Benchmark Value) of the field.
    ///
    /// 3BV is the minimum number of left-clicks required to clear the board,
    /// and is used as a measure of board difficulty.
    ///
    /// Each connected region of `Empty` cells (an "opening") costs 1 click.
    /// Every `Number` cell bordering an opening is claimed by it. Any remaining
    /// `Number` cell not adjacent to an opening costs 1 additional click.
    fn get_3bv(&self) -> u32 {
        let w = self.get_width() as usize;
        let h = self.get_height() as usize;
        let mut visited = vec![vec![false; h]; w];
        let mut count = 0;

        // Step 1: flood-fill each opening (connected Empty cell regions).
        // Number cells bordering an opening are claimed (visited) but don't spread.
        for (x, y) in self.sorted_fields() {
            let xi = x as usize;
            let yi = y as usize;
            if visited[xi][yi] || self.get_cell(x, y) != &Cell::Empty {
                continue;
            }

            count += 1;
            let mut stack = vec![(x, y)];
            while let Some((cx, cy)) = stack.pop() {
                let cxi = cx as usize;
                let cyi = cy as usize;
                if visited[cxi][cyi] {
                    continue;
                }
                visited[cxi][cyi] = true;
                if self.get_cell(cx, cy) == &Cell::Empty {
                    for (nx, ny) in self.surrounding_fields(cx, cy, None) {
                        if !visited[nx as usize][ny as usize] {
                            stack.push((nx, ny));
                        }
                    }
                }
            }
        }

        // Step 2: each Number cell not claimed by an opening costs one additional click.
        for (x, y) in self.sorted_fields() {
            if !visited[x as usize][y as usize] {
                if matches!(self.get_cell(x, y), Cell::Number(_)) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl<T: MineSweeperField> MineSweeperFieldMetrics for T {}
