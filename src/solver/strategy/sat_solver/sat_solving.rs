/*
Constraint propagation + backtracking solver for minesweeper components.

For each cell in a component, determines if it is 100% mine, 100% safe,
or undetermined. Uses two SAT-style feasibility queries per cell:
  - Force cell = mine, check if any valid complete assignment exists
  - Force cell = safe, check if any valid complete assignment exists
If only one succeeds, the cell is determined.

Each query uses constraint propagation (arc consistency) to prune the
search tree, with backtracking that stops at the first valid solution.
Rayon parallelizes the per-cell queries across threads.
*/

use super::constraint_builder::Constraint;
use crate::solver::Finding;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve_component(
    component: &[(u32, u32)],
    constraints: &[Constraint],
    remaining_mines: u32,
) -> Finding {
    let mut finding = Finding::new();

    if component.is_empty() || constraints.is_empty() {
        return finding;
    }

    let field_indices: HashMap<(u32, u32), usize> = component
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect();

    let results: Vec<(usize, Option<bool>)> = (0..component.len())
        .into_par_iter()
        .map(|idx| {
            let can_be_mine =
                can_be_value(idx, true, component.len(), constraints, &field_indices, remaining_mines);
            let can_be_safe =
                can_be_value(idx, false, component.len(), constraints, &field_indices, remaining_mines);

            let determination = match (can_be_mine, can_be_safe) {
                (true, false) => Some(true),
                (false, true) => Some(false),
                _ => None,
            };

            (idx, determination)
        })
        .collect();

    for (idx, determination) in results {
        if let Some(is_mine) = determination {
            if is_mine {
                finding.add_mine_field(component[idx]);
            } else {
                finding.add_safe_field(component[idx]);
            }
        }
    }

    finding
}

fn can_be_value(
    forced_index: usize,
    forced_value: bool,
    component_len: usize,
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    remaining_mines: u32,
) -> bool {
    let mut assignment = vec![None; component_len];
    assignment[forced_index] = Some(forced_value);

    if !propagate(&mut assignment, constraints, field_indices, remaining_mines) {
        return false;
    }

    backtrack_search(&mut assignment, constraints, field_indices, remaining_mines)
}

fn propagate(
    assignment: &mut [Option<bool>],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    remaining_mines: u32,
) -> bool {
    let mut changed = true;

    while changed {
        changed = false;

        // Global mine count check
        let total_mines = assignment.iter().filter(|a| **a == Some(true)).count() as u32;
        if total_mines > remaining_mines {
            return false;
        }

        let total_unknowns = assignment.iter().filter(|a| a.is_none()).count() as u32;
        if total_mines == remaining_mines && total_unknowns > 0 {
            // All remaining unknowns must be safe
            for cell in assignment.iter_mut() {
                if cell.is_none() {
                    *cell = Some(false);
                    changed = true;
                }
            }
        }

        for constraint in constraints {
            let mut mines = 0u32;
            let mut unknowns = Vec::new();

            for &field_pos in &constraint.fields {
                if let Some(&idx) = field_indices.get(&field_pos) {
                    match assignment[idx] {
                        Some(true) => mines += 1,
                        Some(false) => {}
                        None => unknowns.push(idx),
                    }
                }
            }

            // Contradiction: too many mines
            if mines > constraint.mine_count {
                return false;
            }

            let remaining_needed = constraint.mine_count - mines;
            if remaining_needed > unknowns.len() as u32 {
                return false;
            }

            // All unknowns must be mines
            if remaining_needed == unknowns.len() as u32 && !unknowns.is_empty() {
                for idx in unknowns {
                    assignment[idx] = Some(true);
                    changed = true;
                }
            }
            // No more mines needed — all unknowns are safe
            else if remaining_needed == 0 && !unknowns.is_empty() {
                for idx in unknowns {
                    assignment[idx] = Some(false);
                    changed = true;
                }
            }
        }
    }

    true
}

fn backtrack_search(
    assignment: &mut Vec<Option<bool>>,
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    remaining_mines: u32,
) -> bool {
    let position = match assignment.iter().position(|a| a.is_none()) {
        Some(p) => p,
        None => return is_valid_solution(assignment, constraints, field_indices, remaining_mines),
    };

    for value in [false, true] {
        let mut branch = assignment.clone();
        branch[position] = Some(value);

        if propagate(&mut branch, constraints, field_indices, remaining_mines)
            && backtrack_search(&mut branch, constraints, field_indices, remaining_mines)
        {
            return true;
        }
    }

    false
}

fn is_valid_solution(
    assignment: &[Option<bool>],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    remaining_mines: u32,
) -> bool {
    let mine_count = assignment.iter().filter(|a| **a == Some(true)).count() as u32;
    if mine_count > remaining_mines {
        return false;
    }

    for constraint in constraints {
        let mines: u32 = constraint
            .fields
            .iter()
            .filter_map(|pos| field_indices.get(pos))
            .filter(|&&idx| assignment[idx] == Some(true))
            .count() as u32;

        if mines != constraint.mine_count {
            return false;
        }
    }

    true
}
