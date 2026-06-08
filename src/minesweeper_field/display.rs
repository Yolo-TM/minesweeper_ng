use super::{MineSweeperField, MineSweeperFieldMetrics};

pub trait MineSweeperFieldDisplay: MineSweeperField + MineSweeperFieldMetrics {
    fn show(&self) {
        let (w, h, m) = self.get_dimensions();
        println!("Width: {}, Height: {}, Mines: {}, 3BV: {}", w, h, m, self.get_3bv());
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
}

impl<T: MineSweeperField> MineSweeperFieldDisplay for T {}
