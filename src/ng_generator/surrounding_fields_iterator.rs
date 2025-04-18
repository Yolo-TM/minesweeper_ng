pub fn surrounding_fields(x: usize, y: usize, width: usize, height: usize) -> SurroundingFieldsIterator {
    SurroundingFieldsIterator::new(x, y, width, height, 1)
}

pub fn extended_surrounding_fields(x: usize, y: usize, width: usize, height: usize) -> SurroundingFieldsIterator {
    SurroundingFieldsIterator::new(x, y, width, height, 5)
}

pub struct SurroundingFieldsIterator {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    range: isize,
    dx: isize,
    dy: isize,
}

impl SurroundingFieldsIterator {
    pub fn new(x: usize, y: usize, width: usize, height: usize, range: isize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
            width: width as isize,
            height: height as isize,
            range,
            dx: -range,
            dy: -range,
        }
    }
}

impl Iterator for SurroundingFieldsIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.dy <= self.range {
            let nx = self.x + self.dx;
            let ny = self.y + self.dy;

            self.dx += 1;
            if self.dx > self.range {
                self.dx = -self.range;
                self.dy += 1;
            }

            if nx >= 0 && ny >= 0 && nx < self.width && ny < self.height && !(nx == self.x && ny == self.y) {
                return Some((nx as usize, ny as usize));
            }
        }
        None
    }
}