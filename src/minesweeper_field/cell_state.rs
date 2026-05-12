use colored::{ColoredString, Colorize};

use super::Cell;

/// Tracks the player-visible state of a single board cell.
///
/// Uses a state machine to enforce valid transitions:
///
/// ```text
///   Hidden ──► Revealed     (reveal)
///   Hidden ◄──► Flagged     (flag / unflag)
/// ```
///
/// Invalid transitions (e.g. revealing a flagged cell, flagging a revealed cell)
/// return `Err(InvalidTransition)`.
#[derive(Clone, Debug)]
pub enum CellState {
    Hidden(Cell),
    Revealed(Cell),
    Flagged(Cell),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidTransition {
    pub from: &'static str,
    pub to: &'static str,
}

impl std::fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid transition: {} → {}", self.from, self.to)
    }
}

impl CellState {
    /// Returns the underlying field cell, regardless of visibility.
    pub fn cell(&self) -> &Cell {
        match self {
            Self::Hidden(c) | Self::Revealed(c) | Self::Flagged(c) => c,
        }
    }

    /// Alias for `cell()` — used by the solver.
    pub fn get_cell(&self) -> &Cell {
        self.cell()
    }

    pub fn get_colored(&self) -> ColoredString {
        match self {
            CellState::Hidden(_) => "?".black().bold(),
            CellState::Revealed(cell) => cell.get_colored(),
            CellState::Flagged(_) => "F".red().bold(),
        }
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden(_))
    }

    pub fn is_revealed(&self) -> bool {
        matches!(self, Self::Revealed(_))
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self, Self::Flagged(_))
    }

    /// Hidden → Revealed. Fails from Revealed or Flagged.
    pub fn reveal(&mut self) -> Result<(), InvalidTransition> {
        match self {
            Self::Hidden(_) => {
                *self = Self::Revealed(self.cell().clone());
                Ok(())
            }
            Self::Revealed(_) => Err(InvalidTransition {
                from: "Revealed",
                to: "Revealed",
            }),
            Self::Flagged(_) => Err(InvalidTransition {
                from: "Flagged",
                to: "Revealed",
            }),
        }
    }

    /// Hidden → Flagged. Fails from Revealed or Flagged.
    pub fn flag(&mut self) -> Result<(), InvalidTransition> {
        match self {
            Self::Hidden(_) => {
                *self = Self::Flagged(self.cell().clone());
                Ok(())
            }
            Self::Revealed(_) => Err(InvalidTransition {
                from: "Revealed",
                to: "Flagged",
            }),
            Self::Flagged(_) => Err(InvalidTransition {
                from: "Flagged",
                to: "Flagged",
            }),
        }
    }

    /// Flagged → Hidden. Fails from Hidden or Revealed.
    pub fn unflag(&mut self) -> Result<(), InvalidTransition> {
        match self {
            Self::Flagged(_) => {
                *self = Self::Hidden(self.cell().clone());
                Ok(())
            }
            Self::Hidden(_) => Err(InvalidTransition {
                from: "Hidden",
                to: "Hidden",
            }),
            Self::Revealed(_) => Err(InvalidTransition {
                from: "Revealed",
                to: "Hidden",
            }),
        }
    }
}
