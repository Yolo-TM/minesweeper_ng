use super::Solver;

/*
Simple strategy:

For each revealed cell with neighbouring hidden cells:
- If the reduced mine count (Number of Cells - Number of bordering Flagged Cells) is 0, all neighbouring hidden cells are safe to reveal.
- If the reduced mine count equals the number of neighbouring hidden cells, all those hidden cells are mines and should be flagged.
*/

pub fn solve(solver: &Solver) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut reveale: Vec<(u32, u32)> = Vec::new();
    let mut flag: Vec<(u32, u32)> = Vec::new();

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        let needed_mines = solver.get_reduced_count(x, y);
        let fields = solver.get_surrounding_unrevealed(x, y);

        if needed_mines == 0 {
            reveale.extend(fields);
        } else if needed_mines == fields.len() as u8 {
            flag.extend(fields);
        }
    }

    (reveale, flag)
}
