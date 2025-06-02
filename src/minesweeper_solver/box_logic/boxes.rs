#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Box{
    fields: Vec<(u32, u32)>,
    owner: (u32, u32),
    mines: std::ops::RangeInclusive<u8>,
}

impl Box{
    pub fn new(x: u32, y: u32, mines: std::ops::RangeInclusive<u8>) -> Self {
        Box {
            fields: vec![],
            owner: (x, y),
            mines: mines,
        }
    }

    pub fn get_mines(&self) -> std::ops::RangeInclusive<u8> {
        return self.mines;
    }

    pub fn get_field_count(&self) -> usize {
        return self.fields.len();
    }

    pub fn add_field(&mut self, x: u32, y: u32) {
        self.fields.push((x, y));
    }

    pub fn remove_field(&mut self, x: u32, y: u32) {
        self.fields.retain(|&field| field != (x, y));
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        for field in &self.fields {
            if field.0 == x && field.1 == y {
                return true;
            }
        }
        false
    }

    pub fn is_owner(&self, x: u32, y: u32) -> bool {
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

    pub fn is_inside(&self, other: &Box) -> bool {
        let (shared, this_only, other_only) = self.compare_to(&other.fields);

        return this_only.is_empty()
    }

    pub fn covers_same_fields(&self, other: &Box) -> bool {
        let (shared, this_only, other_only) = self.compare_to(&other.fields);
        return this_only.is_empty() && other_only.is_empty();
    }

    pub fn has_same_range(&self, other: &Box) -> bool {
        return self.mines.start == other.mines.start && self.mines.end == other.mines.end;
    }
}