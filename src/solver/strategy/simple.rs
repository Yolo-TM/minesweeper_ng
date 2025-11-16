use super::{Finding, Solver};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DefinedField, MineSweeperField};

    #[test]
    fn test_1_1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

        let finding = solve(&solver);

        // The 1-1 pattern should be solvable with reduction
        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_1_2_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-2.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

        let finding = solve(&solver);

        // The 1-2 pattern should be solvable with reduction
        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_1_2_1_r_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-2-1-R.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_1_3_1_1_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/1-3-1-1.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

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
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }

    #[test]
    fn test_h3_pattern() {
        let field = DefinedField::from_file("src/generated/patterns/h3.minesweeper")
            .expect("Failed to load pattern file");

        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(field.get_start_cell().0, field.get_start_cell().1);

        let finding = solve(&solver);

        assert!(
            finding.get_mine_fields().len() == 0 && finding.get_safe_fields().len() == 0,
            "Shouldn't find some certain fields in 1-3-1-1 pattern"
        );
    }
}
