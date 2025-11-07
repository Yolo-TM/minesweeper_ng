#[derive(Clone, Copy)]
pub enum Mines {
    Count(u32),
    Density(f32),
}

impl Mines {
    pub fn get_percentage(&self, w: u32, h: u32) -> f32 {
        match self {
            Mines::Count(count) => (*count as f32) / (w * h) as f32,
            Mines::Density(percentage) => *percentage,
        }
    }

    pub fn get_fixed_count(&self, w: u32, h: u32) -> u32 {
        match self {
            Mines::Count(count) => *count,
            Mines::Density(percentage) => ((w * h) as f32 * percentage) as u32,
        }
    }

    pub fn is_valid(&self, w: u32, h: u32) -> bool {
        let percentage = self.get_percentage(w, h);
        if percentage <= 0.0 || percentage >= 0.9 {
            return false;
        }

        let count = self.get_fixed_count(w, h);

        count < w * h
    }
}
