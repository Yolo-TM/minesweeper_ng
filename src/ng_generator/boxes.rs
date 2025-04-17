#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Box{
    pub fields: Vec<(usize, usize)>,
    pub owner: (usize, usize),
    pub mines: u8,
}

impl Box{
    pub fn new(x: usize, y: usize, mines: u8) -> Self {
        Box {
            fields: vec![],
            owner: (x, y),
            mines: mines,
        }
    }

    pub fn add_field(&mut self, x: usize, y: usize) {
        self.fields.push((x, y));
    }

    pub fn is_neighbouring(&self, x: usize, y: usize) -> bool {
        for field in &self.fields {
            if (field.0 as isize - x as isize).abs() <= 1 && (field.1 as isize - y as isize).abs() <= 1 {
                return true;
            }
        }
        false
    }

    pub fn is_owner(&self, x: usize, y: usize) -> bool {
        return self.owner.0 == x && self.owner.1 == y
    }

    pub fn get_mine_count(&self) -> u8 {
        return self.mines;
    }

    pub fn compare_to(&self, other: &Box) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut shared: Vec<(usize, usize)> = vec![];
        let mut this_only = vec![];
        let mut other_only = vec![];

        for field in &self.fields {
            if !other.fields.contains(field) {
                this_only.push(*field);
            } else {
                shared.push(*field);
            }
        }
        for field in &other.fields {
            if !self.fields.contains(field) {
                other_only.push(*field);
            }
        }

        (shared, this_only, other_only)
    }
}