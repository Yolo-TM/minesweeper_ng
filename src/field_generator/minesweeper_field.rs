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