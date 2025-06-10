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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_step_to_string() {
        assert_eq!(SolverStep::Basic.to_string(), "Basic Count");
        assert_eq!(SolverStep::Reduction.to_string(), "Basic Reduction");
        assert_eq!(SolverStep::Complex.to_string(), "Extended Reduction");
        assert_eq!(SolverStep::Permutations.to_string(), "Permutations");
    }

    #[test]
    fn test_solver_step_to_number() {
        assert_eq!(SolverStep::Basic.to_number(), 1);
        assert_eq!(SolverStep::Reduction.to_number(), 2);
        assert_eq!(SolverStep::Complex.to_number(), 3);
        assert_eq!(SolverStep::Permutations.to_number(), 4);
    }

    #[test]
    fn test_solver_step_counter_new() {
        let counter = SolverStepCounter::new();
        assert_eq!(counter.get_steps(), 0);
        assert_eq!(counter.get_count(SolverStep::Basic), 0);
        assert_eq!(counter.get_count(SolverStep::Reduction), 0);
        assert_eq!(counter.get_count(SolverStep::Complex), 0);
        assert_eq!(counter.get_count(SolverStep::Permutations), 0);
    }

    #[test]
    fn test_increase_steps() {
        let mut counter = SolverStepCounter::new();
        
        assert_eq!(counter.get_steps(), 0);
        
        counter.increase_steps();
        assert_eq!(counter.get_steps(), 1);
        
        counter.increase_steps();
        counter.increase_steps();
        assert_eq!(counter.get_steps(), 3);
    }

    #[test]
    fn test_get_pretty_steps() {
        let mut counter = SolverStepCounter::new();
        
        // Test with no steps
        assert_eq!(counter.get_pretty_steps(), "No steps taken");
        
        // Test with steps
        counter.increase_steps();
        counter.increase_steps();
        // Note: The actual output will have color codes, but we can't easily test them
        assert!(counter.get_pretty_steps().contains("2"));
    }

    #[test]
    fn test_increment_single_step() {
        let mut counter = SolverStepCounter::new();
        
        counter.increment(SolverStep::Basic);
        assert_eq!(counter.get_count(SolverStep::Basic), 1);
        assert_eq!(counter.get_count(SolverStep::Reduction), 0);
        
        counter.increment(SolverStep::Basic);
        assert_eq!(counter.get_count(SolverStep::Basic), 2);
    }

    #[test]
    fn test_increment_multiple_steps() {
        let mut counter = SolverStepCounter::new();
        
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Reduction);
        counter.increment(SolverStep::Complex);
        counter.increment(SolverStep::Permutations);
        
        assert_eq!(counter.get_count(SolverStep::Basic), 1);
        assert_eq!(counter.get_count(SolverStep::Reduction), 1);
        assert_eq!(counter.get_count(SolverStep::Complex), 1);
        assert_eq!(counter.get_count(SolverStep::Permutations), 1);
    }

    #[test]
    fn test_get_count_nonexistent() {
        let counter = SolverStepCounter::new();
        assert_eq!(counter.get_count(SolverStep::Basic), 0);
        assert_eq!(counter.get_count(SolverStep::Reduction), 0);
        assert_eq!(counter.get_count(SolverStep::Complex), 0);
        assert_eq!(counter.get_count(SolverStep::Permutations), 0);
    }

    #[test]
    fn test_get_complexity_no_steps() {
        let counter = SolverStepCounter::new();
        assert_eq!(counter.get_complexity(), "No steps taken");
    }

    #[test]
    fn test_get_complexity_single_step() {
        let mut counter = SolverStepCounter::new();
        counter.increment(SolverStep::Basic);
        
        let complexity = counter.get_complexity();
        assert!(complexity.contains("Basic Count"));
        assert!(complexity.contains("1"));
    }

    #[test]
    fn test_get_complexity_multiple_steps() {
        let mut counter = SolverStepCounter::new();
        counter.increment(SolverStep::Complex);
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Permutations);
        
        let complexity = counter.get_complexity();
        
        // Should contain all step types that were incremented
        assert!(complexity.contains("Basic Count"));
        assert!(complexity.contains("Extended Reduction"));
        assert!(complexity.contains("Permutations"));
        
        // Should not contain Reduction since it wasn't incremented
        assert!(!complexity.contains("Basic Reduction"));
        
        // Should contain correct counts
        assert!(complexity.contains("2")); // Basic count
        assert!(complexity.contains("1")); // Complex and Permutations count
    }

    #[test]
    fn test_get_complexity_ordering() {
        let mut counter = SolverStepCounter::new();
        
        // Add steps in reverse order to test sorting
        counter.increment(SolverStep::Permutations);
        counter.increment(SolverStep::Complex);
        counter.increment(SolverStep::Reduction);
        counter.increment(SolverStep::Basic);
        
        let complexity = counter.get_complexity();
        
        // Find positions of each step type in the string
        let basic_pos = complexity.find("Basic Count").unwrap();
        let reduction_pos = complexity.find("Basic Reduction").unwrap();
        let complex_pos = complexity.find("Extended Reduction").unwrap();
        let permutations_pos = complexity.find("Permutations").unwrap();
        
        // They should appear in order of their to_number() values
        assert!(basic_pos < reduction_pos);
        assert!(reduction_pos < complex_pos);
        assert!(complex_pos < permutations_pos);
    }

    #[test]
    fn test_get_average_no_steps() {
        let counter = SolverStepCounter::new();
        assert_eq!(counter.get_average(), 0.0);
    }

    #[test]
    fn test_get_average_single_step_type() {
        let mut counter = SolverStepCounter::new();
        
        // Add 3 basic steps
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Basic);
        counter.increase_steps();
        counter.increase_steps();
        counter.increase_steps();
        
        // Average should be: (3 * 1) / 3 = 1.0
        assert_eq!(counter.get_average(), 1.0);
    }

    #[test]
    fn test_get_average_multiple_step_types() {
        let mut counter = SolverStepCounter::new();
        
        // Add: 2 Basic (value 1), 1 Complex (value 3)
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Complex);
        
        // Set steps to 3
        counter.increase_steps();
        counter.increase_steps();
        counter.increase_steps();
        
        // Average should be: (2 * 1 + 1 * 3) / 3 = 5 / 3 ≈ 1.6667
        let expected = (2.0 * 1.0 + 1.0 * 3.0) / 3.0;
        assert!((counter.get_average() - expected).abs() < 0.0001);
    }

    #[test]
    fn test_get_average_complex_scenario() {
        let mut counter = SolverStepCounter::new();
        
        // Add various steps
        counter.increment(SolverStep::Basic);      // 1
        counter.increment(SolverStep::Reduction);  // 2
        counter.increment(SolverStep::Reduction);  // 2
        counter.increment(SolverStep::Complex);    // 3
        counter.increment(SolverStep::Permutations); // 4
        
        // Set steps to 5
        for _ in 0..5 {
            counter.increase_steps();
        }
        
        // Average should be: (1*1 + 2*2 + 1*3 + 1*4) / 5 = (1 + 4 + 3 + 4) / 5 = 12 / 5 = 2.4
        let expected = (1.0 * 1.0 + 2.0 * 2.0 + 1.0 * 3.0 + 1.0 * 4.0) / 5.0;
        assert!((counter.get_average() - expected).abs() < 0.0001);
    }

    #[test]
    fn test_clone_functionality() {
        let mut counter = SolverStepCounter::new();
        counter.increment(SolverStep::Basic);
        counter.increment(SolverStep::Complex);
        counter.increase_steps();
        counter.increase_steps();
        
        let cloned = counter.clone();
        
        assert_eq!(counter.get_steps(), cloned.get_steps());
        assert_eq!(counter.get_count(SolverStep::Basic), cloned.get_count(SolverStep::Basic));
        assert_eq!(counter.get_count(SolverStep::Complex), cloned.get_count(SolverStep::Complex));
        assert_eq!(counter.get_average(), cloned.get_average());
    }

    #[test]
    fn test_debug_formatting() {
        let mut counter = SolverStepCounter::new();
        counter.increment(SolverStep::Basic);
        
        // Test that Debug formatting works (doesn't panic)
        let debug_str = format!("{:?}", counter);
        assert!(debug_str.contains("SolverStepCounter"));
    }

    #[test]
    fn test_equality() {
        let mut counter1 = SolverStepCounter::new();
        let mut counter2 = SolverStepCounter::new();
        
        assert_eq!(counter1, counter2);
        
        counter1.increment(SolverStep::Basic);
        assert_ne!(counter1, counter2);
        
        counter2.increment(SolverStep::Basic);
        assert_eq!(counter1, counter2);
        
        counter1.increase_steps();
        assert_ne!(counter1, counter2);
        
        counter2.increase_steps();
        assert_eq!(counter1, counter2);
    }
}