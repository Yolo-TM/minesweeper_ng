use colored::{ColoredString, Colorize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone, PartialEq)]
pub enum MineSweeperCell {
    Empty,
    Mine,
    Number(u8),
}

#[derive(Clone, PartialEq)]
pub enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, PartialEq)]
pub struct MineSweeperField {
    pub width: u64,
    pub height: u64,
    pub mines: u64,
    pub board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField {
    pub fn new(width: u64, height: u64, mines: u64) -> Self {
        let board = vec![vec![MineSweeperCell::Empty; width as usize]; height as usize];

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

    pub fn new_percentage(width: u64, height: u64, mines: f32) -> Self {
        let mines = ((width * height) as f32 * mines).floor() as u64;
        Self::new(width, height, mines)
    }

    pub fn println(&self) {
        println!("Width: {}, Height: {}, Mines: {}", self.width, self.height, self.mines);
        for row in &self.board {
            for cell in row {
                print!("{} ", self.get_colored_cell(cell));
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
        // Randomly place mines on the board
        let mut placed_mines = 0;

        if self.mines >= self.width * self.height {
            panic!("Too many mines for the given board size!");
        }

        /*
        Currently for testing purposes, but in the future a random seed will be used
        */
        let seed: u64 = 40;
        let mut rng = StdRng::seed_from_u64(seed);

        while placed_mines < self.mines {
            let x = rng.random_range(0..u64::MAX) % self.width;
            let y = rng.random_range(0..u64::MAX) % self.height;

            if self.board[y as usize][x as usize] == MineSweeperCell::Empty {
                self.board[y as usize][x as usize] = MineSweeperCell::Mine;
                placed_mines += 1;
            }
        }
    }

    fn calculate_numbers(&mut self) {
        // Calculate the numbers for each cell based on the mines around it
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y as usize][x as usize] == MineSweeperCell::Mine {
                    continue; // Skip mines
                }

                let mut count = 0;
                self.count_sourrounding_mines(&mut count, x, y);
                if count != 0 {
                    // Only set the cell if there are adjacent mines
                    self.board[y as usize][x as usize] = MineSweeperCell::Number(count as u8);
                }
            }
        }
    }

    fn count_sourrounding_mines(&self, count: &mut u64, x: u64, y: u64) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;

                if nx >= 0
                && ny >= 0
                && nx < self.width as i64
                && ny < self.height as i64
                && self.board[ny as usize][nx as usize] == MineSweeperCell::Mine {
                    *count += 1;
                }
            }
        }
    }
}

pub fn get_ng_minesweeper_field() -> MineSweeperField {
    let board = vec![vec![MineSweeperCell::Empty; 10 as usize]; 10 as usize];

    let mut field = MineSweeperField{
        width: 9,
        height: 9,
        mines: 10,
        board,
    };

    let mine_positions = vec![
        (0, 0), (0, 1), (1, 1), (4, 0), (3, 2),
        (3, 3), (0, 5), (8, 5), (4, 7), (3, 8),
    ];

    for &(x, y) in &mine_positions {
        field.board[y as usize][x as usize] = MineSweeperCell::Mine;
    }

    field.calculate_numbers();
    field
}

impl MineSweeperCell {
    pub fn is_empty(&self) -> bool {
        matches!(self, MineSweeperCell::Empty)
    }

    pub fn is_mine(&self) -> bool {
        matches!(self, MineSweeperCell::Mine)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, MineSweeperCell::Number(_))
    }

    pub fn get_number(&self) -> Option<u8> {
        if let MineSweeperCell::Number(num) = self {
            Some(*num)
        } else {
            None
        }
    }
}