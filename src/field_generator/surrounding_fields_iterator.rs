pub struct SurroundingFieldsIterator {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub range: u8,
    pub dx: i8,
    pub dy: i8,
}

impl Iterator for SurroundingFieldsIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        while self.dy <= self.range as i8 {
            let new_x: i32 = self.x as i32 + self.dx as i32;
            let new_y: i32 = self.y as i32 + self.dy as i32;
            if new_x < 0 || new_y < 0 || new_x >= self.width as i32 || new_y >= self.height as i32 {
                self.increment();
                continue;
            }

            let nx = new_x as u32;
            let ny = new_y as u32;

            self.increment();

            if !(nx == self.x && ny == self.y) {
                return Some((nx, ny));
            }
        }
        None
    }
}

impl SurroundingFieldsIterator {
    fn increment(&mut self) {
        self.dx += 1;
        if self.dx > self.range as i8 {
            self.dx = -(self.range as i8);
            self.dy += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_field_range_1() {
        let iter = SurroundingFieldsIterator {
            x: 1,
            y: 1,
            width: 3,
            height: 3,
            range: 1,
            dx: -1,
            dy: -1,
        };
        
        let mut result: Vec<_> = iter.collect();
        result.sort();
        
        let expected = vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2)
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_corner_field_top_left() {
        let iter = SurroundingFieldsIterator {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
            range: 1,
            dx: -1,
            dy: -1,
        };

        let mut result: Vec<_> = iter.collect();
        result.sort();

        let expected = vec![(0, 1), (1, 0), (1, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_corner_field_bottom_right() {
        let iter = SurroundingFieldsIterator {
            x: 2,
            y: 2,
            width: 3,
            height: 3,
            range: 1,
            dx: -1,
            dy: -1,
        };

        let mut result: Vec<_> = iter.collect();
        result.sort();

        let expected = vec![(1, 1), (1, 2), (2, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_field() {
        let iter = SurroundingFieldsIterator {
            x: 1,
            y: 0,
            width: 3,
            height: 3,
            range: 1,
            dx: -1,
            dy: -1,
        };

        let mut result: Vec<_> = iter.collect();
        result.sort();

        let expected = vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_2() {
        let iter = SurroundingFieldsIterator {
            x: 2,
            y: 2,
            width: 5,
            height: 5,
            range: 2,
            dx: -2,
            dy: -2,
        };

        let result: Vec<_> = iter.collect();

        // Should include all fields within 2 squares of (2,2) except (2,2) itself
        assert!(result.contains(&(0, 0)));
        assert!(result.contains(&(4, 4)));
        assert!(result.contains(&(1, 1)));
        assert!(result.contains(&(3, 3)));
        assert!(!result.contains(&(2, 2))); // Should not include center

        // Count should be 5x5 - 1 = 24 (all fields in 5x5 grid except center)
        assert_eq!(result.len(), 24);
    }

    #[test]
    fn test_single_field_no_neighbors() {
        let iter = SurroundingFieldsIterator {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            range: 1,
            dx: -1,
            dy: -1,
        };

        let result: Vec<_> = iter.collect();
        assert_eq!(result.len(), 0); // No neighbors in 1x1 field
    }

    #[test]
    fn test_out_of_bounds() {
        let iter = SurroundingFieldsIterator {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
            range: 2,
            dx: -2,
            dy: -2,
        };

        let result: Vec<_> = iter.collect();

        // Should only include valid coordinates within bounds
        for (x, y) in &result {
            assert!(*x < 2);
            assert!(*y < 2);
        }

        // Should not include the center field (0,0)
        assert!(!result.contains(&(0, 0)));
    }

    #[test]
    fn test_range_0() {
        let iter = SurroundingFieldsIterator {
            x: 1,
            y: 1,
            width: 3,
            height: 3,
            range: 0,
            dx: 0,
            dy: 0,
        };

        let result: Vec<_> = iter.collect();
        assert_eq!(result.len(), 0); // Range 0 should return no neighbors
    }

    #[test]
    fn test_increment_function() {
        let mut iter = SurroundingFieldsIterator {
            x: 1,
            y: 1,
            width: 3,
            height: 3,
            range: 1,
            dx: -1,
            dy: -1,
        };

        // Test increment behavior
        assert_eq!(iter.dx, -1);
        assert_eq!(iter.dy, -1);

        iter.increment();
        assert_eq!(iter.dx, 0);
        assert_eq!(iter.dy, -1);

        iter.increment();
        assert_eq!(iter.dx, 1);
        assert_eq!(iter.dy, -1);

        iter.increment(); // Should wrap to next row
        assert_eq!(iter.dx, -1);
        assert_eq!(iter.dy, 0);
    }
}
