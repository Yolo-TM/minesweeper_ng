mod components;
mod constraint_builder;
mod permutation_checker;

#[cfg(test)]
mod tests;

use super::{Finding, Solver};
pub use components::find_independent_components;
use constraint_builder::build_constraints;
use permutation_checker::solve_component;

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    let components = find_independent_components(solver);

    solver.println(
        &format!("Found {} independent component(s)", components.len()),
        5,
    );

    for (i, component) in components.iter().enumerate() {
        solver.println(
            &format!("  Component {}: {} fields", i + 1, component.len()),
            5,
        );

        // Build constraints for this component
        let constraints = build_constraints(solver, &component);

        solver.println(
            &format!(
                "  Component {} has {} constraints",
                i + 1,
                constraints.len()
            ),
            6,
        );

        // Solve this component
        let component_finding = solve_component(&component, &constraints);

        solver.println(
            &format!(
                "  Component {} found {} mines, {} safe",
                i + 1,
                component_finding.get_mine_fields().len(),
                component_finding.get_safe_fields().len()
            ),
            6,
        );

        // Merge findings
        finding.add_mine_fields(component_finding.get_mine_fields().clone());
        finding.add_safe_fields(component_finding.get_safe_fields().clone());
    }

    finding
}
