pub struct MineSweeperFieldIterator {
    pub width: u32,
    pub height: u32,
    pub current_x: u32,
    pub current_y: u32,
}

impl Iterator for MineSweeperFieldIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.height {
            return None;
        }

        let coord = (self.current_x, self.current_y);

        self.current_x += 1;
        if self.current_x >= self.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_field_iterator() {
        let mut iter = MineSweeperFieldIterator {
            width: 0,
            height: 0,
            current_x: 0,
            current_y: 0,
        };

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_cell_iterator() {
        let mut iter = MineSweeperFieldIterator {
            width: 1,
            height: 1,
            current_x: 0,
            current_y: 0,
        };

        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_row_iterator() {
        let iter = MineSweeperFieldIterator {
            width: 3,
            height: 1,
            current_x: 0,
            current_y: 0,
        };

        let expected = vec![(0, 0), (1, 0), (2, 0)];
        let actual: Vec<_> = iter.collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_column_iterator() {
        let iter = MineSweeperFieldIterator {
            width: 1,
            height: 3,
            current_x: 0,
            current_y: 0,
        };

        let expected = vec![(0, 0), (0, 1), (0, 2)];
        let actual: Vec<_> = iter.collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rectangular_field_iterator() {
        let iter = MineSweeperFieldIterator {
            width: 3,
            height: 2,
            current_x: 0,
            current_y: 0,
        };

        let expected = vec![
            (0, 0), (1, 0), (2, 0),
            (0, 1), (1, 1), (2, 1)
        ];
        let actual: Vec<_> = iter.collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_iterator_order() {
        // Test that iterator goes row by row, left to right
        let mut iter = MineSweeperFieldIterator {
            width: 2,
            height: 2,
            current_x: 0,
            current_y: 0,
        };

        assert_eq!(iter.next(), Some((0, 0))); // First row, first column
        assert_eq!(iter.next(), Some((1, 0))); // First row, second column
        assert_eq!(iter.next(), Some((0, 1))); // Second row, first column
        assert_eq!(iter.next(), Some((1, 1))); // Second row, second column
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_count() {
        let iter = MineSweeperFieldIterator {
            width: 5,
            height: 4,
            current_x: 0,
            current_y: 0,
        };

        let count = iter.count();
        assert_eq!(count, 20); // 5 * 4 = 20
    }

    #[test]
    fn test_partial_iteration() {
        let iter = MineSweeperFieldIterator {
            width: 3,
            height: 3,
            current_x: 0,
            current_y: 0,
        };

        // Take only first 5 elements
        let partial: Vec<_> = iter.take(5).collect();
        assert_eq!(partial, vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1)]);
    }
}
