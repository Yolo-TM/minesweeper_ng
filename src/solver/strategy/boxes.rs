use super::Solver;
use std::vec;

/*

Boxes strategy: In Rework

*/

pub fn solve(solver: &Solver) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut safe_fields = vec![];
    let mut mine_fields = vec![];

    for (x, y) in solver.sorted_fields() {
        if !solver.has_informations(x, y) {
            continue;
        }

        let surrounding_unrevealed = solver.get_surrounding_unrevealed(x, y);
        let reduced_count = solver.get_reduced_count(x, y);
        let group = Group::new(surrounding_unrevealed, reduced_count);

        /*
        Gather all Possible Groups around our current one
        */
        let mut possible_relevant_groups: Vec<Group> = vec![];
        for (sx, sy) in solver.surrounding_fields(x, y, Some(2)) {
            if !solver.has_informations(sx, sy) {
                continue;
            }

            let surrounding_unrevealed = solver.get_surrounding_unrevealed(sx, sy);
            let reduced_count = solver.get_reduced_count(sx, sy);
            possible_relevant_groups.push(Group::new(surrounding_unrevealed, reduced_count));
        }

        /*
        Fields with the same count and same fields are duplicates which can be removed
        println!("Deduplicateing groups...");
        */
        let mut dedup_relevant_groups: Vec<Group> = vec![];
        'outer: for g in &possible_relevant_groups {
            for dg in &dedup_relevant_groups {
                if g.is_same(dg) {
                    continue 'outer;
                }
            }
            dedup_relevant_groups.push(Group::new(g.fields.clone(), g.count));
        }

        /*
        A Group is relevant if the majority of its own fields are shared with the master group
        println!("Removing non relevant groups...");
        */
        dedup_relevant_groups.retain(|g| g.is_relevant_to(&group));

        /*
        If there are any relevant groups with more than 1 mine count, we can try to break down this group further
        */
        let mut relevant_groups: Vec<Group> = vec![];
        for g in &dedup_relevant_groups {
            if g.count == 1 {
                relevant_groups.push(Group::from(g));
                continue;
            }

            // Try to find subsets
            let mut found_subset = false;
            for other in &possible_relevant_groups {
                if other.is_subset_of(g) && other.fields.len() != g.fields.len() {
                    // create the 2 new groups
                    let mut shared_fields = vec![];
                    let mut unique_fields = vec![];

                    for field in &g.fields {
                        if other.fields.contains(field) {
                            shared_fields.push(*field);
                        } else {
                            unique_fields.push(*field);
                        }
                    }

                    let shared_count = other.count;
                    let unique_count = g.count - shared_count;
                    relevant_groups.push(Group::new(shared_fields, shared_count));
                    relevant_groups.push(Group::new(unique_fields, unique_count));
                    found_subset = true;
                    break;
                }
            }

            if !found_subset {
                relevant_groups.push(Group::from(g));
            }
        }

        /*
        Check Relevance again after subset splitting
        */
        relevant_groups.retain(|g| g.is_relevant_to(&group));

        /*
        Now Compare or new groups to our main group and check if we can deduce any safe or mine fields
        */

        println!("Current Field: ({}, {})\n Group: {:?}", x, y, group.fields);
        println!("Possible relevant groups:");
        for g in &relevant_groups {
            println!("  Group: {:?}, Count: {}", g.fields, g.count);
        }
    }

    (safe_fields, mine_fields)
}

struct Group {
    fields: Vec<(u32, u32)>,
    count: u8,
}

impl Group {
    fn new(fields: Vec<(u32, u32)>, count: u8) -> Self {
        Self { fields, count }
    }

    fn from(other: &Group) -> Self {
        Self {
            fields: other.fields.clone(),
            count: other.count,
        }
    }

    fn is_relevant_to(&self, other: &Group) -> bool {
        let mut shared_fields = 0;

        for field in &self.fields {
            if other.fields.contains(field) {
                shared_fields += 1;
            }
        }

        shared_fields * 2 > self.fields.len()
    }

    fn has_same_count(&self, other: &Group) -> bool {
        self.count == other.count
    }

    fn has_same_fields(&self, other: &Group) -> bool {
        if self.fields.len() != other.fields.len() {
            return false;
        }

        for field in &self.fields {
            if !other.fields.contains(field) {
                return false;
            }
        }

        true
    }

    fn is_same(&self, other: &Group) -> bool {
        return self.has_same_count(other) && self.has_same_fields(other);
    }

    fn is_subset_of(&self, other: &Group) -> bool {
        for field in &self.fields {
            if !other.fields.contains(field) {
                return false;
            }
        }

        true
    }
}
