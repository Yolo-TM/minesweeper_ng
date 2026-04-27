mod cell;
mod defined_field;
mod display;
mod error;
mod file_io;
mod iterators;
mod mines;
mod random_field;
mod r#trait;

#[cfg(feature = "json")]
mod json_io;
#[cfg(feature = "svg")]
mod svg;

pub use cell::Cell;
pub use defined_field::DefinedField;
pub use display::MineSweeperFieldDisplay;
pub use error::FieldError;
pub use file_io::MineSweeperFieldFileIO;
pub use iterators::{SortedCells, SurroundingCells};
pub use mines::Mines;
pub use random_field::RandomField;
pub use r#trait::MineSweeperField;

#[cfg(feature = "json")]
pub use json_io::MineSweeperFieldJson;
#[cfg(feature = "svg")]
pub use svg::{MineSweeperFieldSvg, SVG_Mode};
