use std::collections::HashMap;
use colored::Colorize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SolverStep {
    Basic,
    Reduction,
    Complex,
    Permutations
}

impl SolverStep {
    pub fn to_string(&self) -> String {
        match self {
            SolverStep::Basic => "Basic Count".to_string(),
            SolverStep::Reduction => "Basic Reduction".to_string(),
            SolverStep::Complex => "Extended Reduction".to_string(),
            SolverStep::Permutations => "Permutations".to_string(),
        }
    }

    pub fn to_number(&self) -> u8 {
        match self {
            SolverStep::Basic => 1,
            SolverStep::Reduction => 2,
            SolverStep::Complex => 3,
            SolverStep::Permutations => 4,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SolverStepCounter {
    // One for each SolverStep variant, needs to be updated
    counters: HashMap<SolverStep, u32>,
    steps: u32
}

impl SolverStepCounter {
    pub fn new() -> Self {
        SolverStepCounter {
            counters: HashMap::new(),
            steps: 0
        }
    }

    pub fn increase_steps(&mut self) {
        self.steps += 1;
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn get_pretty_steps(&self) -> String {
        if self.steps == 0 {
            return "No steps taken".to_string();
        }

        format!("{}", self.steps.to_string().blue())
    }

    pub fn increment(&mut self, step: SolverStep) {
        let count = self.counters.entry(step).or_insert(0);
        *count += 1;
    }

    pub fn get_count(&self, step: SolverStep) -> u32 {
        *self.counters.get(&step).unwrap_or(&0)
    }

    pub fn get_complexity(&self) -> String {
        let mut complexity_str = String::new();
        let mut existing_steps: Vec<SolverStep> = self.counters.keys().cloned().collect();

        if existing_steps.is_empty() {
            return "No steps taken".to_string();
        }

        existing_steps.sort_by(|a, b| a.to_number().cmp(&b.to_number()));

        for step in existing_steps {
            let count = self.get_count(step.clone());
            if count > 0 {
                if !complexity_str.is_empty() {
                    complexity_str.push_str(", ");
                }
                complexity_str.push_str(&format!("{}: {}", step.to_string(), count.to_string().blue()));
            }
        }

        complexity_str
    }

    pub fn get_average(&self) -> f64 {
        if self.steps == 0 {
            return 0.0;
        }

        let mut average: f64 = 0.0;
        for (step, count) in &self.counters {
            average += *count as f64 * step.to_number() as f64;
        }
        average /= self.steps as f64;

        average
    }
}