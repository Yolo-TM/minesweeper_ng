use super::Solver;
use std::vec;

/*

Boxes strategy:

- Complex strategy which analyses groups of fields and their mine counts to deduce safe fields and mine fields.
- Should be able to solve things like 1-3-1 corners etc..
- Works Partially
- Logic is very complex and not proven to be true / work for every Case
> Therefore this strategy is experimental and not used for now

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
        let group = Group::new(surrounding_unrevealed, reduced_count..=reduced_count);

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
            possible_relevant_groups.push(Group::new(surrounding_unrevealed, reduced_count..=reduced_count));
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
            dedup_relevant_groups.push(Group::new(g.fields.clone(), g.range.clone()));
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
            if *g.range.end() == 1 {
                relevant_groups.push(Group::from(g));
                continue;
            }

            let mut found_subset = false;
            for other in &possible_relevant_groups {
                if other.is_subset_of(g) && other.fields.len() != g.fields.len() {
                    let mut shared_fields = vec![];
                    let mut unique_fields = vec![];
                    for field in &g.fields {
                        if other.fields.contains(field) {
                            shared_fields.push(*field);
                        } else {
                            unique_fields.push(*field);
                        }
                    }
                    let shared_count = *other.range.start();
                    let unique_count = g.range.start().saturating_sub(shared_count);
                    relevant_groups.push(Group::new(shared_fields, shared_count..=shared_count));
                    relevant_groups.push(Group::new(unique_fields, unique_count..=unique_count));
                    found_subset = true;
                    break;
                }
            }
            if !found_subset {
                relevant_groups.push(Group::from(g));
            }
        }

        // Re-check relevance after subset splitting
        relevant_groups.retain(|g| g.is_relevant_to(&group));

        // Begin refinement of constraints by overlapping each relevant group with cumulatively refined set
        let mut all_groups = vec![Group::from(&group)];
        for g in &relevant_groups {
            let mut next_all_groups: Vec<Group> = Vec::new();
            let push_or_merge = |target: &mut Vec<Group>, mut new_g: Group| {
                let max_possible = new_g.fields.len() as u8;
                let start = *new_g.range.start();
                let end = *new_g.range.end();
                let clamped_start = start.min(max_possible);
                let clamped_end = end.min(max_possible);
                new_g.range = clamped_start..=clamped_end;
                if clamped_start > clamped_end { return; }
                for existing in target.iter_mut() {
                    if existing.has_same_fields(&new_g) {
                        let ns = *new_g.range.start();
                        let ne = *new_g.range.end();
                        let es = *existing.range.start();
                        let ee = *existing.range.end();
                        let is = ns.max(es);
                        let ie = ne.min(ee);
                        if is <= ie { existing.range = is..=ie; }
                        return;
                    }
                }
                target.push(new_g);
            };
            for ag in &all_groups {
                let shared_fields: Vec<(u32, u32)> = g.fields.iter().filter(|f| ag.fields.contains(f)).cloned().collect();
                let unique_fields_g: Vec<(u32, u32)> = g.fields.iter().filter(|f| !ag.fields.contains(f)).cloned().collect();
                let unique_fields_ag: Vec<(u32, u32)> = ag.fields.iter().filter(|f| !g.fields.contains(f)).cloned().collect();
                if shared_fields.is_empty() {
                    push_or_merge(&mut next_all_groups, Group::from(ag));
                    continue;
                }
                let s = shared_fields.len() as u8;
                let a = unique_fields_ag.len() as u8;
                let b = unique_fields_g.len() as u8;
                let min_g = *g.range.start();
                let max_g = *g.range.end();
                let min_ag = *ag.range.start();
                let max_ag = *ag.range.end();
                let mut x_low = 0u8;
                x_low = x_low.max(min_ag.saturating_sub(a));
                x_low = x_low.max(min_g.saturating_sub(b));
                let x_high = s.min(max_ag).min(max_g);
                if x_low > x_high {
                    push_or_merge(&mut next_all_groups, Group::from(ag));
                    continue;
                }
                let shared_group = if !shared_fields.is_empty() { Some(Group::new(shared_fields.clone(), x_low..=x_high)) } else { None };
                let y_low = min_ag.saturating_sub(x_high).min(a);
                let y_high = max_ag.saturating_sub(x_low).min(a);
                let unique_ag_group = if !unique_fields_ag.is_empty() { Some(Group::new(unique_fields_ag.clone(), y_low..=y_high)) } else { None };
                let z_low = min_g.saturating_sub(x_high).min(b);
                let z_high = max_g.saturating_sub(x_low).min(b);
                if let Some(sg) = shared_group { push_or_merge(&mut next_all_groups, sg); }
                if let Some(ug) = unique_ag_group { push_or_merge(&mut next_all_groups, ug); }
                let mut record = |fields: &Vec<(u32, u32)>, low: u8, high: u8| {
                    if fields.is_empty() { return; }
                    let n = fields.len() as u8;
                    if high == 0 {
                        for &f in fields { if !safe_fields.contains(&f) { safe_fields.push(f); } }
                    }
                    if low == n {
                        for &f in fields { if !mine_fields.contains(&f) { mine_fields.push(f); } }
                    }
                };
                record(&shared_fields, x_low, x_high);
                record(&unique_fields_ag, y_low, y_high);
                record(&unique_fields_g, z_low, z_high);
            }
            let mut merged: Vec<Group> = Vec::new();
            'outer_merge: for ng in next_all_groups {
                for mg in merged.iter_mut() {
                    if mg.has_same_fields(&ng) {
                        let ns = *ng.range.start();
                        let ne = *ng.range.end();
                        let es = *mg.range.start();
                        let ee = *mg.range.end();
                        let is = ns.max(es);
                        let ie = ne.min(ee);
                        if is <= ie { mg.range = is..=ie; }
                        continue 'outer_merge;
                    }
                }
                merged.push(ng);
            }
            all_groups = merged;
        }

        println!("Current Field: ({}, {})\n Group: {:?}", x, y, group.fields);
        println!("Possible relevant groups:");
        for g in &relevant_groups {
            println!("  Group: {:?}, Count: {:?}", g.fields, g.range);
        }
    }

    (safe_fields, mine_fields)
}

struct Group {
    fields: Vec<(u32, u32)>,
    range: std::ops::RangeInclusive<u8>,
}

impl Group {
    fn new(fields: Vec<(u32, u32)>, range: std::ops::RangeInclusive<u8>) -> Self {
        Self { fields, range }
    }

    fn from(other: &Group) -> Self {
        Self {
            fields: other.fields.clone(),
            range: other.range.clone(),
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
        self.range.start() == other.range.start() && self.range.end() == other.range.end()
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
