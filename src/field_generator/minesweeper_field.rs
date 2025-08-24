use std::fs::File;
use std::io::{Read, Write};
use serde_json::Value;
use super::{
    MineField,
    MineSweeperCell,
    MineSweeperFieldCreation,
    MineSweeperFieldIterator,
    SurroundingFieldsIterator,
};

pub trait MineSweeperField: Sync + Send + Clone + 'static {
    #[track_caller]
    fn new(width: u32, height: u32, mines: MineSweeperFieldCreation) -> Self;

    fn get_mines(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_start_cell(&self) -> (u32, u32);
    fn get_field(&self) -> Vec<Vec<MineSweeperCell>>;

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell;
    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell);

    fn show(&self) {
        let (w, h, m) = self.get_dimensions();
        println!("Width: {}, Height: {}, Mines: {}", w, h, m);
        println!("Start field: {:?}", self.get_start_cell());

        print!("╔═");
        for _ in 0..self.get_width() {
            print!("══");
        }
        println!("╗");

        print!("║");
        for (x, y) in self.sorted_fields() {
            print!(" {}", &self.get_cell(x, y).get_colored());

            if x == self.get_width() - 1 {
                print!(" ║");
                println!();

                if y != self.get_height() - 1 {
                    print!("║");
                }
            }
        }

        print!("╚═");
        for _ in 0..self.get_width() {
            print!("══");
        }
        println!("╝");
    }

    fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.get_width(), self.get_height(), self.get_mines())
    }

    fn assign_numbers(&mut self) {
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == MineSweeperCell::Mine {
                continue;
            }

            let count = self.get_sourrounding_mine_count(x, y);
            if count != 0 {
                self.set_cell(x, y, MineSweeperCell::Number(count));
            }
        }
    }

    fn get_sourrounding_mine_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for (x, y) in self.surrounding_fields(x, y, None) {
            if self.get_cell(x, y) == MineSweeperCell::Mine {
                count += 1;
            }
        }
        count
    }

    fn sorted_fields(&self) -> MineSweeperFieldIterator {
        MineSweeperFieldIterator {
            width: self.get_width(),
            height: self.get_height(),
            current_x: 0,
            current_y: 0,
        }
    }

    fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingFieldsIterator {
        let range = range.unwrap_or(1);

        SurroundingFieldsIterator {
            x,
            y,
            width: self.get_width(),
            height: self.get_height(),
            range,
            dx: -(range as i8),
            dy: -(range as i8),
        }
    }

    fn as_json(&self) -> String {
        let mine_positions: Vec<(u32, u32)> = self.sorted_fields()
            .filter(|&(x, y)| self.get_cell(x, y) == MineSweeperCell::Mine)
            .collect();

        let json = serde_json::json!({
            "width": self.get_width(),
            "height": self.get_height(),
            "mines": self.get_mines(),
            "start_x": self.get_start_cell().0,
            "start_y": self.get_start_cell().1,
            "mine_positions": mine_positions
        });

        serde_json::to_string_pretty(&json).unwrap()
    }

    fn to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;

        let (w, h, m) = self.get_dimensions();
        file.write_all(&w.to_le_bytes())?;
        file.write_all(&h.to_le_bytes())?;
        file.write_all(&m.to_le_bytes())?;
        file.write_all(&self.get_start_cell().0.to_le_bytes())?;
        file.write_all(&self.get_start_cell().1.to_le_bytes())?;

        let mut bits: Vec<u8> = Vec::new();
        let mut current_byte: u8 = 0u8;
        let mut bit_count: u8  = 0;

        for (x, y) in self.sorted_fields() {
            let is_mine = if self.get_cell(x, y) == MineSweeperCell::Mine { 1 } else { 0 };

            current_byte |= is_mine << (7 - bit_count);
            bit_count += 1;

            if bit_count == 8 {
                bits.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        // byte not full, push remaining
        if bit_count > 0 {
            bits.push(current_byte);
        }

        file.write_all(&bits)?;

        Ok(())
    }

    fn from_json(json: &str) -> Option<impl MineSweeperField> {
        let parsed: Value = serde_json::from_str(json).ok()?;

        let width = parsed["width"].as_u64()? as u32;
        let height = parsed["height"].as_u64()? as u32;
        let mines = parsed["mines"].as_u64()? as u32;
        let start_x = parsed["start_x"].as_u64()? as u32;
        let start_y = parsed["start_y"].as_u64()? as u32;


        let mut mine_array = vec![];
        if let Some(mine_positions) = parsed["mine_positions"].as_array() {
            for position in mine_positions {
                if let Some((x, y)) = position.as_array()
                    .and_then(|arr| Some((arr[0].as_u64()? as u32, arr[1].as_u64()? as u32)))
                {
                    mine_array.push((x, y));
                }
            }
        }

        let mut field = MineField::new(width, height, MineSweeperFieldCreation::FixedCount(mines));
        field.initialize(mine_array);
        field.set_start_cell(start_x, start_y);

        Some(field)
    }
    
    fn from_file(file_path: &str) -> std::io::Result<impl MineSweeperField> {
        let mut file = File::open(file_path)?;
        let mut buffer = [0u8; 20]; // buffer for 5 u32 integers

        file.read_exact(&mut buffer)?;

        let width = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let height = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        let mines = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        let start_x = u32::from_le_bytes(buffer[12..16].try_into().unwrap());
        let start_y = u32::from_le_bytes(buffer[16..20].try_into().unwrap());

        if start_x >= width || start_y >= height {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Start field out of bounds.",
            ));
        }
        
        let mut field = MineField::new(width, height, MineSweeperFieldCreation::FixedCount(mines));
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
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Mine position out of bounds.",
                        ));
                    }
                    mine_positions.push((x, y));
                }
            }
        }

        if mine_positions.len() != mines as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Number of mines does not match the expected count.",
            ));
        }

        field.initialize(mine_positions);
        field.assign_numbers();
        Ok(field)
    }
}