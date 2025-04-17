use super::minesweeper_cell::MineSweeperCell;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct MineSweeperField {
    pub width: usize,
    pub height: usize,
    pub mines: u64,
    pub start_field: (usize, usize),
    pub board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField {
    pub fn new_field(width: usize, height: usize, mines: u64) -> Self {
        let percentage = mines as f32 / (width * height) as f32;

        Self::new(width, height, percentage)
    }

    pub fn new(width: usize, height: usize, percentage: f32) -> Self {
        let board = vec![vec![MineSweeperCell::Empty; height]; width];

        if percentage > 1.0 {
            panic!("Too many mines for the given board size!");
        }

        if percentage < 0.0 {
            panic!("Negative percentage of mines!");
        }

        if percentage > 0.3 {
            println!("Warning: {}% of the fields are mines!", percentage * 100.0);
        }

        let mines = ((width * height) as f32 * percentage) as u64;

        let mut field = MineSweeperField{
            width,
            height,
            mines,
            board,
            start_field: (0, 0),
        };

        field.place_mines();
        field.initialize();
        field
    }

    pub fn initialize(&mut self) {
        self.assign_numbers();
        self.set_start_field();
    }

    pub fn print(&self) {
        println!("Width: {}, Height: {}, Mines: {}", self.width, self.height, self.mines);
        for (x, y) in self.sorted_fields() {
            print!("{} ", &self.board[x][y].get_colored());

            if x == self.width - 1 {
                println!();
            }
        }
    }

    fn assign_numbers(&mut self) {
        for (x, y) in self.sorted_fields() {
            if self.board[x][y] == MineSweeperCell::Mine {
                continue;
            }

            let count = self.get_sourrounding_mine_count(x, y);
            if count != 0 {
                self.board[x][y] = MineSweeperCell::Number(count);
            }
        }
    }

    fn set_start_field(&mut self) {
        // Set the start field to the first empty cell found
        // Can later also be set to a random empty cell
        for (x, y) in self.sorted_fields() {
            if self.board[x][y] == MineSweeperCell::Empty {
                self.start_field = (x, y);
                return;
            }
        }
    }

    fn place_mines(&mut self) {
        let mut placed_mines = 0;

        if self.mines >= (self.width * self.height) as u64 {
            panic!("Too many mines for the given board size!");
        }

        /*
        Currently for testing purposes, but in the future a random seed will be used
        */
        let seed: u64 = 40;
        let mut rng = StdRng::seed_from_u64(seed);

        while placed_mines < self.mines {
            let x = (rng.random_range(0..u64::MAX) % self.width as u64 ) as usize;
            let y = (rng.random_range(0..u64::MAX) % self.height as u64 ) as usize;

            if self.board[x][y] == MineSweeperCell::Empty {
                self.board[x][y] = MineSweeperCell::Mine;
                placed_mines += 1;
            }
        }
    }

    fn get_sourrounding_mine_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;

                if nx >= 0 && ny >= 0
                && nx < self.width as i64
                && ny < self.height as i64
                && self.board[nx as usize][ny as usize] == MineSweeperCell::Mine {
                    count += 1;
                }
            }
        }
        count
    }
}