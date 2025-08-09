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