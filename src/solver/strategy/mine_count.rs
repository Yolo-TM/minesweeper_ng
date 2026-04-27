use super::{Finding, Solver};
use crate::solver::CellState;

/*
Mine count strategy:

If the total number of remaining unflagged mines equals zero, all hidden cells are safe.
This catches end-game states where all mines are accounted for by flags but some
hidden cells haven't been opened yet because they were never adjacent to a revealed number.
*/

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    if solver.get_remaining_mines() != 0 {
        return finding;
    }

    for (x, y) in solver.sorted_fields() {
        if matches!(solver.get_state(x, y), CellState::Hidden(_)) {
            finding.add_safe_field((x, y));
        }
    }

    finding
}
