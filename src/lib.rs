pub mod field_generator;
pub mod ng_generator;
pub mod minesweeper_solver;

// Re-export commonly used types for cleaner imports
pub use field_generator::{MineSweeperField, MineSweeperFieldCreation, minesweeper_field};
pub use ng_generator::minesweeper_ng_field;
