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

    pub fn get_field_count(&self) -> usize {
        return self.fields.len();
    }

    pub fn get_fields(&self) -> &Vec<(u32, u32)> {
        return &self.fields;
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
        let (_shared, this_only, _other_only) = self.compare_to(&other.fields);

        return this_only.is_empty()
    }

    pub fn covers_same_fields(&self, other: &Box) -> bool {
        let (_shared, this_only, other_only) = self.compare_to(&other.fields);
        return this_only.is_empty() && other_only.is_empty();
    }

    pub fn has_same_range(&self, other: &Box) -> bool {
        return self.mines.start() == other.mines.start() && self.mines.end() == other.mines.end();
    }

    pub fn is_same_range(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        return self.mines.start() == range.start() && self.mines.end() == range.end();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_creation() {
        let box_obj = Box::new(1, 2, 2..=4);

        assert_eq!(box_obj.owner, (1, 2));
        assert_eq!(box_obj.get_mines(), 2..=4);
        assert_eq!(box_obj.get_field_count(), 0);
    }

    #[test]
    fn test_add_remove_fields() {
        let mut box_obj = Box::new(0, 0, 1..=3);

        // Add fields
        box_obj.add_field(1, 1);
        box_obj.add_field(2, 2);
        assert_eq!(box_obj.get_field_count(), 2);

        // Check contains
        assert!(box_obj.contains(1, 1));
        assert!(box_obj.contains(2, 2));
        assert!(!box_obj.contains(3, 3));

        // Remove field
        box_obj.remove_field(1, 1);
        assert_eq!(box_obj.get_field_count(), 1);
        assert!(!box_obj.contains(1, 1));
        assert!(box_obj.contains(2, 2));
    }

    #[test]
    fn test_is_neighbouring() {
        let mut box_obj = Box::new(5, 5, 1..=2);
        box_obj.add_field(3, 3);
        box_obj.add_field(7, 7);

        // Should be neighbouring to fields within 1 distance
        assert!(box_obj.is_neighbouring(3, 4)); // Adjacent to (3,3)
        assert!(box_obj.is_neighbouring(4, 3)); // Adjacent to (3,3)
        assert!(box_obj.is_neighbouring(4, 4)); // Diagonal to (3,3)
        assert!(box_obj.is_neighbouring(6, 6)); // Adjacent to (7,7)

        // Should not be neighbouring to distant fields
        assert!(!box_obj.is_neighbouring(1, 1));
        assert!(!box_obj.is_neighbouring(9, 9));
        assert!(!box_obj.is_neighbouring(5, 3)); // Too far from both fields
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
    fn test_is_inside() {
        let mut box1 = Box::new(0, 0, 1..=2);
        box1.add_field(1, 1);
        box1.add_field(2, 2);

        let mut box2 = Box::new(1, 1, 2..=3);
        box2.add_field(1, 1);
        box2.add_field(2, 2);
        box2.add_field(3, 3);

        // box1 should be inside box2 (all fields of box1 are in box2)
        assert!(box1.is_inside(&box2));
        assert!(!box2.is_inside(&box1));
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
    fn test_is_same_range() {
        let box_obj = Box::new(0, 0, 2..=5);

        assert!(box_obj.is_same_range(2..=5));
        assert!(!box_obj.is_same_range(1..=5));
        assert!(!box_obj.is_same_range(2..=4));
        assert!(!box_obj.is_same_range(3..=6));
    }

    #[test]
    fn test_edge_cases() {
        let mut box_obj = Box::new(0, 0, 0..=0);

        // Empty box
        assert_eq!(box_obj.get_field_count(), 0);
        assert!(!box_obj.contains(0, 0));
        assert!(!box_obj.is_neighbouring(0, 0));

        // Single field
        box_obj.add_field(5, 5);
        assert_eq!(box_obj.get_field_count(), 1);
        assert!(box_obj.is_neighbouring(5, 6));
        assert!(box_obj.is_neighbouring(6, 6));
        assert!(!box_obj.is_neighbouring(7, 7));
    }

    #[test]
    fn test_remove_nonexistent_field() {
        let mut box_obj = Box::new(0, 0, 1..=2);
        box_obj.add_field(1, 1);

        // Removing a field that doesn't exist should not change anything
        box_obj.remove_field(2, 2);
        assert_eq!(box_obj.get_field_count(), 1);
        assert!(box_obj.contains(1, 1));
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
        assert_eq!(box1.get_field_count(), box2.get_field_count());
    }
}