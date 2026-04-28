use super::generator;
use crate::solver::create_solver;
use crate::{Cell, DefinedField, FieldError, MineSweeperField, Mines};

const DEFAULT_BATCH_SIZE: usize = 20;

/// A minesweeper field guaranteed to be deterministically solvable without guessing.
#[derive(Clone)]
pub struct NoGuessField(pub(super) DefinedField);

impl MineSweeperField for NoGuessField {
    fn get_mines(&self) -> u32 {
        self.0.get_mines()
    }
    fn get_width(&self) -> u32 {
        self.0.get_width()
    }
    fn get_height(&self) -> u32 {
        self.0.get_height()
    }
    fn get_start_cell(&self) -> (u32, u32) {
        self.0.get_start_cell()
    }
    fn get_cell(&self, x: u32, y: u32) -> &Cell {
        self.0.get_cell(x, y)
    }
    fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        self.0.set_cell(x, y, cell)
    }
}

impl NoGuessField {
    pub fn new(width: u32, height: u32, mines: Mines) -> Result<Self, FieldError> {
        generator::generate(width, height, mines, DEFAULT_BATCH_SIZE)
    }

    pub fn from_file(file_path: &str) -> Result<NoGuessField, FieldError> {
        let field = DefinedField::from_file(file_path)?;
        let mut solver = create_solver(&field);
        solver.solve();
        if !solver.is_solved() {
            return Err(FieldError::InvalidFileData(
                "field requires guessing and is not a valid no-guess field".into(),
            ));
        }
        Ok(NoGuessField(field))
    }
}
