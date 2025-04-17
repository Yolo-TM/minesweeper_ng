use super::minesweeper_field::MineSweeperField;

pub struct MineSweeperFieldIterator {
    width: usize,
    height: usize,
    current_x: usize,
    current_y: usize,
}

impl MineSweeperField {
    pub fn sorted_fields(&self) -> MineSweeperFieldIterator {
        MineSweeperFieldIterator {
            width: self.width.clone(),
            height: self.height.clone(),
            current_x: 0,
            current_y: 0,
        }
    }
}

impl Iterator for MineSweeperFieldIterator {
    type Item = (usize, usize);

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