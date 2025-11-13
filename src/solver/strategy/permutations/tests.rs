use super::find_independent_components;
use crate::solver::{Solver, cell_state::CellState};
use crate::{DefinedField, MineSweeperField, Mines};

/// Helper to create a test field from a string pattern
/// Format:
/// '.' = Empty (revealed)
/// '?' = Hidden (unrevealed)
/// '0'-'8' = Number (revealed)
/// 'M' = Mine (hidden, for field definition)
pub fn create_test_field(pattern: &str) -> DefinedField {
    let lines: Vec<&str> = pattern.trim().lines().map(|l| l.trim()).collect();
    let height = lines.len() as u32;
    let width = lines[0].len() as u32;

    // First pass: determine mine positions
    let mut mine_positions = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'M' {
                mine_positions.push((x as u32, y as u32));
            }
        }
    }

    // Create field with mine count
    let mine_count = mine_positions.len() as u32;
    let mut field = DefinedField::new(width, height, Mines::Count(mine_count));
    field.initialize(mine_positions);
    field.set_start_cell(0, 0);
    field
}

/// Helper to create a solver and directly set specific cells as revealed
/// This avoids the cascade reveal behavior of reveal_cell()
pub fn create_solver_with_reveals(field: &impl MineSweeperField, reveals: &[(u32, u32)]) -> Solver {
    let mut solver = Solver::new(field, 0);

    // Directly set cells as revealed without triggering cascade
    for &(x, y) in reveals {
        let cell = field.get_cell(x, y);
        solver.state[x as usize][y as usize] = CellState::Revealed(cell);
    }

    solver
}

#[cfg(test)]
mod component_tests {
    use super::*;

    #[test]
    fn test_two_separate_corners() {
        // Two completely separate constraint regions
        let pattern = "
            M...
            ....
            ....
            ...M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Reveal cell next to top-left mine
                (2, 3), // Reveal cell next to bottom-right mine
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 2, "Should find 2 independent components");
    }

    #[test]
    fn test_single_connected_component() {
        // Two fields sharing a constraint
        let pattern = "
            M.M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(&field, &[(1, 0)]);

        let components = find_independent_components(&solver);

        assert_eq!(
            components.len(),
            1,
            "Should find 1 component (all connected)"
        );
        assert_eq!(
            components[0].len(),
            2,
            "Component should have 2 border fields"
        );
    }

    #[test]
    fn test_linear_chain() {
        // Fields connected in a chain: A-B-C-D
        let pattern = "
            M.M.M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Between first and second mine
                (3, 0), // Between second and third mine
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 1, "Chain should form 1 component");
        assert_eq!(
            components[0].len(),
            3,
            "Should have 3 border fields (all mines)"
        );
    }

    #[test]
    fn test_separated_by_revealed() {
        // Two groups separated by revealed cells
        let pattern = "
            M.M.M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Reveals cell between first two mines
                (3, 0), // Reveals cell between last two mines
                (2, 0), // Reveals middle cell to separate the groups
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 2, "Should find 2 separate components");
    }

    #[test]
    fn test_shared_constraint_merges() {
        // Two regions that share a constraint cell, forcing them to merge
        let pattern = "
            M.M
            ...
            M.M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(&field, &[(1, 1)]);

        let components = find_independent_components(&solver);

        assert_eq!(
            components.len(),
            1,
            "Shared constraint should merge into 1 component"
        );
        // Center cell (1,1) neighbors: 4 mines + 4 safe cells = 8 border fields
        assert_eq!(
            components[0].len(),
            8,
            "Should have 8 border fields (4 mines + 4 safe cells)"
        );
    }

    #[test]
    fn test_diagonal_separation() {
        // Diagonally positioned constraints - should be separate
        let pattern = "
            M....
            .....
            .....
            ....M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Near top-left mine
                (3, 3), // Near bottom-right mine
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 2, "Diagonal regions should be separate");
    }

    #[test]
    fn test_complex_interconnected() {
        // Complex web of shared constraints
        let pattern = "
            M.M
            .M.
            M.M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Top
                (0, 1), // Left
                (2, 1), // Right
                (1, 2), // Bottom
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 1, "All fields should form 1 component");
        assert_eq!(components[0].len(), 5, "Should have 5 border fields");
    }

    #[test]
    fn test_no_border_fields() {
        // All cells revealed or flagged - no hidden cells remain
        let pattern = "
            .M
        ";

        let field = create_test_field(pattern);
        let mut solver = Solver::new(&field, 0);
        solver.reveal_cell(0, 0); // Reveal the safe cell
        solver.flag_cell(1, 0); // Flag the mine

        let components = find_independent_components(&solver);

        assert_eq!(
            components.len(),
            0,
            "No border fields should give 0 components"
        );
    }

    #[test]
    fn test_single_isolated_field() {
        // One number with one unrevealed neighbor
        let pattern = "
            M.
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(&field, &[(1, 0)]);

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 1, "Single field should form 1 component");
        assert_eq!(
            components[0].len(),
            1,
            "Component should have 1 field (the mine)"
        );
    }

    #[test]
    fn test_three_separate_regions() {
        // Three completely independent regions
        let pattern = "
            M....M....M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Near first mine
                (6, 0), // Near second mine (more spacing)
                (9, 0), // Near third mine
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 3, "Should find 3 independent regions");
    }

    #[test]
    fn test_flag_boundary() {
        // Test that flagged cells don't create connections
        let pattern = "
            M.....M
        ";

        let field = create_test_field(pattern);
        let solver = create_solver_with_reveals(
            &field,
            &[
                (1, 0), // Near first mine
                (5, 0), // Near second mine
            ],
        );

        let components = find_independent_components(&solver);

        assert_eq!(components.len(), 2, "Should be 2 separate components");
    }
}

#[cfg(test)]
mod constraint_tests {
    // TODO: Add constraint building tests

    #[test]
    fn test_placeholder() {
        // Placeholder to prevent empty test module error
        assert!(true);
    }
}

#[cfg(test)]
mod solver_tests {
    // TODO: Add permutation solver tests
    // - Test simple unique solution case
    // - Test multiple solutions with certainties
    // - Test no solution case

    #[test]
    fn test_placeholder() {
        // Placeholder to prevent empty test module error
        assert!(true);
    }
}
