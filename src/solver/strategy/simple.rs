use super::{Solver, Finding};

/*
Simple strategy:

For each revealed cell with neighbouring hidden cells:
- If the reduced mine count (Number of Cells - Number of bordering Flagged Cells) is 0, all neighbouring hidden cells are safe to reveal.
- If the reduced mine count equals the number of neighbouring hidden cells, all those hidden cells are mines and should be flagged.
*/

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        let needed_mines = solver.get_reduced_count(x, y);
        let fields = solver.get_surrounding_unrevealed(x, y);

        if needed_mines == 0 {
            // Only add new fields to avoid duplicates
            finding.add_safe_fields(fields);
        } else if needed_mines == fields.len() as u8 {
            finding.add_mine_fields(fields);
        }
    }

    finding
}
