use std::cmp::Ordering;

pub fn sort_by_min_distance(permutation_vector: &mut Vec<((usize, usize), bool)>) {
    if permutation_vector.is_empty() {
        return;
    }

    // Use the smallest x, y combination as the starting point
    let (mut start_x, mut start_y) = permutation_vector[0].0;
    for i in 1..permutation_vector.len() {
        let sum = start_x + start_y;
        let (x, y) = permutation_vector[i].0;
        let sum2 = x + y;

        if sum2 < sum {
            start_x = x;
            start_y = y;
        } else if sum2 == sum && x < start_x {
            start_x = x;
            start_y = y;
        }
    }

    // remove the starting point from the permutation vector
    let start_index = permutation_vector.iter().position(|&x| x.0 == (start_x, start_y)).unwrap();
    let start_point = permutation_vector.remove(start_index);

    let mut sorted_vector = vec![start_point];


    while !permutation_vector.is_empty() {
        // Find the closest point to the last point in the sorted vector
        let last_point = sorted_vector.last().unwrap().0;
        let (index, _) = permutation_vector
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = distance(last_point, a.0);
                let dist_b = distance(last_point, b.0);
                dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal)
            })
            .unwrap();

        // Add the closest point to the sorted vector and remove it from the original vector
        sorted_vector.push(permutation_vector.remove(index));
    }

    // Replace the original vector with the sorted one
    *permutation_vector = sorted_vector;
}

// Helper function to calculate the Euclidean distance between two points
fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    (dx * dx + dy * dy).sqrt()
}
