use super::Solver;
use crate::CellState;
use crate::Cell;
use log::debug;

impl Solver {
    pub(super) fn get_remaining_mines(&self) -> u32 {
        let mut flagged_count = 0;
        for (x, y) in self.sorted_fields() {
            if self.get_state(x, y).is_flagged() {
                flagged_count += 1;
            }
        }
        self.mines.saturating_sub(flagged_count)
    }

    pub(super) fn get_state(&self, x: u32, y: u32) -> &CellState {
        &self.state[x as usize][y as usize]
    }

    pub(super) fn flag_cell(&mut self, x: u32, y: u32) {
        // flag() enforces Hidden → Flagged; silently ignore invalid transitions
        let _ = self.state[x as usize][y as usize].flag();
    }

    #[track_caller]
    pub(super) fn reveal_cell(
        &mut self,
        x: u32,
        y: u32,
        recursive_revealed_fields: &mut Vec<Vec<(u32, u32)>>,
        depth: usize,
    ) {
        if self.get_state(x, y).is_revealed() {
            return;
        }
        let cell = self.get_state(x, y).cell().clone();

        match cell {
            Cell::Mine => {
                debug!("{}", self.format_field_state());
                debug!("Stepped on a mine at ({}, {})! Solver failed.", x, y);
                panic!("Solver hit a mine!");
            }
            Cell::Number(n) => {
                // reveal() enforces Hidden → Revealed
                self.state[x as usize][y as usize]
                    .reveal()
                    .expect("reveal_cell called on non-hidden cell");

                if self.get_surrounding_flag_count(x, y) == n {
                    self.reveal_surrounding_cells(x, y, recursive_revealed_fields, depth);
                }
            }
            Cell::Empty => {
                self.state[x as usize][y as usize]
                    .reveal()
                    .expect("reveal_cell called on non-hidden cell");

                self.reveal_surrounding_cells(x, y, recursive_revealed_fields, depth);
            }
        }
    }

    #[track_caller]
    pub(super) fn reveal_surrounding_cells(
        &mut self,
        x: u32,
        y: u32,
        recursive_revealed_fields: &mut Vec<Vec<(u32, u32)>>,
        depth: usize,
    ) {
        // Ensure a vector exists for this depth
        while recursive_revealed_fields.len() <= depth {
            recursive_revealed_fields.push(vec![]);
        }

        for (sx, sy) in self.surrounding_fields(x, y, None) {
            if self.get_state(sx, sy).is_hidden() {
                recursive_revealed_fields[depth].push((sx, sy));
                self.reveal_cell(sx, sy, recursive_revealed_fields, depth + 1);
            }
        }
    }

    pub(super) fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        self.surrounding_fields(x, y, None)
            .any(|(nx, ny)| self.get_state(nx, ny).is_hidden())
    }

    pub(super) fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        self.surrounding_fields(x, y, None)
            .filter(|&(sx, sy)| self.get_state(sx, sy).is_flagged())
            .count() as u8
    }

    pub(super) fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        self.surrounding_fields(x, y, None)
            .filter(|&(nx, ny)| self.get_state(nx, ny).is_hidden())
            .count() as u8
    }

    pub(super) fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        self.surrounding_fields(x, y, None)
            .filter(|&(nx, ny)| self.get_state(nx, ny).is_hidden())
            .collect()
    }

    pub(super) fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = match self.get_state(x, y).cell() {
            Cell::Number(n) => n,
            _ => panic!(
                "get_reduced_count called on non-number cell at ({}, {})",
                x, y
            ),
        };

        if flag_count > *number {
            panic!(
                "Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}",
                x, y, flag_count, number
            );
        }

        number - flag_count
    }

    pub(super) fn has_informations(&self, x: u32, y: u32) -> bool {
        self.get_state(x, y).is_revealed()
            && matches!(self.get_state(x, y).cell(), Cell::Number(_))
            && self.has_unrevealed_neighbours(x, y)
    }
}
