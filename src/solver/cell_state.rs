use crate::Cell;
use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub enum CellState {
    Hidden(Cell),
    Revealed(Cell),
    Flagged(Cell),
}

impl CellState {
    pub fn get_colored(&self) -> ColoredString {
        match self {
            CellState::Hidden(_cell) => "?".black().bold(),
            CellState::Revealed(cell) => cell.get_colored(),
            CellState::Flagged(_cell) => "F".red().bold(),
        }
    }

    pub fn get_cell(&self) -> &Cell {
        match self {
            CellState::Hidden(cell) => cell,
            CellState::Revealed(cell) => cell,
            CellState::Flagged(cell) => cell,
        }
    }
}
