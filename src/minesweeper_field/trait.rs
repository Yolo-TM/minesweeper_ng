use super::{Cell, SortedCells, SurroundingCells};

pub trait MineSweeperField: Clone {
    fn get_mines(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_start_cell(&self) -> (u32, u32);

    fn get_cell(&self, x: u32, y: u32) -> &Cell;
    fn set_cell(&mut self, x: u32, y: u32, cell: Cell);

    fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.get_width(), self.get_height(), self.get_mines())
    }

    fn assign_numbers(&mut self) {
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == &Cell::Mine {
                continue;
            }

            let count = self.get_surrounding_mine_count(x, y);
            if count != 0 {
                self.set_cell(x, y, Cell::Number(count));
            } else if self.get_cell(x, y) != &Cell::Empty {
                self.set_cell(x, y, Cell::Empty);
            }
        }
    }

    fn get_surrounding_mine_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for (x, y) in self.surrounding_fields(x, y, None) {
            if self.get_cell(x, y) == &Cell::Mine {
                count += 1;
            }
        }
        count
    }

    fn sorted_fields(&self) -> SortedCells {
        SortedCells {
            width: self.get_width(),
            height: self.get_height(),
            current_x: 0,
            current_y: 0,
        }
    }

    fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingCells {
        let range = range.unwrap_or(1);

        SurroundingCells {
            x,
            y,
            width: self.get_width(),
            height: self.get_height(),
            range,
            dx: -(range as i8),
            dy: -(range as i8),
        }
    }
}
