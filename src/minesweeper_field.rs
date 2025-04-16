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
    let width = 30;
    let height = 20;
    let board = vec![vec![MineSweeperCell::Empty; height as usize]; width as usize];

    let mut field = MineSweeperField{
        width: width,
        height: height,
        mines: 130,
        board,
    };

    let mine_positions: Vec<(usize, usize)> = vec![
        (0,2), (0,3), (0,5), (0,17),
        (1,3), (1,5), (1,7), (1,16), (1,17),
        (2,3), (2,5), (2,7), (2,18),
        (3,1), (3,14),
        (4,9), (4,12), (4,17), (4,18),
        (5,1), (5,2), (5,3), (5,14),
        (6,2), (6,3), (6,5), (6,11), (6,13), (6,14), (6,16), (6,18),
        (7,0), (7,7), (7,12), (7,14),
        (8,4), (8,5), (8,13), (8,16),
        (9,0), (9,9), (9,17), (9,18),
        (10,0), (10,2), (10,4), (10,5), (10,15), (10,16), (10,18),
        (11,3), (11,6), (11,10), (11,15), (11,16), (11,18),
        (12,4), (12,16),
        (13,9), (13,12), (13,15), (13,18),
        (14,0), (14,4), (14,11), (14,13), (14,19),
        (15,2), (15,7), (15,10), (15,13), (15,15),
        (16,1),
        (17,5), (17,14), (17,17), (17,18),
        (18,4), (18,6), (18,10), (18,11),
        (19,2), (19,3), (19,9), (19,11), (19,12), (19,15), (19,17), (19,19),
        (20,4), (20,10),
        (21,5), (21,8),
        (22,1), (22,10), (22,11), (22,12),
        (23,2), (23,3), (23,6), (23,13), (23,17), (23,18),
        (24,0), (24,5), (24,7), (24,15),
        (25,9), (25,11), (25,15), (25,16), (25,19),
        (26,2), (26,5), (26,13), (26,15),
        (27,1), (27,3), (27,5), (27,10), (27,11), (27,17), (27,18),
        (28,2), (28,16), (28,19),
        (29,2), (29,10), (29,11), (29,16)
    ];

    for &(x, y) in &mine_positions {
        field.board[x][y] = MineSweeperCell::Mine;
    }

    field.calculate_numbers();
    field
}