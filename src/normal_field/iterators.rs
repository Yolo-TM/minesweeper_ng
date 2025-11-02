pub struct SortedCells {
    pub width: u32,
    pub height: u32,
    pub current_x: u32,
    pub current_y: u32,
}

impl Iterator for SortedCells {
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

pub struct SurroundingCells {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub range: u8,
    pub dx: i8,
    pub dy: i8,
}

impl Iterator for SurroundingCells {
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

impl SurroundingCells {
    fn increment(&mut self) {
        self.dx += 1;
        if self.dx > self.range as i8 {
            self.dx = -(self.range as i8);
            self.dy += 1;
        }
    }
}