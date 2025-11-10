
pub struct Finding {
    safe_fields: Vec<(u32, u32)>,
    mine_fields: Vec<(u32, u32)>,
}

impl Finding {
    pub fn new() -> Self {
        Finding {
            safe_fields: Vec::new(),
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

    pub fn get_safe_fields(&self) -> &Vec<(u32, u32)> {
        &self.safe_fields
    }

    pub fn get_mine_fields(&self) -> &Vec<(u32, u32)> {
        &self.mine_fields
    }
}