#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Box{
    fields: Vec<(u32, u32)>,
    owner: (u32, u32),
    mines: u8,
}

impl Box{
    pub fn new(x: u32, y: u32, mines: u8) -> Self {
        Box {
            fields: vec![],
            owner: (x, y),
            mines: mines,
        }
    }

    pub fn get_mines(&self) -> u8 {
        return self.mines;
    }

    pub fn get_field_count(&self) -> usize {
        return self.fields.len();
    }

    pub fn add_field(&mut self, x: u32, y: u32) {
        self.fields.push((x, y));
    }

    fn remove_field(&mut self, x: u32, y: u32) {
        self.fields.retain(|&field| field != (x, y));
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        for field in &self.fields {
            if field.0 == x && field.1 == y {
                return true;
            }
        }
        false
    }

    fn is_owner(&self, x: u32, y: u32) -> bool {
        return self.owner.0 == x && self.owner.1 == y
    }

    pub fn is_neighbouring(&self, x: u32, y: u32) -> bool {
        for field in &self.fields {
            if (field.0 as isize - x as isize).abs() <= 1 && (field.1 as isize - y as isize).abs() <= 1 {
                return true;
            }
        }
        false
    }

    pub fn compare_to(&self, other: &Vec<(u32, u32)>) -> (Vec<(u32, u32)>, Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut shared: Vec<(u32, u32)> = vec![];
        let mut this_only = vec![];
        let mut other_only = vec![];

        for field in &self.fields {
            if !other.contains(field) {
                this_only.push(*field);
            } else {
                shared.push(*field);
            }
        }
        for field in other {
            if !self.fields.contains(field) {
                other_only.push(*field);
            }
        }

        (shared, this_only, other_only)
    }

    fn is_inside(&self, other: &Box) -> bool {
        for field in &self.fields {
            if !other.fields.contains(field) {
                return false;
            }
        }
        true
    }
}