use super::{Solver, Finding};
use std::collections::{HashSet, HashMap};

/*
Permutations Strategy

Get all Border Fields which have revealed neighbours. Iterate over all possible solutions to find safe moves. (Moves which are the same in all valid solutions)
*/

pub fn solve(solver: &Solver) -> Finding {
    let mut finding = Finding::new();

    let fields = sorted_border_fields(solver);
    start_recursion(solver, &fields, &mut finding);

    finding
}

fn sorted_border_fields(solver: &Solver) -> Vec<(u32, u32)> {
    let mut fields_set: HashSet<(u32, u32)> = HashSet::new();

    for (x, y) in solver.sorted_fields() {
        if solver.has_informations(x, y) {
            let unrevealed = solver.get_surrounding_unrevealed(x, y);
            for field in unrevealed {
                fields_set.insert(field);
            }
        }
    }

    fields_set.into_iter().collect()
}

fn start_recursion(solver: &Solver, fields: &Vec<(u32, u32)>, finding: &mut Finding) {
    println!("Starting Permutations of 2^{} = {} possibilities", fields.len(), 2u128.pow(fields.len() as u32));

    // Pre-compute field index mapping for O(1) lookup
    let field_indices: HashMap<(u32, u32), usize> = fields
        .iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect();

    // Pre-compute constraints: for each number cell, which field indices are its neighbors
    let mut constraints: Vec<Constraint> = Vec::new();
    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        let reduced_count = solver.get_reduced_count(x, y);
        let surrounding_unrevealed = solver.get_surrounding_unrevealed(x, y);
        
        let neighbor_indices: Vec<usize> = surrounding_unrevealed
            .iter()
            .filter_map(|pos| field_indices.get(pos).copied())
            .collect();
        
        if !neighbor_indices.is_empty() {
            constraints.push(Constraint {
                neighbor_indices,
                required_mines: reduced_count,
            });
        }
    }

    // Track mine/safe counts across all valid solutions
    let mut mine_counts = vec![0u32; fields.len()];
    let mut safe_counts = vec![0u32; fields.len()];
    let mut total_solutions = 0u32;

    let mut current_assignment = vec![false; fields.len()];
    recurse_optimized(
        &constraints,
        0,
        &mut current_assignment,
        &mut mine_counts,
        &mut safe_counts,
        &mut total_solutions,
    );

    // Determine deterministic fields
    if total_solutions > 0 {
        for index in 0..fields.len() {
            let (fx, fy) = fields[index];
            if mine_counts[index] == total_solutions {
                finding.add_mine_field((fx, fy));
            } else if safe_counts[index] == total_solutions {
                finding.add_safe_field((fx, fy));
            }
        }
    }
}

struct Constraint {
    neighbor_indices: Vec<usize>,
    required_mines: u8,
}

fn recurse_optimized(
    constraints: &[Constraint],
    index: usize,
    current_assignment: &mut Vec<bool>,
    mine_counts: &mut Vec<u32>,
    safe_counts: &mut Vec<u32>,
    total_solutions: &mut u32,
) {
    if index == current_assignment.len() {
        // Validate all constraints
        if is_valid_solution(constraints, current_assignment) {
            *total_solutions += 1;
            for i in 0..current_assignment.len() {
                if current_assignment[i] {
                    mine_counts[i] += 1;
                } else {
                    safe_counts[i] += 1;
                }
            }
        }
        return;
    }

    // Try safe (false) first
    current_assignment[index] = false;
    if can_be_valid(constraints, current_assignment, index) {
        recurse_optimized(constraints, index + 1, current_assignment, mine_counts, safe_counts, total_solutions);
    }

    // Try mine (true)
    current_assignment[index] = true;
    if can_be_valid(constraints, current_assignment, index) {
        recurse_optimized(constraints, index + 1, current_assignment, mine_counts, safe_counts, total_solutions);
    }
}

// Check if current partial assignment can still lead to valid solution
fn can_be_valid(constraints: &[Constraint], current_assignment: &[bool], filled_up_to: usize) -> bool {
    for constraint in constraints {
        let mut assigned_mines = 0u8;
        let mut unassigned_count = 0;
        
        for &idx in &constraint.neighbor_indices {
            if idx <= filled_up_to {
                if current_assignment[idx] {
                    assigned_mines += 1;
                }
            } else {
                unassigned_count += 1;
            }
        }
        
        let required = constraint.required_mines;
        
        // Too many mines already
        if assigned_mines > required {
            return false;
        }
        
        // Not enough positions left to place required mines
        if assigned_mines + unassigned_count < required {
            return false;
        }
    }
    
    true
}

// Final validation when all fields are assigned
fn is_valid_solution(constraints: &[Constraint], current_assignment: &[bool]) -> bool {
    for constraint in constraints {
        let mines = constraint.neighbor_indices
            .iter()
            .filter(|&&idx| current_assignment[idx])
            .count() as u8;
        
        if mines != constraint.required_mines {
            return false;
        }
    }
    
    true
}