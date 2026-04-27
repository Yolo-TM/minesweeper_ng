use super::components::find_independent_components;
use super::constraint_builder::build_constraints;
use super::sat_solving::solve_component;
use super::{Finding, Solver};
use log::trace;

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    let components = find_independent_components(solver);

    trace!("Found {} independent component(s)", components.len());

    for (i, component) in components.iter().enumerate() {
        trace!("  Component {}: {} fields", i + 1, component.len());

        let constraints = build_constraints(solver, &component);

        trace!(
            "  Component {} has {} constraints",
            i + 1,
            constraints.len()
        );

        let remaining_mines = solver.get_remaining_mines();
        let component_finding = solve_component(&component, &constraints, remaining_mines);

        trace!(
            "  Component {} found {} mines, {} safe",
            i + 1,
            component_finding.get_mine_fields().len(),
            component_finding.get_safe_fields().len()
        );

        finding.add_mine_fields(component_finding.get_mine_fields().clone());
        finding.add_safe_fields(component_finding.get_safe_fields().clone());
    }

    finding
}
