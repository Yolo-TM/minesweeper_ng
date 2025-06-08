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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_creation() {
        let box_obj = Box::new(1, 2, 2..=4);

        assert_eq!(box_obj.get_owner(), (1, 2));
        assert_eq!(box_obj.get_mines(), 2..=4);
        assert_eq!(box_obj.get_fields().len(), 0);
    }

    #[test]
    fn test_box_creation_with_fields() {
        let mut box_obj = Box::new(0, 0, 1..=3);

        // Add fields
        box_obj.add_field(1, 1);
        box_obj.add_field(2, 2);
        assert_eq!(box_obj.get_fields().len(), 2);
    }

    #[test]
    fn test_compare_to() {
        let mut box_obj = Box::new(0, 0, 1..=2);
        box_obj.add_field(1, 1);
        box_obj.add_field(2, 2);
        box_obj.add_field(3, 3);

        let other_fields = vec![(2, 2), (3, 3), (4, 4)];
        let (shared, this_only, other_only) = box_obj.compare_to(&other_fields);

        assert_eq!(shared, vec![(2, 2), (3, 3)]);
        assert_eq!(this_only, vec![(1, 1)]);
        assert_eq!(other_only, vec![(4, 4)]);
    }

    #[test]
    fn test_covers_same_fields() {
        let mut box1 = Box::new(0, 0, 1..=2);
        box1.add_field(1, 1);
        box1.add_field(2, 2);

        let mut box2 = Box::new(1, 1, 2..=3);
        box2.add_field(1, 1);
        box2.add_field(2, 2);

        let mut box3 = Box::new(2, 2, 1..=1);
        box3.add_field(1, 1);
        box3.add_field(3, 3);

        assert!(box1.covers_same_fields(&box2));
        assert!(box2.covers_same_fields(&box1));
        assert!(!box1.covers_same_fields(&box3));
    }

    #[test]
    fn test_has_same_range() {
        let box1 = Box::new(0, 0, 1..=3);
        let box2 = Box::new(1, 1, 1..=3);
        let box3 = Box::new(2, 2, 2..=4);

        assert!(box1.has_same_range(&box2));
        assert!(!box1.has_same_range(&box3));
        assert!(!box2.has_same_range(&box3));
    }

    #[test]
    fn test_clone_and_equality() {
        let mut box1 = Box::new(1, 2, 3..=5);
        box1.add_field(4, 5);
        box1.add_field(6, 7);

        let box2 = box1.clone();

        assert_eq!(box1, box2);
        assert_eq!(box1.owner, box2.owner);
        assert_eq!(box1.get_mines(), box2.get_mines());
        assert_eq!(box1.get_fields().len(), box2.get_fields().len());
    }
}