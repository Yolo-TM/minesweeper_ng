use super::Solver;
use crate::normal_field::{SortedCells, SurroundingCells};

impl Solver {

    pub(super) fn println(&self, message: &str, verbosity_level: u8) {
        if self.verbosity >= verbosity_level {
            println!("{}", message);
        }
    }

    pub(super) fn print_field(&self, verbosity_level: u8) {
        if self.verbosity >= verbosity_level {
            self.print_field_state();
        }
    }

    pub(super) fn print_field_state(&self) {
        print!("╔═");
        for _ in 0..self.width {
            print!("══");
        }
        println!("╗");

        print!("║");
        for (x, y) in self.sorted_fields() {
            print!(" {}", self.get_state(x, y).get_colored());

            if x == self.width - 1 {
                print!(" ║");
                println!();

                if y != self.height - 1 {
                    print!("║");
                }
            }
        }

        print!("╚═");
        for _ in 0..self.width {
            print!("══");
        }
        println!("╝");
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
        SurroundingCells { x, y, width: self.width, height: self.height, range, dx: -(range as i8), dy: -(range as i8) }
    }
}