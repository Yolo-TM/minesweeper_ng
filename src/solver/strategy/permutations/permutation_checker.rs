use super::constraint_builder::Constraint;
use crate::solver::Finding;
use std::collections::HashMap;

/// Solve a single component using backtracking permutation checking
/// Returns findings (certain mines/safe cells) that are valid in ALL solutions
pub fn solve_component(component: &[(u32, u32)], constraints: &[Constraint]) -> Finding {
    let mut finding = Finding::new();

    if component.is_empty() || constraints.is_empty() {
        return finding;
    }

    let field_indices: HashMap<(u32, u32), usize> = component
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect();

    let mut assignment = vec![None; component.len()];

    let mut field_certainties: Vec<Option<bool>> = vec![None; component.len()];
    let mut first_solution = true;
    let mut solution_count = 0;

    backtrack(
        &mut assignment,
        0,
        constraints,
        &field_indices,
        &mut field_certainties,
        &mut first_solution,
        &mut solution_count,
    );

    if solution_count == 0 {
        return finding;
    }

    // Extract certainties
    for (idx, &pos) in component.iter().enumerate() {
        if let Some(value) = field_certainties[idx] {
            if value {
                finding.add_mine_field(pos);
            } else {
                finding.add_safe_field(pos);
            }
        }
    }

    finding
}

/// Recursive backtracking solver
/// Tracks certainties on-the-fly without storing all solutions
fn backtrack(
    assignment: &mut Vec<Option<bool>>,
    position: usize,
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    field_certainties: &mut Vec<Option<bool>>,
    first_solution: &mut bool,
    solution_count: &mut usize,
) -> bool {
    // Base case: all fields assigned
    if position >= assignment.len() {
        let complete: Vec<bool> = assignment.iter().map(|&x| x.unwrap()).collect();
        if is_valid_solution(&complete, constraints, field_indices) {
            *solution_count += 1;

            if *first_solution {
                // First solution: initialize certainties with this solution
                for (idx, &value) in complete.iter().enumerate() {
                    field_certainties[idx] = Some(value);
                }
                *first_solution = false;
            } else {
                // Subsequent solutions: mark fields as uncertain if they differ
                for (idx, &value) in complete.iter().enumerate() {
                    if let Some(prev) = field_certainties[idx] {
                        if prev != value {
                            field_certainties[idx] = None;
                        }
                    }
                }
            }
        }
        return false;
    }

    // Early pruning
    if !is_valid_partial(assignment, constraints, field_indices) {
        return false;
    }

    // Try safe
    assignment[position] = Some(false);
    backtrack(
        assignment,
        position + 1,
        constraints,
        field_indices,
        field_certainties,
        first_solution,
        solution_count,
    );

    // Try mine
    assignment[position] = Some(true);
    backtrack(
        assignment,
        position + 1,
        constraints,
        field_indices,
        field_certainties,
        first_solution,
        solution_count,
    );

    // Backtrack
    assignment[position] = None;
    false
}

/// Check if current partial assignment is still valid
/// This is used for early pruning during backtracking
fn is_valid_partial(
    assignment: &[Option<bool>],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
) -> bool {
    for constraint in constraints {
        let mut mines_count = 0;
        let mut _safe_count = 0;
        let mut unknown_count = 0;

        for &field_pos in &constraint.fields {
            if let Some(&idx) = field_indices.get(&field_pos) {
                match assignment[idx] {
                    Some(true) => mines_count += 1,
                    Some(false) => _safe_count += 1,
                    None => unknown_count += 1,
                }
            }
        }

        // Too many mines already assigned
        if mines_count > constraint.mine_count {
            return false;
        }

        // Not enough fields left to satisfy mine requirement
        if mines_count + unknown_count < constraint.mine_count {
            return false;
        }
    }

    true
}

/// Check if a complete assignment satisfies all constraints
fn is_valid_solution(
    assignment: &[bool],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
) -> bool {
    for constraint in constraints {
        let mut mines_count = 0;

        for &field_pos in &constraint.fields {
            if let Some(&idx) = field_indices.get(&field_pos) {
                if assignment[idx] {
                    mines_count += 1;
                }
            }
        }

        if mines_count != constraint.mine_count {
            return false;
        }
    }

    true
}
