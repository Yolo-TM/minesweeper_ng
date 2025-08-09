#[derive(Clone)]
pub enum MineSweeperFieldCreation {
    FixedCount(u32),
    Percentage(f32),
}

impl MineSweeperFieldCreation {
    pub fn get_percentage(&self, w: u32, h: u32) -> f32 {
        match self {
            MineSweeperFieldCreation::FixedCount(count) => (*count as f32) / (w * h) as f32,
            MineSweeperFieldCreation::Percentage(percentage) => *percentage,
        }
    }

    pub fn get_fixed_count(&self, w: u32, h: u32) -> u32 {
        match self {
            MineSweeperFieldCreation::FixedCount(count) => *count,
            MineSweeperFieldCreation::Percentage(percentage) => ((w * h) as f32 * percentage) as u32,
        }
    }
}