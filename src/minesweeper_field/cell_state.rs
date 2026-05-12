use colored::{ColoredString, Colorize};

use super::Cell;

/// Visibility state of a single board cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Hidden,
    Revealed,
    Flagged,
}

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
pub struct CellState {
    cell: Cell,
    visibility: Visibility,
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
    /// Creates a new cell state, defaulting to `Hidden`.
    pub fn new(cell: Cell) -> Self {
        Self {
            cell,
            visibility: Visibility::Hidden,
        }
    }

    /// Creates a new cell state with the given visibility.
    pub fn with_visibility(cell: Cell, visibility: Visibility) -> Self {
        Self { cell, visibility }
    }

    /// Returns the underlying field cell, regardless of visibility.
    pub fn cell(&self) -> &Cell {
        &self.cell
    }

    /// Alias for `cell()` — used by the solver.
    pub fn get_cell(&self) -> &Cell {
        self.cell()
    }

    /// Returns the current visibility state.
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn get_colored(&self) -> ColoredString {
        match self.visibility {
            Visibility::Hidden => "?".black().bold(),
            Visibility::Revealed => self.cell.get_colored(),
            Visibility::Flagged => "F".red().bold(),
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.visibility == Visibility::Hidden
    }

    pub fn is_revealed(&self) -> bool {
        self.visibility == Visibility::Revealed
    }

    pub fn is_flagged(&self) -> bool {
        self.visibility == Visibility::Flagged
    }

    /// Hidden → Revealed. Fails from Revealed or Flagged.
    pub fn reveal(&mut self) -> Result<(), InvalidTransition> {
        match self.visibility {
            Visibility::Hidden => {
                self.visibility = Visibility::Revealed;
                Ok(())
            }
            Visibility::Revealed => Err(InvalidTransition {
                from: "Revealed",
                to: "Revealed",
            }),
            Visibility::Flagged => Err(InvalidTransition {
                from: "Flagged",
                to: "Revealed",
            }),
        }
    }

    /// Hidden → Flagged. Fails from Revealed or Flagged.
    pub fn flag(&mut self) -> Result<(), InvalidTransition> {
        match self.visibility {
            Visibility::Hidden => {
                self.visibility = Visibility::Flagged;
                Ok(())
            }
            Visibility::Revealed => Err(InvalidTransition {
                from: "Revealed",
                to: "Flagged",
            }),
            Visibility::Flagged => Err(InvalidTransition {
                from: "Flagged",
                to: "Flagged",
            }),
        }
    }

    /// Flagged → Hidden. Fails from Hidden or Revealed.
    pub fn unflag(&mut self) -> Result<(), InvalidTransition> {
        match self.visibility {
            Visibility::Flagged => {
                self.visibility = Visibility::Hidden;
                Ok(())
            }
            Visibility::Hidden => Err(InvalidTransition {
                from: "Hidden",
                to: "Hidden",
            }),
            Visibility::Revealed => Err(InvalidTransition {
                from: "Revealed",
                to: "Hidden",
            }),
        }
    }
}
