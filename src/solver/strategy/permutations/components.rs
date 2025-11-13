use crate::solver::Solver;
use std::collections::{HashMap, HashSet};

/// Find all independent components of border fields
/// Each component contains fields that are mutually dependent through shared constraints
/// Fields in different components have no shared constraints and can be solved independently
pub fn find_independent_components(solver: &Solver) -> Vec<Vec<(u32, u32)>> {
    // Step 1: Collect all border fields (unrevealed fields adjacent to revealed numbers)
    let mut all_border_fields: HashSet<(u32, u32)> = HashSet::new();

    for (x, y) in solver.sorted_fields() {
        if solver.has_informations(x, y) {
            let unrevealed = solver.get_surrounding_unrevealed(x, y);
            for field in unrevealed {
                all_border_fields.insert(field);
            }
        }
    }

    if all_border_fields.is_empty() {
        return vec![];
    }

    // Step 2: Build adjacency graph
    // Two border fields are adjacent if they share at least one constraint (revealed number)
    // This ensures all fields affected by the same constraint stay in the same component
    let mut adjacency: HashMap<(u32, u32), HashSet<(u32, u32)>> = HashMap::new();

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        // Get all unrevealed neighbors that are border fields
        let unrevealed = solver.get_surrounding_unrevealed(x, y);
        let constraint_fields: Vec<_> = unrevealed
            .iter()
            .filter(|f| all_border_fields.contains(f))
            .copied()
            .collect();

        // Connect all pairs of fields in this constraint
        // They must be in the same component since they share this revealed number's mine count
        for &field1 in &constraint_fields {
            for &field2 in &constraint_fields {
                if field1 != field2 {
                    adjacency
                        .entry(field1)
                        .or_insert_with(HashSet::new)
                        .insert(field2);
                }
            }
        }
    }

    // Step 3: Find connected components using Depth-First Search (DFS)
    let mut components: Vec<Vec<(u32, u32)>> = Vec::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    for &start_field in &all_border_fields {
        if visited.contains(&start_field) {
            continue;
        }

        let mut component: Vec<(u32, u32)> = Vec::new();
        let mut stack: Vec<(u32, u32)> = vec![start_field];

        while let Some(field) = stack.pop() {
            if visited.contains(&field) {
                continue;
            }

            visited.insert(field);
            component.push(field);

            // Add all adjacent fields to the stack
            if let Some(neighbors) = adjacency.get(&field) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        components.push(component);
    }

    components
}
