use std::cmp::Ordering;

pub fn sort_by_min_distance(permutation_vector: &mut Vec<((u32, u32), bool)>) {
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
fn distance(a: (u32, u32), b: (u32, u32)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        // Test distance between same points
        assert_eq!(distance((0, 0), (0, 0)), 0.0);

        // Test distance between horizontal neighbors
        assert_eq!(distance((0, 0), (1, 0)), 1.0);
        assert_eq!(distance((0, 0), (3, 0)), 3.0);

        // Test distance between vertical neighbors
        assert_eq!(distance((0, 0), (0, 1)), 1.0);
        assert_eq!(distance((0, 0), (0, 4)), 4.0);

        // Test diagonal distances
        assert_eq!(distance((0, 0), (1, 1)), (2.0_f64).sqrt());
        assert_eq!(distance((0, 0), (3, 4)), 5.0); // 3-4-5 triangle

        // Test symmetric distance
        assert_eq!(distance((1, 2), (4, 6)), distance((4, 6), (1, 2)));
    }

    #[test]
    fn test_sort_empty_vector() {
        let mut empty_vec: Vec<((u32, u32), bool)> = vec![];
        sort_by_min_distance(&mut empty_vec);
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn test_sort_single_element() {
        let mut single_vec = vec![((5, 3), false)];
        sort_by_min_distance(&mut single_vec);

        assert_eq!(single_vec.len(), 1);
        assert_eq!(single_vec[0], ((5, 3), false));
    }

    #[test]
    fn test_sort_two_elements() {
        let mut two_vec = vec![((5, 5), true), ((0, 0), false)];
        sort_by_min_distance(&mut two_vec);

        assert_eq!(two_vec.len(), 2);
        // Should start with (0,0) as it has smaller coordinate sum
        assert_eq!(two_vec[0], ((0, 0), false));
        assert_eq!(two_vec[1], ((5, 5), true));
    }

    #[test]
    fn test_sort_coordinates_by_sum() {
        let mut vec = vec![
            ((3, 4), false), // sum = 7
            ((1, 1), true),  // sum = 2 (should be first)
            ((2, 3), false), // sum = 5
            ((0, 6), true),  // sum = 6
        ];

        sort_by_min_distance(&mut vec);

        // Should start with (1,1) as it has the smallest sum
        assert_eq!(vec[0], ((1, 1), true));
    }

    #[test]
    fn test_sort_same_sum_prefer_smaller_x() {
        let mut vec = vec![
            ((3, 2), false), // sum = 5, x = 3
            ((1, 4), true),  // sum = 5, x = 1 (should be first)
            ((2, 3), false), // sum = 5, x = 2
        ];

        sort_by_min_distance(&mut vec);

        // Should start with (1,4) as it has the smallest x for sum = 5
        assert_eq!(vec[0], ((1, 4), true));
    }

    #[test]
    fn test_sort_nearest_neighbor_order() {
        let mut vec = vec![
            ((0, 0), false),
            ((10, 10), true),
            ((1, 1), false),
            ((2, 2), true),
        ];

        sort_by_min_distance(&mut vec);

        // Should start with (0,0) as it has smallest sum
        assert_eq!(vec[0], ((0, 0), false));
        // Next should be (1,1) as it's closest to (0,0)
        assert_eq!(vec[1], ((1, 1), false));
        // Then (2,2) as it's closest to (1,1)
        assert_eq!(vec[2], ((2, 2), true));
        // Finally (10,10)
        assert_eq!(vec[3], ((10, 10), true));
    }

    #[test]
    fn test_sort_complex_pattern() {
        let mut vec = vec![
            ((0, 0), false),
            ((5, 5), true),
            ((1, 0), false),
            ((0, 1), true),
            ((2, 0), false),
        ];

        sort_by_min_distance(&mut vec);

        // Should start with (0,0)
        assert_eq!(vec[0], ((0, 0), false));
          // Verify that each subsequent point is reasonably close to the previous
        for i in 1..vec.len() {
            let prev_pos = vec[i-1].0;
            let curr_pos = vec[i].0;
            let dist = distance(prev_pos, curr_pos);
            
            // Each step should be relatively small (allowing for larger field traversal)
            assert!(dist <= 8.0, "Large jump from {:?} to {:?}", prev_pos, curr_pos);
        }
    }

    #[test]
    fn test_sort_preserves_boolean_values() {
        let mut vec = vec![
            ((0, 0), true),
            ((1, 1), false),
            ((2, 2), true),
        ];

        let original_bools: Vec<bool> = vec.iter().map(|(_, b)| *b).collect();
        sort_by_min_distance(&mut vec);
        let sorted_bools: Vec<bool> = vec.iter().map(|(_, b)| *b).collect();

        // The boolean values should be preserved, just in different order
        assert_eq!(original_bools.len(), sorted_bools.len());
        for bool_val in original_bools {
            assert!(sorted_bools.contains(&bool_val));
        }
    }

    #[test]
    fn test_sort_preserves_all_coordinates() {
        let mut vec = vec![
            ((3, 7), false),
            ((1, 2), true),
            ((5, 1), false),
            ((0, 4), true),
        ];

        let original_coords: Vec<(u32, u32)> = vec.iter().map(|(pos, _)| *pos).collect();
        sort_by_min_distance(&mut vec);
        let sorted_coords: Vec<(u32, u32)> = vec.iter().map(|(pos, _)| *pos).collect();

        // All coordinates should be preserved
        assert_eq!(original_coords.len(), sorted_coords.len());
        for coord in original_coords {
            assert!(sorted_coords.contains(&coord));
        }
    }

    #[test]
    fn test_sort_grid_pattern() {
        // Test a 2x2 grid starting from different corners
        let mut vec = vec![
            ((1, 1), false), // Center-ish
            ((0, 0), true),  // Top-left
            ((1, 0), false), // Top-right
            ((0, 1), true),  // Bottom-left
        ];

        sort_by_min_distance(&mut vec);

        // Should start with (0,0) due to smallest sum
        assert_eq!(vec[0], ((0, 0), true));

        // Verify the path makes sense (no large jumps)
        for i in 1..vec.len() {
            let dist = distance(vec[i-1].0, vec[i].0);
            assert!(dist <= 2.0, "Unexpected large distance in grid traversal");
        }
    }
}
