use super::{
    MineSweeperCell,
    MineSweeperFieldIterator,
    SurroundingFieldsIterator,
};

pub trait MineSweeperField: Sync + Send + Clone + 'static {

    #[track_caller]
    fn new(width: u32, height: u32, mines: MineSweeperFieldCreation) -> Self;

    fn get_mines(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_start_field(&self) -> (u32, u32);
    fn get_field(&self) -> Vec<Vec<MineSweeperCell>>;

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell;
    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell);

    fn show(&self) {
        let (w, h, m) = self.get_dimensions();
        println!("Width: {}, Height: {}, Mines: {}", w, h, m);
        println!("Start field: {:?}", self.get_start_field());
        for (x, y) in self.sorted_fields() {
            print!("{} ", &self.get_cell(x, y).get_colored());

            if x == self.get_width() - 1 {
                println!();
            }
        }
    }

    fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.get_width(), self.get_height(), self.get_mines())
    }

    fn assign_numbers(&mut self) {
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == MineSweeperCell::Mine {
                continue;
            }

            let count = self.get_sourrounding_mine_count(x, y);
            if count != 0 {
                self.set_cell(x, y, MineSweeperCell::Number(count));
            }
        }
    }

    fn get_sourrounding_mine_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for (x, y) in self.surrounding_fields(x, y, None) {
            if self.get_cell(x, y) == MineSweeperCell::Mine {
                count += 1;
            }
        }
        count
    }

    fn sorted_fields(&self) -> MineSweeperFieldIterator {
        MineSweeperFieldIterator {
            width: self.get_width(),
            height: self.get_height(),
            current_x: 0,
            current_y: 0,
        }
    }

    fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingFieldsIterator {
        let range = range.unwrap_or(1);

        SurroundingFieldsIterator {
            x,
            y,
            width: self.get_width(),
            height: self.get_height(),
            range,
            dx: -(range as i8),
            dy: -(range as i8),
        }
    }
    // Serialize / Output ?
}

#[derive(Clone)]
pub enum MineSweeperFieldCreation {
    FixedCount(u32),
    Percentage(f32),
}

impl MineSweeperFieldCreation {
    pub fn get_percentage(&self, w: u32, h: u32) -> f32 {
        match self {
            MineSweeperFieldCreation::FixedCount(count) => (*count as f32) / (w * h) as f32,
            MineSweeperFieldCreation::Percentage(percentage) => *percentage,
        }
    }

    pub fn get_fixed_count(&self, w: u32, h: u32) -> u32 {
        match self {
            MineSweeperFieldCreation::FixedCount(count) => *count,
            MineSweeperFieldCreation::Percentage(percentage) => ((w * h) as f32 * percentage) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_count_creation() {
        let creation = MineSweeperFieldCreation::FixedCount(20);
        assert_eq!(creation.get_fixed_count(10, 10), 20);
        assert_eq!(creation.get_percentage(10, 10), 0.2);

        // Test different field sizes
        assert_eq!(creation.get_percentage(5, 4), 1.0); // 20/20 = 1.0
        assert_eq!(creation.get_percentage(20, 10), 0.1); // 20/200 = 0.1
    }

    #[test]
    fn test_percentage_creation() {
        let creation = MineSweeperFieldCreation::Percentage(0.15);
        assert_eq!(creation.get_percentage(10, 10), 0.15);
        assert_eq!(creation.get_fixed_count(10, 10), 15); // 100 * 0.15 = 15

        // Test different field sizes
        assert_eq!(creation.get_fixed_count(20, 20), 60); // 400 * 0.15 = 60
        assert_eq!(creation.get_fixed_count(8, 8), 9); // 64 * 0.15 = 9.6 -> 9
    }

    #[test]
    fn test_zero_percentage() {
        let creation = MineSweeperFieldCreation::Percentage(0.0);
        assert_eq!(creation.get_percentage(10, 10), 0.0);
        assert_eq!(creation.get_fixed_count(10, 10), 0);
    }

    #[test]
    fn test_full_percentage() {
        let creation = MineSweeperFieldCreation::Percentage(1.0);
        assert_eq!(creation.get_percentage(10, 10), 1.0);
        assert_eq!(creation.get_fixed_count(10, 10), 100);
    }

    #[test]
    fn test_zero_fixed_count() {
        let creation = MineSweeperFieldCreation::FixedCount(0);
        assert_eq!(creation.get_fixed_count(10, 10), 0);
        assert_eq!(creation.get_percentage(10, 10), 0.0);
    }

    #[test]
    fn test_edge_cases() {
        // Test with minimal field size
        let creation = MineSweeperFieldCreation::Percentage(0.5);
        assert_eq!(creation.get_fixed_count(1, 1), 0); // 1 * 0.5 = 0.5 -> 0
        assert_eq!(creation.get_fixed_count(2, 1), 1); // 2 * 0.5 = 1.0 -> 1

        // Test high percentage
        let creation_high = MineSweeperFieldCreation::Percentage(0.9);
        assert_eq!(creation_high.get_fixed_count(10, 10), 90);
        assert_eq!(creation_high.get_percentage(10, 10), 0.9);
    }

    #[test]
    fn test_clone() {
        let original = MineSweeperFieldCreation::FixedCount(42);
        let cloned = original.clone();
        
        assert_eq!(original.get_fixed_count(10, 10), cloned.get_fixed_count(10, 10));
        assert_eq!(original.get_percentage(10, 10), cloned.get_percentage(10, 10));
    }

    #[test]
    fn test_rounding_behavior() {
        // Test that floating point calculations round down as expected
        let creation = MineSweeperFieldCreation::Percentage(0.33);
        assert_eq!(creation.get_fixed_count(10, 10), 33); // 100 * 0.33 = 33.0

        let creation2 = MineSweeperFieldCreation::Percentage(0.333);
        assert_eq!(creation2.get_fixed_count(10, 10), 33); // 100 * 0.333 = 33.3 -> 33
    }
}