use super::Solver;
use crate::Cell;
use crate::CellState;
use crate::game_ops::{self, RevealResult};

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
        let _ = game_ops::flag_cell(&mut self.state, x, y);
    }

    pub(super) fn reveal_cell(
        &mut self,
        x: u32,
        y: u32,
        recursive_revealed_fields: &mut Vec<Vec<(u32, u32)>>,
    ) {
        let result = game_ops::reveal_cell(&mut self.state, self.width, self.height, x, y);

        match result {
            RevealResult::Mine => panic!("Solver hit a mine at ({}, {})!", x, y),
            RevealResult::Revealed { cascades, .. } => {
                // Map cascade depth layers into the solver's recursive_revealed_fields format
                for (i, wave) in cascades.into_iter().enumerate() {
                    while recursive_revealed_fields.len() <= i {
                        recursive_revealed_fields.push(vec![]);
                    }
                    recursive_revealed_fields[i].extend(wave);
                }
            }
            RevealResult::Chord { cascades, .. } => {
                for (i, wave) in cascades.into_iter().enumerate() {
                    while recursive_revealed_fields.len() <= i {
                        recursive_revealed_fields.push(vec![]);
                    }
                    recursive_revealed_fields[i].extend(wave);
                }
            }
            RevealResult::AlreadyRevealed | RevealResult::Flagged => {}
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
