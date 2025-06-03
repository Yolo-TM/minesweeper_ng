use colored::{ColoredString, Colorize};

#[derive(Clone, PartialEq, Debug)]
pub enum MineSweeperCell {
    Empty,
    Mine,
    Number(u8),
}

impl MineSweeperCell {
    pub fn get_number(&self) -> u8 {
        match self {
            MineSweeperCell::Empty => 0,
            MineSweeperCell::Mine => 9,
            MineSweeperCell::Number(num) => *num,
        }
    }

    pub fn get_colored(&self) -> ColoredString {
        match self.get_number() {
            0 => " ".white(),
            1 => "1".bright_blue(),
            2 => "2".green(),
            3 => "3".bright_red(),
            4 => "4".bright_purple(),
            5 => "5".yellow(),
            6 => "6".cyan(),
            7 => "7".black(),
            8 => "8".white(),
            9 => "#".black().bold(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_cell() {
        let cell = MineSweeperCell::Empty;
        assert_eq!(cell.get_number(), 0);
        assert!(cell == MineSweeperCell::Empty);
    }

    #[test]
    fn test_mine_cell() {
        let cell = MineSweeperCell::Mine;
        assert_eq!(cell.get_number(), 9);
        assert!(cell == MineSweeperCell::Mine);
    }

    #[test]
    fn test_number_cells() {
        for i in 1..=8 {
            let cell = MineSweeperCell::Number(i);
            assert_eq!(cell.get_number(), i);
            assert!(cell == MineSweeperCell::Number(i));
        }
    }

    #[test]
    fn test_cell_equality() {
        assert_eq!(MineSweeperCell::Empty, MineSweeperCell::Empty);
        assert_eq!(MineSweeperCell::Mine, MineSweeperCell::Mine);
        assert_eq!(MineSweeperCell::Number(3), MineSweeperCell::Number(3));

        assert_ne!(MineSweeperCell::Empty, MineSweeperCell::Mine);
        assert_ne!(MineSweeperCell::Number(1), MineSweeperCell::Number(2));
        assert_ne!(MineSweeperCell::Empty, MineSweeperCell::Number(0));
    }

    #[test]
    fn test_cell_clone() {
        let original = MineSweeperCell::Number(5);
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.get_number(), cloned.get_number());
    }

    #[test]
    fn test_colored_output_exists() {
        // Test that all cell types produce colored output without panicking
        assert!(MineSweeperCell::Empty.get_colored().len() > 0);
        assert!(MineSweeperCell::Mine.get_colored().len() > 0);

        for i in 1..=8 {
            assert!(MineSweeperCell::Number(i).get_colored().len() > 0);
        }
    }

    #[test]
    fn test_number_bounds() {
        // Test edge cases for numbers
        let cell_min = MineSweeperCell::Number(1);
        let cell_max = MineSweeperCell::Number(8);

        assert_eq!(cell_min.get_number(), 1);
        assert_eq!(cell_max.get_number(), 8);
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_cell_colored_panic() {
        // Create a Number cell with an invalid value (> 9)
        // This should panic when get_colored() is called due to the unreachable!() case
        let invalid_cell = MineSweeperCell::Number(10);
        invalid_cell.get_colored(); // Should panic
    }

    #[test]
    #[should_panic]
    fn test_extremely_large_number_cell_panic() {
        // Test with an extremely large number to ensure it panics
        let invalid_cell = MineSweeperCell::Number(255);
        invalid_cell.get_colored(); // Should panic
    }

    #[test]
    #[should_panic]
    fn test_edge_invalid_number_cell_panic() {
        // Test with number 11 (just above the valid range)
        let invalid_cell = MineSweeperCell::Number(11);
        invalid_cell.get_colored(); // Should panic
    }
}
