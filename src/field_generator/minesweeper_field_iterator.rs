
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
