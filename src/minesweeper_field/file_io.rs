use super::{Cell, MineSweeperField};
use std::fs::File;
use std::io::Write;

pub trait MineSweeperFieldFileIO: MineSweeperField {
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
        let mut bit_count: u8 = 0;

        for (x, y) in self.sorted_fields() {
            let is_mine = if self.get_cell(x, y) == &Cell::Mine {
                1
            } else {
                0
            };

            current_byte |= is_mine << (7 - bit_count);
            bit_count += 1;

            if bit_count == 8 {
                bits.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        if bit_count > 0 {
            bits.push(current_byte);
        }

        file.write_all(&bits)?;

        Ok(())
    }
}

impl<T: MineSweeperField> MineSweeperFieldFileIO for T {}
