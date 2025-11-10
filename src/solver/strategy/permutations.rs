use super::{Solver, Finding};

/*
Simple strategy:

For each revealed cell with neighbouring hidden cells:
- If the reduced mine count (Number of Cells - Number of bordering Flagged Cells) is 0, all neighbouring hidden cells are safe to reveal.
- If the reduced mine count equals the number of neighbouring hidden cells, all those hidden cells are mines and should be flagged.
*/

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    // get a list of all hidden fields which neighbours a revealed cell to try all 
    // create a copy which directly sets the flag to be able to use helpers for validation?

    finding
}
