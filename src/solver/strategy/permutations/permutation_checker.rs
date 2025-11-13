use super::constraint_builder::Constraint;
use crate::solver::Finding;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

const PARALLEL_THRESHOLD: usize = 18;

fn calculate_split_depth(component_size: usize) -> usize {
    ((component_size as f64) * 0.3).floor() as usize
}

fn calculate_num_threads(num_branches: usize) -> usize {
    let cpu_count = num_cpus::get();
    (cpu_count * 5).min(num_branches)
}

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

    let field_certainties = if component.len() >= PARALLEL_THRESHOLD {
        solve_parallel(component, constraints, &field_indices)
    } else {
        solve_sequential(component, constraints, &field_indices)
    };

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

fn solve_sequential(
    component: &[(u32, u32)],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
) -> Vec<Option<bool>> {
    let mut assignment = vec![None; component.len()];
    let mut field_certainties: Vec<Option<bool>> = vec![None; component.len()];
    let mut first_solution = true;
    let mut solution_count = 0;

    backtrack(
        &mut assignment,
        0,
        constraints,
        field_indices,
        &mut field_certainties,
        &mut first_solution,
        &mut solution_count,
    );

    field_certainties
}

fn solve_parallel(
    component: &[(u32, u32)],
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
) -> Vec<Option<bool>> {
    let split_depth = calculate_split_depth(component.len());
    let num_branches = 1 << split_depth;

    let constraints = Arc::new(constraints.to_vec());
    let field_indices = Arc::new(field_indices.clone());
    let component_len = component.len();

    // Pre-filter valid branches before spawning threads
    let valid_branches: Vec<usize> = (0..num_branches)
        .filter(|&branch_id| {
            let mut assignment = vec![None; component_len];
            for i in 0..split_depth {
                assignment[i] = Some((branch_id >> i) & 1 == 1);
            }
            is_valid_partial(&assignment, &constraints, &field_indices)
        })
        .collect();

    // Calculate threads based on valid branches after pruning
    let num_threads = calculate_num_threads(valid_branches.len());

    eprintln!(
        "[Permutations] Fields {}, Split depth: {}, Valid branches: {}, Threads: {}",
        component.len(),
        split_depth,
        valid_branches.len(),
        num_threads
    );

    // Distribute valid branches across threads
    let valid_per_thread = (valid_branches.len() + num_threads - 1) / num_threads;
    let mut valid_assignments = Vec::new();

    for thread_idx in 0..num_threads {
        let start_idx = thread_idx * valid_per_thread;
        let end_idx = ((thread_idx + 1) * valid_per_thread).min(valid_branches.len());

        if start_idx >= valid_branches.len() {
            break;
        }

        valid_assignments.push((start_idx, end_idx));
    }

    let handles: Vec<_> = valid_assignments
        .into_iter()
        .map(|(start_idx, end_idx)| {
            let constraints = Arc::clone(&constraints);
            let field_indices = Arc::clone(&field_indices);
            let valid_branches = valid_branches.clone();

            thread::spawn(move || {
                let mut local_certainties: Vec<Option<bool>> = vec![None; component_len];
                let mut local_first_solution = true;
                let mut local_solution_count = 0;

                for idx in start_idx..end_idx {
                    let branch_id = valid_branches[idx];
                    let mut assignment = vec![None; component_len];

                    for i in 0..split_depth {
                        assignment[i] = Some((branch_id >> i) & 1 == 1);
                    }

                    backtrack(
                        &mut assignment,
                        split_depth,
                        &constraints,
                        &field_indices,
                        &mut local_certainties,
                        &mut local_first_solution,
                        &mut local_solution_count,
                    );
                }

                (local_certainties, local_solution_count)
            })
        })
        .collect();

    let mut global_certainties: Vec<Option<bool>> = vec![None; component_len];
    let mut global_first = true;

    for handle in handles {
        let (local_certainties, local_count) = handle.join().unwrap();

        if local_count > 0 {
            if global_first {
                global_certainties = local_certainties;
                global_first = false;
            } else {
                for idx in 0..component_len {
                    match (global_certainties[idx], local_certainties[idx]) {
                        (Some(global_val), Some(local_val)) if global_val != local_val => {
                            global_certainties[idx] = None;
                        }
                        (Some(_), None) | (None, _) => {
                            global_certainties[idx] = None;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    global_certainties
}

fn backtrack(
    assignment: &mut Vec<Option<bool>>,
    position: usize,
    constraints: &[Constraint],
    field_indices: &HashMap<(u32, u32), usize>,
    field_certainties: &mut Vec<Option<bool>>,
    first_solution: &mut bool,
    solution_count: &mut usize,
) {
    if position >= assignment.len() {
        // Were done here with this path, return values
        let complete: Vec<bool> = assignment.iter().map(|&x| x.unwrap()).collect();
        if is_valid_solution(&complete, constraints, field_indices) {
            *solution_count += 1;

            if *first_solution {
                for (idx, &value) in complete.iter().enumerate() {
                    field_certainties[idx] = Some(value);
                }
                *first_solution = false;
            } else {
                for (idx, &value) in complete.iter().enumerate() {
                    if let Some(prev) = field_certainties[idx] {
                        if prev != value {
                            field_certainties[idx] = None;
                        }
                    }
                }
            }
        }
        return;
    }

    if !is_valid_partial(assignment, constraints, field_indices) {
        return;
    }

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

    assignment[position] = None;
}

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

        if mines_count > constraint.mine_count {
            return false;
        }

        if mines_count + unknown_count < constraint.mine_count {
            return false;
        }
    }

    true
}

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
