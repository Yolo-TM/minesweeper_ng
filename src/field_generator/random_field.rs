use super::{
    MineSweeperCell,
    MineSweeperField,
    MineSweeperFieldCreation,
};

use rand::{
    Rng,
    rngs::StdRng,
    SeedableRng,
};

#[derive(Clone)]
pub struct RandomField {
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),
    board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField for RandomField {
    #[track_caller]
    fn new(width: u32, height: u32, mines: MineSweeperFieldCreation) -> Self {
        let percentage = mines.get_percentage(width, height);

        if percentage >= 0.9 {
            panic!("Too many mines, this won't be solvable!");
        }

        if percentage <= 0.0 {
            panic!("Negative or zero percentage of mines!");
        }

        if percentage > 0.25 {
            println!("Warning: {}% of the fields are mines!", percentage * 100.0);
        }

        let board = vec![vec![MineSweeperCell::Empty; height as usize]; width as usize];
        let mines = mines.get_fixed_count(width, height);

        let mut field = RandomField {
            width,
            height,
            mines,
            board,
            start_cell: (0, 0),
        };

        field.initialize();
        field
    }

    fn get_mines(&self) -> u32 {
        self.mines
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_start_cell(&self) -> (u32, u32) {
        self.start_cell
    }

    fn get_field(&self) -> Vec<Vec<MineSweeperCell>> {
        self.board.clone()
    }

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell {
        self.board[x as usize][y as usize].clone()
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell) {
        self.board[x as usize][y as usize] = cell;
    }

}

impl RandomField {

    fn initialize(&mut self) {
        self.place_mines();
        self.assign_numbers();
        self.set_start_cell();
    }

    fn set_start_cell(&mut self) {
        /*
        TODO:
        Set the start field to the first empty cell found
        Can later also be set to a random empty cell
        */
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == MineSweeperCell::Empty {
                self.start_cell = (x, y);
                return;
            }
        }
    }

    fn place_mines(&mut self) {
        let mut placed_mines = 0;
        /*
        TODO:
        Currently for testing purposes, but in the future a random seed will be used
        */
        let seed: u64 = 40;
        let mut rng = StdRng::seed_from_u64(seed);

        while placed_mines < self.mines {
            let x = (rng.random_range(0..u64::MAX) % self.width as u64 ) as u32;
            let y = (rng.random_range(0..u64::MAX) % self.height as u64 ) as u32;

            if self.get_cell(x, y) == MineSweeperCell::Empty {
                self.set_cell(x, y, MineSweeperCell::Mine);
                placed_mines += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation_basic() {
        let field = RandomField::new(10, 10, MineSweeperFieldCreation::FixedCount(10));

        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 10);
        assert_eq!(field.get_mines(), 10);
    }

    #[test]
    fn test_field_creation_percentage() {
        let field = RandomField::new(10, 10, MineSweeperFieldCreation::Percentage(0.2));

        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 10);
        assert_eq!(field.get_mines(), 20); // 100 * 0.2 = 20
    }

    #[test]
    fn test_mine_placement_count() {
        let field = RandomField::new(10, 10, MineSweeperFieldCreation::FixedCount(15));

        let mut mine_count = 0;
        for (x, y) in field.sorted_fields() {
            if field.get_cell(x, y) == MineSweeperCell::Mine {
                mine_count += 1;
            }
        }

        assert_eq!(mine_count, 15);
    }

    #[test]
    fn test_number_assignment() {
        let field = RandomField::new(20, 18, MineSweeperFieldCreation::Percentage(0.25));

        // Check that non-mine cells have appropriate numbers
        for (x, y) in field.sorted_fields() {
            let cell = field.get_cell(x, y);
            match cell {
                MineSweeperCell::Empty => {
                    // Should be surrounded by 0 mines
                    assert_eq!(field.get_sourrounding_mine_count(x, y), 0);
                },
                MineSweeperCell::Number(n) => {
                    // Number should match surrounding mine count
                    assert_eq!(field.get_sourrounding_mine_count(x, y), n);
                    assert!(n >= 1 && n <= 8);
                },
                MineSweeperCell::Mine => {
                    // Mine cells are valid
                }
            }
        }
    }

    #[test]
    fn test_start_cell_not_mine() {
        let field = RandomField::new(5, 5, MineSweeperFieldCreation::FixedCount(10));
        let (start_x, start_y) = field.get_start_cell();

        // Start field should not be a mine
        assert_ne!(field.get_cell(start_x, start_y), MineSweeperCell::Mine);
    }

    #[test]
    fn test_field_bounds() {
        let field = RandomField::new(3, 4, MineSweeperFieldCreation::FixedCount(2));

        // Test that all coordinates are within bounds
        for (x, y) in field.sorted_fields() {
            assert!(x < 3);
            assert!(y < 4);

            // Should be able to get cell without panic
            let _cell = field.get_cell(x, y);
        }
    }

    #[test]
    fn test_cell_modification() {
        let mut field = RandomField::new(3, 3, MineSweeperFieldCreation::FixedCount(1));

        // Test setting and getting cells
        field.set_cell(1, 1, MineSweeperCell::Number(5));
        assert_eq!(field.get_cell(1, 1), MineSweeperCell::Number(5));

        field.set_cell(2, 2, MineSweeperCell::Mine);
        assert_eq!(field.get_cell(2, 2), MineSweeperCell::Mine);
    }

    #[test]
    fn test_get_field_clone() {
        let field = RandomField::new(2, 2, MineSweeperFieldCreation::FixedCount(1));
        let board_clone = field.get_field();
        
        // Should be same dimensions
        assert_eq!(board_clone.len(), 2);
        assert_eq!(board_clone[0].len(), 2);
        
        // Should contain same data
        for x in 0..2 {
            for y in 0..2 {
                assert_eq!(field.get_cell(x, y), board_clone[x as usize][y as usize]);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Too many mines")]
    fn test_too_many_mines_panic() {
        RandomField::new(2, 2, MineSweeperFieldCreation::Percentage(0.95));
    }

    #[test]
    #[should_panic(expected = "Negative or zero percentage")]
    fn test_zero_mines_panic() {
        RandomField::new(5, 5, MineSweeperFieldCreation::Percentage(0.0));
    }

    #[test]
    #[should_panic(expected = "Negative or zero percentage")]
    fn test_negative_mines_panic() {
        RandomField::new(5, 5, MineSweeperFieldCreation::Percentage(-0.1));
    }

    #[test]
    fn test_clone() {
        let field1 = RandomField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        let field2 = field1.clone();

        assert_eq!(field1.get_width(), field2.get_width());
        assert_eq!(field1.get_height(), field2.get_height());
        assert_eq!(field1.get_mines(), field2.get_mines());
        assert_eq!(field1.get_start_cell(), field2.get_start_cell());

        // Check that all cells are the same
        for (x, y) in field1.sorted_fields() {
            assert_eq!(field1.get_cell(x, y), field2.get_cell(x, y));
        }
    }

    #[test]
    fn test_surrounding_mine_count() {
        let mut field = RandomField::new(3, 3, MineSweeperFieldCreation::FixedCount(2));
        
        // Clear all mines first
        for x in 0..3 {
            for y in 0..3 {
                field.set_cell(x, y, MineSweeperCell::Empty);
            }
        }
        
        // Place mines manually for testing
        field.set_cell(0, 0, MineSweeperCell::Mine);
        field.set_cell(2, 2, MineSweeperCell::Mine);
        
        // Test center cell - should see both mines
        assert_eq!(field.get_sourrounding_mine_count(1, 1), 2);
        
        // Test corner - should see one mine
        assert_eq!(field.get_sourrounding_mine_count(0, 1), 1);
        
        // Test far corner - should see one mine
        assert_eq!(field.get_sourrounding_mine_count(2, 1), 1);
    }
}
