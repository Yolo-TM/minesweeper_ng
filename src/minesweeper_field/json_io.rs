use super::error::FieldError;
use super::{Cell, DefinedField, MineSweeperField, Mines};
use serde_json::Value;

pub trait MineSweeperFieldJson: MineSweeperField {
    fn as_json(&self) -> String {
        let mine_positions: Vec<(u32, u32)> = self
            .sorted_fields()
            .filter(|&(x, y)| self.get_cell(x, y) == &Cell::Mine)
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

    fn from_json(json: &str) -> Result<impl MineSweeperField, FieldError> {
        let parsed: Value = serde_json::from_str(json)
            .map_err(|e| FieldError::SerializationError(e.to_string()))?;

        let width = parsed["width"]
            .as_u64()
            .ok_or_else(|| FieldError::InvalidFileData("missing 'width'".into()))?
            as u32;
        let height = parsed["height"]
            .as_u64()
            .ok_or_else(|| FieldError::InvalidFileData("missing 'height'".into()))?
            as u32;
        let mines = parsed["mines"]
            .as_u64()
            .ok_or_else(|| FieldError::InvalidFileData("missing 'mines'".into()))?
            as u32;
        let start_x = parsed["start_x"]
            .as_u64()
            .ok_or_else(|| FieldError::InvalidFileData("missing 'start_x'".into()))?
            as u32;
        let start_y = parsed["start_y"]
            .as_u64()
            .ok_or_else(|| FieldError::InvalidFileData("missing 'start_y'".into()))?
            as u32;

        if start_x >= width || start_y >= height {
            return Err(FieldError::OutOfBounds {
                x: start_x,
                y: start_y,
                width,
                height,
            });
        }

        let mut mine_array = vec![];
        if let Some(mine_positions) = parsed["mine_positions"].as_array() {
            for position in mine_positions {
                if let Some((x, y)) = position
                    .as_array()
                    .and_then(|arr| Some((arr[0].as_u64()? as u32, arr[1].as_u64()? as u32)))
                {
                    mine_array.push((x, y));
                }
            }
        }

        if mine_array.len() != mines as usize {
            return Err(FieldError::InvalidFileData(format!(
                "expected {} mines but found {} positions",
                mines,
                mine_array.len()
            )));
        }

        let mut field = DefinedField::new(width, height, Mines::Count(mines))?;
        field.initialize(mine_array);
        field.set_start_cell(start_x, start_y);

        Ok(field)
    }
}

impl<T: MineSweeperField> MineSweeperFieldJson for T {}
