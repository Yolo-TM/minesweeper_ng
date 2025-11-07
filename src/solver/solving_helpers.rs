use core::panic;
use crate::Cell;
use super::{Solver, CellState};

impl Solver {

    pub(super) fn get_state(&self, x: u32, y: u32) -> &CellState {
        &self.state[x as usize][y as usize]
    }

    pub(super) fn flag_cell(&mut self, x: u32, y: u32) {
        let state = self.get_state(x, y);

        if !matches!(state, CellState::Hidden(_)) {
            return;
        }

        // Don't bother checking if it's actually a mine here, if its no mine, were definitely hitting one in the next step
        let cell = state.get_cell().clone();
        self.state[x as usize][y as usize] = CellState::Flagged(cell);
    }

    #[track_caller]
    pub(super) fn reveal_cell(&mut self, x: u32, y: u32) {
        let state = self.get_state(x, y);

        if let CellState::Revealed(_) = state {
            return;
        }
        let cell = state.get_cell().clone();

        match cell {
            Cell::Mine => {
                self.print_field(2);
                self.println(&format!("Stepped on a mine at ({}, {})! Solver failed.", x, y), 2);
                panic!("Solver hit a mine!");
            }
            Cell::Number(n) => {
                self.state[x as usize][y as usize] = CellState::Revealed(cell);

                if self.get_surrounding_flag_count(x, y) == n {
                    self.reveal_surrounding_cells(x, y);
                }
            }
            Cell::Empty => {
                self.state[x as usize][y as usize] = CellState::Revealed(cell);

                self.reveal_surrounding_cells(x, y);
            }
        }
    }

    #[track_caller]
    pub(super) fn reveal_surrounding_cells(&mut self, x: u32, y: u32) {
        for (sx, sy) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(sx, sy) {
                self.reveal_cell(sx, sy);
            }
        }
    }

    pub(super) fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                return true;
            }
        }

        false
    }

    pub(super) fn has_revealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Revealed(_) = self.get_state(new_x, new_y) {
                return true;
            }
        }

        false
    }

    pub(super) fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        let mut flag_count = 0;

        for (sx, sy) in self.surrounding_fields(x, y, None) {
            if let CellState::Flagged(_) = self.get_state(sx, sy) {
                flag_count += 1;
            }
        }

        flag_count
    }

    pub(super) fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                count += 1;
            }
        }

        count
    }

    pub(super) fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.surrounding_fields(x, y, None) {
            if let CellState::Hidden(_) = self.get_state(new_x, new_y) {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub(super) fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = match self.get_state(x, y).get_cell() {
            Cell::Number(n) => n,
            _ => panic!("get_reduced_count called on non-number cell at ({}, {})", x, y),
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
        matches!(self.get_state(x, y), CellState::Revealed(_))
        && matches!(self.get_state(x, y).get_cell(), Cell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }
}