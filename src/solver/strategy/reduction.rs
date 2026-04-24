use super::{Finding, Solver};

/*
Reduction strategy:

For each revealed Number cell with neighbouring hidden cells:
- Compare it to its revealed neighbouring cells and check for shared hidden cells.
- If all hidden cells of the first cell are also hidden cells of the second cell:
    - If both have the same reduced mine count, all unique hidden cells of the second cell are safe to reveal.
    - If the second cell has more mines than the first, and the difference equals the number of unique hidden cells of the second cell, those unique cells are mines.
- If the first cell has more mines than the second, and the difference equals the number of unique hidden cells of the first cell, those unique cells are mines.
*/

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }
        let reduced_count = solver.get_reduced_count(x, y);
        let hidden_fields = solver.get_surrounding_unrevealed(x, y);

        for (new_x, new_y) in solver.surrounding_fields(x, y, Some(3)) {
            if !solver.has_informations(new_x, new_y) {
                continue;
            }

            let reduced_count2 = solver.get_reduced_count(new_x, new_y);
            let hidden_fields2 = solver.get_surrounding_unrevealed(new_x, new_y);

            // Partition fields into shared and unique to second cell
            let (shared_fields, unique_to_second): (Vec<_>, Vec<_>) = hidden_fields2
                .iter()
                .partition(|cell| hidden_fields.contains(cell));

            // Case 1: All hidden fields of first cell are shared with second cell
            if hidden_fields.len() == shared_fields.len() {
                // Found two numbers which share the same unrevealed fields.
                // Now we can check if we can solve other neighbouring fields with this extra information

                if reduced_count == reduced_count2 {
                    // Same mine count in shared area → unique fields are safe
                    finding.add_safe_fields(unique_to_second);
                } else {
                    let reduced_diff = reduced_count2 - reduced_count;
                    let unique_count = solver.get_surrounding_unrevealed_count(new_x, new_y)
                        - shared_fields.len() as u8;

                    if reduced_diff == unique_count {
                        // All unique fields must be mines
                        finding.add_mine_fields(unique_to_second);
                    }
                }
            }
            // Case 2: First cell has more mines than second
            else if reduced_count > reduced_count2 {
                // Fields unique to first cell
                let reduced_diff = (reduced_count - reduced_count2) as usize;
                let unique_to_first: Vec<_> = hidden_fields
                    .iter()
                    .filter(|cell| !shared_fields.contains(cell))
                    .copied()
                    .collect();

                if reduced_diff == unique_to_first.len() {
                    // All unique fields of first cell must be mines
                    finding.add_mine_fields(unique_to_first);
                }
            }
        }
    }

    finding
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DefinedField;

    #[test]
    fn test_1_1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        // The 1-1 pattern should be solvable with reduction
        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in 1-1 pattern"
        );
    }

    #[test]
    fn test_1_2_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-2.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        // The 1-2 pattern should be solvable with reduction
        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in 1-2 pattern"
        );
    }

    #[test]
    fn test_1_2_1_r_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-2-1-R.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        // The 1-2-1-R pattern should be solvable with reduction
        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in 1-2-1-R pattern"
        );
    }

    #[test]
    fn test_1_2_2_1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-2-2-1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in 1-2-2-1 pattern"
        );
    }

    #[test]
    fn test_1_3_1_1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-3-1-1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_1_3_1_2_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-3-1-2.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_1_3_1_3_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-3-1-3.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_2_2_2_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/2-2-2.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_b1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/b1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in b1 pattern"
        );
    }

    #[test]
    fn test_h2_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/h2.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in h2 pattern"
        );
    }

    #[test]
    fn test_h3_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/h3.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.open_start_cell();

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() > 0 || finding.get_safe_fields().len() > 0,
            "Should find some certain fields in h3 pattern"
        );
    }
}
