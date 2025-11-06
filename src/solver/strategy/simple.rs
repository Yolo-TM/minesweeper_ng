use super::{Cell, CellState, Solver};

pub fn solve(solver: &Solver) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut reveale: Vec<(u32, u32)> = Vec::new();
    let mut flag: Vec<(u32, u32)> = Vec::new();

    for (x, y) in solver.sorted_fields() {
        if let CellState::Revealed(_) = solver.get_state(x, y)
        && matches!(solver.get_state(x, y).get_cell(), Cell::Number(_))
        && solver.has_unrevealed_neighbours(x, y) {
            let needed_mines = solver.get_reduced_count(x, y);
            let fields = solver.get_surrounding_unrevealed(x, y);

            if needed_mines == 0 {
                reveale.extend(fields);
            } else if needed_mines == fields.len() as u8 {
                flag.extend(fields);
            }
        }
    }

    (reveale, flag)
}