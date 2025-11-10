impl<M> MineSweeperSolver<M> where M: MineSweeperField {

    pub(in crate::solver) fn apply_extended_box_logic(&self) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut safe_fields = vec![];
        let mut mine_fields = vec![];
        let mut boxes: Vec<Box> = vec![];

        // Create Boxes around fields with unrevealed neighbours
        for (x, y) in self.field.sorted_fields() {
            if !self.has_informations(x, y) {
                continue;
            }

            let count = self.get_reduced_count(x, y) as usize;
            let surrounding_hidden_fields = self.get_surrounding_unrevealed(x, y);
            let mut new_box = Box::new(x, y, count..=count);
            for cell in &surrounding_hidden_fields {
                new_box.add_field(cell.0, cell.1);
            }
            boxes.push(new_box);
        }

        // Map Boxes to other Boxes which could be important
        let mut possible_cases: Vec<(Box, Vec<Box>)> = vec![];
        for i in 0..boxes.len() {
            // Search through all boxes to find possible important adjacent boxes

            let box_ = &boxes[i];
            let mut possible_fields: Vec<Box> = vec![];

            for j in 0..boxes.len() {
                if i == j {
                    continue;
                }
                let other_box = &boxes[j];
                let (shared, _this, other) = box_.compare_to(&other_box.get_fields());

                // If this box could hold important informations
                // it shares more fields than fields it doesnt share
                if shared.len() > 0 && shared.len() >= other.len() && !(box_.covers_same_fields(other_box) && box_.has_same_range(other_box)) {

                    // check if we dont already have it
                    let mut allow_insert = true;
                    for fieldn in &possible_fields {
                        if fieldn.covers_same_fields(other_box) && fieldn.has_same_range(other_box) {
                            allow_insert = false;
                            break;
                        }
                    }

                    if allow_insert {
                        possible_fields.push(other_box.clone());
                    }
                }
            }

            // also collect all boxes which are completely inside of boxes in the vector bc their information might be important
            for j in 0..boxes.len() {
                if i == j {
                    continue;
                }
                // if the box is already inside of possible_fields, we dont need to check it
                if possible_fields.iter().any(|b| b.get_owner() == boxes[j].get_owner()) {
                    continue;
                }

                // if it would be a duplicate, we dont need to check it
                if possible_fields.iter().any(|b| b.covers_same_fields(&boxes[j]) && b.has_same_range(&boxes[j])) {
                    continue;
                }

                if possible_fields.iter().any(|b| boxes[j].is_inside_of(b)) {
                    possible_fields.push(boxes[j].clone());
                }
            }

            if !possible_fields.is_empty() {
                let mut allow_insert = true;

                // go through all possible fields and check that there is atleast one range which is different to box_
                let range = box_.get_mines();
                if possible_fields.iter().all(|field| field.get_mines() == range) {
                    allow_insert = false;
                }

                if allow_insert {
                    possible_cases.push((box_.clone(), possible_fields));
                }
            }
        }

        for (original, adjacent) in possible_cases {
            let mut field_tuples: Vec<(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)> = vec![(original.get_mines(), original.get_fields().clone())];
            let mut safe = vec![];
            let mut mines = vec![];

            self.recursive_search(original.get_fields(), &mut field_tuples, &adjacent, 0, &mut safe, &mut mines);

            // Nothing found, lets try all permutations
            if safe.is_empty() && mines.is_empty() {
                let mut all_boxes = adjacent.iter().map(|b| b.clone()).collect::<Vec<Box>>();
                all_boxes.append(vec![original.clone()].as_mut());
                self.deep_search(&all_boxes, &mut safe, &mut mines);
            }

            // Still nothing :(
            if safe.is_empty() && mines.is_empty() {
                continue;
            }

            for &(x, y) in &safe {
                safe_fields.push((x, y));
            }

            for &(x, y) in &mines {
                mine_fields.push((x, y));
            }
        }

        (safe_fields, mine_fields)
    }

    fn recursive_search(
        &self,
        original_fields: &Vec<(u32, u32)>,
        field_tuples: &mut Vec<(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)>,
        boxes: &Vec<Box>,
        current_box_index: usize,
        safe_fields: &mut Vec<(u32, u32)>,
        mine_fields: &mut Vec<(u32, u32)>
    ) {
        if current_box_index == boxes.len() {
            return;
        }
        let box_ = boxes[current_box_index].clone();
        {
            // Overlay the field tuples with our current information in the current box
            let mut new_tuples = Vec::new();
            let mut processed_indices = Vec::new();
            
            for i in 0..field_tuples.len() {
                let (range, fields) = &field_tuples[i].clone();
                let (shared, this_only, other_only) = box_.compare_to(fields);
                let second_range = box_.get_mines().clone();

                if shared.len() == 0 || other_only.len() == 0 {
                    // No overlap, keep the tuple as is
                    continue;
                }

                let mut new_shared_range = second_range.start().saturating_sub(this_only.len())..=*second_range.end().min(&shared.len());
                let mut new_other_range = range.start().saturating_sub(*new_shared_range.end())..=range.end().saturating_sub(*new_shared_range.start());

                if (new_shared_range.end() > new_other_range.end())
                || (shared.len() == 1 && new_shared_range.start() != new_shared_range.end())
                || (other_only.len() == 1 && *new_other_range.start() == 0 && *new_other_range.end() == 1) {
                    // If the new shared range is larger than the new other range, we cannot split
                    // or 
                    // If the shared range is only one field, we cannot split it
                    // or
                    // we are creating a 0..1 range for a single field in other_only

                    // stop
                    continue;
                } else if other_only.len() == 1 && new_other_range.start() != new_other_range.end() && *new_other_range.start() != 0 {
                    // if other only contains 1 mine
                    // and the the range is not definitive (start != end)
                    // e.g. range 1..=3 for other_only but its len = 1
                    // and the start is not zero (would lead to a range of 0..1 which is not helpful)

                    let excess_mines = new_other_range.end() - new_other_range.start();

                    if self.did_redistribute_mines(excess_mines, &mut new_shared_range, field_tuples, i) {
                        // Clamp the other_only range to its actual field count
                        new_other_range = *new_other_range.start()..=other_only.len();
                    } else {
                        // didnt work .-.
                        continue;
                    }
                } else if *new_other_range.end() > other_only.len() {
                    // If the new other range is larger than the shared range, we cannot split
                    let diff = *new_other_range.end() - other_only.len();
                    new_other_range = *new_other_range.start()..=other_only.len();
                    new_shared_range = *new_shared_range.start() + diff..=*new_shared_range.end();

                    if new_shared_range.start() > new_shared_range.end() {
                        // If the new shared range is invalid, skip this tuple
                        continue;
                    }
                }

                // Mark this tuple for removal since it will be split
                processed_indices.push(i);

                new_tuples.push((
                    new_other_range,
                    other_only.clone()
                ));

                new_tuples.push((
                    new_shared_range,
                    shared.clone()
                ));
            }

            for &index in processed_indices.iter().rev() {
                field_tuples.remove(index);
            }
            field_tuples.append(&mut new_tuples);
        }
        self.recursive_search(original_fields, field_tuples, boxes, current_box_index + 1, safe_fields, mine_fields);
        {
            // Cross check if we can get informations based on the field tuples and the current box
            for i in 0..field_tuples.len() {
                let (tuple_range, fields) = &field_tuples[i];

                // if the range is the same as the amount of fields, we can assume that all fields are mines
                if *tuple_range.start() == *tuple_range.end() && *tuple_range.start() == fields.len() {
                    for field in fields {
                        if !mine_fields.contains(field) {
                            mine_fields.push(*field);
                        }
                    }
                }

                let (_shared, this_only, other_only) = box_.compare_to(&fields);
                // no overlap
                if other_only.len() != 0 {
                    continue;
                }

                // were overlapping, check if we can get safe fields
                if box_.is_same_range(tuple_range.clone()) {
                    for field in &this_only {
                        safe_fields.push(*field);
                    }
                }
            }
        }
    }

    fn deep_search(
        &self,
        boxes: &Vec<Box>,
        safe_fields: &mut Vec<(u32, u32)>,
        mine_fields: &mut Vec<(u32, u32)>
    ) {
        let mut all_fields: HashMap<(u32, u32), u32> = HashMap::new();
        for i in 0 .. boxes.len() {
            for field in boxes[i].get_fields() {
                all_fields.insert(*field, 0);
            }
        }

        let valid_permutations = self.generate_all_permutations(&mut all_fields, &boxes);

        for ((x, y), count) in all_fields.iter() {
            if *count == 0 {
                // This field is never a mine, so we can reveal it
                if !safe_fields.contains(&(*x, *y)) {
                    safe_fields.push((*x, *y));
                }
            } else if *count == valid_permutations {
                // This field is always a mine, so we can flag it
                if !mine_fields.contains(&(*x, *y)) {
                    mine_fields.push((*x, *y));
                }
            }
        }
    }

    fn generate_all_permutations(
        &self,
        all_fields: &mut HashMap<(u32, u32), u32>,
        boxes: &Vec<Box>
    ) -> u32 {
        let mut field_vec: Vec<((u32, u32), bool) > = all_fields.iter().map(|(&k, &v)| (k, v == 1)).collect();
        let mut permutations = 0;

        self.recursively_generate_permutations(all_fields, &mut field_vec, 0, boxes, &mut permutations);
        return permutations;
    }

    fn recursively_generate_permutations(
        &self,
        field_map: &mut HashMap<(u32, u32), u32>,
        field_vec: &mut Vec<((u32, u32), bool)>,
        index: usize,
        boxes: &Vec<Box>,
        permutations: &mut u32
    ) {
        if index == field_vec.len() {
            self.check_permutation(field_map, permutations, field_vec, boxes);
            return;
        }

        // always reset all following fields
        for i in index..field_vec.len() {
            field_vec[i].1 = false;
        }

        self.recursively_generate_permutations(field_map, field_vec, index + 1, boxes, permutations);

        field_vec[index].1 = true; // Mine
        self.recursively_generate_permutations(field_map, field_vec, index + 1, boxes, permutations);
    }

    fn check_permutation(
        &self,
        field_map: &mut HashMap<(u32, u32), u32>,
        permutations: &mut u32,
        field_vec: &Vec<((u32, u32), bool)>,
        boxes: &Vec<Box>
    ) {
        let lookup_map: HashMap<(u32, u32), bool> = field_vec.iter()
            .map(|(field, is_mine)| (*field, if *is_mine { true } else { false }))
            .collect();

        // go through all boxes and check if the minecount at those fields is in range of the needed minecount
        for box_ in boxes {
            let mut mine_count = 0;
            for field in box_.get_fields() {
                if let Some(&is_mine) = lookup_map.get(field) {
                    if is_mine {
                        mine_count += 1;
                    }
                }
            }

            if !box_.get_mines().contains(&mine_count) {
                // This box is invalid, stop checking
                return;
            }
        }

        *permutations += 1;
        // Update the field_map with the current permutation
        for (field, is_mine) in field_vec {
            if !is_mine {
                continue;
            }

            if let Some(count) = field_map.get_mut(field) {
                *count += 1;
            }
        }
    }

    fn did_redistribute_mines(
        &self,
        excess_mines: usize,
        shared_range: &mut std::ops::RangeInclusive<usize>,
        field_tuples: &mut [(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)],
        current_tuple_index: usize
    ) -> bool {
        let mut remaining_excess = excess_mines;

        // does it fit into the shared range?
        let shared_capacity = shared_range.end().saturating_sub(*shared_range.start());
        let shared_absorption = remaining_excess.min(shared_capacity);

        if shared_absorption > 0 {
            *shared_range = (*shared_range.start() + shared_absorption)..=*shared_range.end();
            remaining_excess -= shared_absorption;
        }

        // add it to other tuples
        if remaining_excess > 0 {
            self.distribute_excess_to_other_tuples(
                &mut remaining_excess,
                field_tuples,
                current_tuple_index
            );
        }

        // did we do it?
        remaining_excess == 0
    }

    fn distribute_excess_to_other_tuples(
        &self,
        remaining_excess: &mut usize,
        field_tuples: &mut [(std::ops::RangeInclusive<usize>, Vec<(u32, u32)>)],
        skip_index: usize
    ) {
        // can we do sth?
        let mut distribution_plan = Vec::new();
        let mut test_excess = *remaining_excess;
        let mut too_much_available_tuples = false;

        for (j, (range, _fields)) in field_tuples.iter().enumerate() {
            if j == skip_index {
                continue;
            }

            let tuple_capacity = range.end().saturating_sub(*range.start());

            if tuple_capacity == 0 {
                // No capacity to absorb excess
                continue;
            }

            if test_excess == 0 {
                // we have nothing to distribute, but we have tuples left which would absorb sth
                // this could mean that a previous tuple is overloaded and therefore wrong if we add the absorption to it
                too_much_available_tuples = true;
                break;
            }

            // could be a problem as this fills as much as possible to this tuple instead of distributing it evenly / based on logic
            let absorption = test_excess.min(tuple_capacity);

            if absorption > 0 {
                distribution_plan.push((j, absorption));
                test_excess -= absorption;
            }
        }

        // apply
        if test_excess == 0 && !too_much_available_tuples {
            for (index, absorption) in distribution_plan {
                let (range, _fields) = &mut field_tuples[index];
                let new_start = *range.start() + absorption;
                *range = new_start..=*range.end();
            }
            *remaining_excess = 0;
        }
    }
}