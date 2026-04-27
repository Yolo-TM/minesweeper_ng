use super::error::FieldError;
use super::{Cell, MineSweeperField, Mines};
use log::warn;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct DefinedField {
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),
    board: Vec<Vec<Cell>>,
}

impl MineSweeperField for DefinedField {
    fn get_mines(&self) -> u32 {
        self.mines
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_start_cell(&self) -> (u32, u32) {
        self.start_cell
    }

    fn get_cell(&self, x: u32, y: u32) -> &Cell {
        &self.board[x as usize][y as usize]
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        self.board[x as usize][y as usize] = cell;
    }
}

impl DefinedField {
    pub fn new(width: u32, height: u32, mines: Mines) -> Result<Self, FieldError> {
        if !mines.is_valid(width, height) {
            return Err(FieldError::InvalidMineConfig {
                reason: format!(
                    "{} mines on a {}x{} field is not valid",
                    mines.get_fixed_count(width, height),
                    width,
                    height
                ),
            });
        }

        let percentage = mines.get_percentage(width, height);
        if percentage > 0.25 {
            warn!("{}% of the cells are mines!", percentage * 100.0);
        }

        let board = vec![vec![Cell::Empty; height as usize]; width as usize];
        let mines = mines.get_fixed_count(width, height);

        Ok(DefinedField {
            width,
            height,
            mines,
            board,
            start_cell: (0, 0),
        })
    }

    pub fn initialize(&mut self, mine_positions: Vec<(u32, u32)>) {
        for &(x, y) in &mine_positions {
            self.set_cell(x, y, Cell::Mine);
        }

        self.assign_numbers();
    }

    pub fn set_start_cell(&mut self, x: u32, y: u32) {
        self.start_cell = (x, y);
    }

    pub fn place_mine(&mut self, x: u32, y: u32) {
        if !matches!(self.get_cell(x, y), Cell::Mine) {
            self.set_cell(x, y, Cell::Mine);
            self.mines += 1;
            self.assign_numbers();
        }
    }

    pub fn remove_mine(&mut self, x: u32, y: u32) {
        if matches!(self.get_cell(x, y), Cell::Mine) {
            self.set_cell(x, y, Cell::Empty);
            self.mines -= 1;
            self.assign_numbers();
        }
    }

    pub fn from_file(file_path: &str) -> Result<DefinedField, FieldError> {
        let mut file = File::open(file_path)?;
        let mut buffer = [0u8; 20];

        file.read_exact(&mut buffer)?;

        let width = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let height = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        let mines = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        let start_x = u32::from_le_bytes(buffer[12..16].try_into().unwrap());
        let start_y = u32::from_le_bytes(buffer[16..20].try_into().unwrap());

        if start_x >= width || start_y >= height {
            return Err(FieldError::OutOfBounds {
                x: start_x,
                y: start_y,
                width,
                height,
            });
        }

        let mut field = DefinedField::new(width, height, Mines::Count(mines))?;
        field.set_start_cell(start_x, start_y);

        let mut bits = vec![];
        file.read_to_end(&mut bits)?;

        let mut mine_positions = vec![];
        for (i, byte) in bits.iter().enumerate() {
            for bit in 0..8 {
                if (byte >> (7 - bit)) & 1 == 1 {
                    let x = (i * 8 + bit) as u32 % width;
                    let y = (i * 8 + bit) as u32 / width;

                    if x >= width || y >= height {
                        return Err(FieldError::OutOfBounds {
                            x,
                            y,
                            width,
                            height,
                        });
                    }
                    mine_positions.push((x, y));
                }
            }
        }

        if mine_positions.len() != mines as usize {
            return Err(FieldError::InvalidFileData(format!(
                "expected {} mines but found {}",
                mines,
                mine_positions.len()
            )));
        }

        field.initialize(mine_positions);
        Ok(field)
    }
}

