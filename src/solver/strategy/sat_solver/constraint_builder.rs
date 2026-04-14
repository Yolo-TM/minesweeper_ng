use super::Solver;

/// Represents a constraint on a set of fields
/// Example: A revealed cell showing "2" means exactly 2 of its unrevealed neighbors are mines
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The number of mines that must exist among the fields
    pub mine_count: u32,
    /// The fields (positions) that this constraint applies to
    pub fields: Vec<(u32, u32)>,
}

/// Build all constraints for a given component of border fields
/// Each constraint represents a revealed number cell and the unrevealed fields it touches
pub fn build_constraints(solver: &Solver, component: &[(u32, u32)]) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let component_set: std::collections::HashSet<_> = component.iter().copied().collect();

    // Iterate through all revealed cells with information
    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        // Get unrevealed neighbors that are in this component
        let unrevealed = solver.get_surrounding_unrevealed(x, y);
        let constraint_fields: Vec<_> = unrevealed
            .iter()
            .filter(|f| component_set.contains(f))
            .copied()
            .collect();

        // Only create a constraint if this cell has fields in the component
        if constraint_fields.is_empty() {
            continue;
        }

        // Get the number from the revealed cell and calculate remaining mines
        // (already accounts for flagged neighbors)
        let mine_count = solver.get_reduced_count(x, y) as u32;

        constraints.push(Constraint {
            mine_count,
            fields: constraint_fields,
        });
    }

    constraints
}
