#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Box{
    fields: Vec<(u32, u32)>,
    owner: (u32, u32),
    mines: std::ops::RangeInclusive<usize>,
}

impl Box{
    pub fn new(x: u32, y: u32, mines: std::ops::RangeInclusive<usize>) -> Self {
        Box {
            fields: vec![],
            owner: (x, y),
            mines: mines,
        }
    }

    pub fn get_mines(&self) -> std::ops::RangeInclusive<usize> {
        return self.mines.clone();
    }

    pub fn get_owner(&self) -> (u32, u32) {
        return self.owner;
    }

    pub fn get_fields(&self) -> &Vec<(u32, u32)> {
        return &self.fields;
    }

    pub fn add_field(&mut self, x: u32, y: u32) {
        self.fields.push((x, y));
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

    pub fn covers_same_fields(&self, other: &Box) -> bool {
        let (_shared, this_only, other_only) = self.compare_to(&other.fields);
        return this_only.is_empty() && other_only.is_empty();
    }

    pub fn is_inside_of(&self, other: &Box) -> bool {
        let (_shared, this_only, _other_only) = self.compare_to(&other.fields);

        return this_only.is_empty()
    }

    pub fn has_same_range(&self, other: &Box) -> bool {
        return self.mines.start() == other.mines.start() && self.mines.end() == other.mines.end();
    }

    pub fn is_same_range(&self, other: std::ops::RangeInclusive<usize>) -> bool {
        return self.mines.start() == other.start() && self.mines.end() == other.end();
    }
}