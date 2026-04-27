use super::Solver;
use crate::minesweeper_field::{SortedCells, SurroundingCells};

impl Solver {
    pub(crate) fn format_field_state(&self) -> String {
        let mut output = String::new();

        output.push_str("\n╔═");
        for _ in 0..self.width {
            output.push_str("══");
        }
        output.push_str("╗\n");

        output.push('║');
        for (x, y) in self.sorted_fields() {
            output.push_str(&format!(" {}", self.get_state(x, y).get_colored()));

            if x == self.width - 1 {
                output.push_str(" ║\n");

                if y != self.height - 1 {
                    output.push('║');
                }
            }
        }

        output.push_str("╚═");
        for _ in 0..self.width {
            output.push_str("══");
        }
        output.push_str("╝");

        output
    }

    pub(super) fn sorted_fields(&self) -> SortedCells {
        SortedCells {
            width: self.width,
            height: self.height,
            current_x: 0,
            current_y: 0,
        }
    }

    pub(super) fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingCells {
        let range = range.unwrap_or(1);
        SurroundingCells {
            x,
            y,
            width: self.width,
            height: self.height,
            range,
            dx: -(range as i8),
            dy: -(range as i8),
        }
    }
}
