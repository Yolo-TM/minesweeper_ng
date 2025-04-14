use colored::{ColoredString, Colorize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone, PartialEq)]
pub enum MineSweeperCell {
    Empty,
    Mine,
    Number(u8),
}

impl MineSweeperCell {
    pub fn get_number(&self) -> u8 {
        match self {
            MineSweeperCell::Empty => 0,
            MineSweeperCell::Mine => 9,
            MineSweeperCell::Number(num) => *num,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MineSweeperField {
    pub width: usize,
    pub height: usize,
    pub mines: u64,
    pub board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField {
    pub fn new(width: usize, height: usize, mines: u64) -> Self {
        let board = vec![vec![MineSweeperCell::Empty; height as usize]; width as usize];

        let mut field = MineSweeperField{
            width,
            height,
            mines,
            board,
        };

        field.create_mines();
        field.calculate_numbers();
        field
    }

    pub fn new_percentage(width: usize, height: usize, mines: f32) -> Self {
        let mines = ((width * height) as f32 * mines).floor() as u64;
        Self::new(width, height, mines)
    }

    pub fn println(&self) {
        println!("Width: {}, Height: {}, Mines: {}", self.width, self.height, self.mines);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{} ", self.get_colored_cell(&self.board[x][y]));
            }
            println!();
        }
    }

    fn get_colored_cell(&self, cell: &MineSweeperCell) -> ColoredString {
        match cell {
            MineSweeperCell::Empty => " ".white(),
            MineSweeperCell::Number(num) => self.get_colored_number(num),
            MineSweeperCell::Mine => "#".black().bold(),
        }
    }

    pub fn get_colored_number(&self, num: &u8) -> ColoredString {
        match num {
            1 => "1".bright_blue(),
            2 => "2".green(),
            3 => "3".bright_red(),
            4 => "4".bright_purple(),
            5 => "5".yellow(),
            6 => "6".cyan(),
            7 => "7".black(),
            8 => "8".white(),
            _ => unreachable!(),
        }
    }

    fn create_mines(&mut self) {
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

    fn calculate_numbers(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.board[x][y] == MineSweeperCell::Mine {
                    continue;
                }

                let mut count = 0;
                self.count_sourrounding_mines(&mut count, x, y);
                if count != 0 {
                    self.board[x][y] = MineSweeperCell::Number(count as u8);
                }
            }
        }
    }

    fn count_sourrounding_mines(&self, count: &mut u64, x: usize, y: usize) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;

                if nx >= 0
                && ny >= 0
                && nx < self.width as i64
                && ny < self.height as i64
                && self.board[nx as usize][ny as usize] == MineSweeperCell::Mine {
                    *count += 1;
                }
            }
        }
    }
}

pub fn get_ng_minesweeper_field() -> MineSweeperField {
    let board = vec![vec![MineSweeperCell::Empty; 10 as usize]; 10 as usize];

    let mut field = MineSweeperField{
        width: 10,
        height: 10,
        mines: 10,
        board,
    };

    let mine_positions: Vec<(usize, usize)> = vec![
        (0, 0), (1, 0), (1, 1), (0, 4), (2, 3),
        (3, 3), (5, 0), (5, 8), (7, 4), (8, 3),
    ];

    for &(x, y) in &mine_positions {
        field.board[x][y] = MineSweeperCell::Mine;
    }

    field.calculate_numbers();
    field
}