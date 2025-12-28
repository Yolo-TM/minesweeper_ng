#[derive(Debug)]
pub struct Finding {
    safe_fields: Vec<(u32, u32)>,
    recursive_informations: Vec<Vec<(u32, u32)>>, // Fields which were also revealed after applying this Findings informations
    mine_fields: Vec<(u32, u32)>,
}

impl Finding {
    pub fn new() -> Self {
        Finding {
            safe_fields: Vec::new(),
            recursive_informations: Vec::new(),
            mine_fields: Vec::new(),
        }
    }

    pub fn success(&self) -> bool {
        !self.safe_fields.is_empty() || !self.mine_fields.is_empty()
    }

    pub fn add_safe_field(&mut self, field: (u32, u32)) {
        if !self.safe_fields.contains(&field) {
            self.safe_fields.push(field);
        }
    }

    pub fn add_safe_fields(&mut self, fields: Vec<(u32, u32)>) {
        for field in fields {
            self.add_safe_field(field);
        }
    }

    pub fn add_mine_field(&mut self, field: (u32, u32)) {
        if !self.mine_fields.contains(&field) {
            self.mine_fields.push(field);
        }
    }

    pub fn add_mine_fields(&mut self, fields: Vec<(u32, u32)>) {
        for field in fields {
            self.add_mine_field(field);
        }
    }

    pub fn add_recursive_informations(&mut self, fields: Vec<Vec<(u32, u32)>>) {
        for i in 0..fields.len() {
            if self.recursive_informations.len() <= i {
                self.recursive_informations.push(Vec::new());
            }

            // Avoid adding fields which are already known as safe or were revealed in previous steps
            'field_loop: for field in fields[i].iter() {
                if self.safe_fields.contains(field) {
                    continue;
                }
                for j in 0..=i {
                    if self.recursive_informations[j].contains(field) {
                        continue 'field_loop;
                    }
                }
                self.recursive_informations[i].push(*field);
            }
        }
    }

    pub fn get_safe_fields(&self) -> &Vec<(u32, u32)> {
        &self.safe_fields
    }

    pub fn get_mine_fields(&self) -> &Vec<(u32, u32)> {
        &self.mine_fields
    }
}