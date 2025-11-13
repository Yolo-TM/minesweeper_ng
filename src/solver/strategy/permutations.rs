use super::{Solver, Finding};

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
    let mut fields: Vec<(u32, u32)> = Vec::new();

    for (x, y) in solver.sorted_fields() {
        if solver.has_informations(x, y) {
            let unrevealed = solver.get_surrounding_unrevealed(x, y);
            for field in unrevealed {
                if !fields.contains(&field) {
                    fields.push(field);
                }
            }
        }
    }

    fields
}

fn start_recursion(solver: &Solver, fields: &Vec<(u32, u32)>, finding: &mut Finding) {
    let mut current_assignment: Vec<bool> = vec![false; fields.len()];
    let mut solutions: Vec<Vec<bool>> = Vec::new();

    println!("Starting Permutations of 2^{} = {} possibilities", fields.len(), 2u128.pow(fields.len() as u32));

    recurse(solver, fields, 0, &mut current_assignment, &mut solutions);

    for index in 0..fields.len() {
        let mut all_mines = true;
        let mut all_safe = true;

        for solution in &solutions {
            if solution[index] {
                all_safe = false;
            } else {
                all_mines = false;
            }
        }

        let (fx, fy) = fields[index];
        if all_mines {
            finding.add_mine_field((fx, fy));
        } else if all_safe {
            finding.add_safe_field((fx, fy));
        }
    }

}

fn recurse(
    solver: &Solver,
    fields: &Vec<(u32, u32)>,
    index: usize,
    current_assignment: &mut Vec<bool>,
    solutions: &mut Vec<Vec<bool>>,
) {
    if index == fields.len() {
        validate_solution(solver, fields, current_assignment, solutions);
        return;
    }

    current_assignment[index] = false;
    recurse(solver, fields, index + 1, current_assignment, solutions);

    current_assignment[index] = true;
    recurse(solver, fields, index + 1, current_assignment, solutions);
}

fn validate_solution(
    solver: &Solver,
    fields: &Vec<(u32, u32)>,
    current_assignment: &Vec<bool>,
    solutions: &mut Vec<Vec<bool>>,
) {

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        let reduced_count = solver.get_reduced_count(x, y);
        let surrounding_unrevealed = solver.get_surrounding_unrevealed(x, y);

        let mut assigned_mines = 0u8;
        for &(ux, uy) in &surrounding_unrevealed {
            if let Some(index) = fields.iter().position(|&(fx, fy)| fx == ux && fy == uy) {
                if current_assignment[index] {
                    assigned_mines += 1;
                }
            }
        }

        if assigned_mines != reduced_count {
            return;
        }
    }

    solutions.push(current_assignment.clone());
}