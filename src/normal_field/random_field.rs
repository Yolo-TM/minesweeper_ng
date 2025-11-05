use super::{Cell, Mines, MineSweeperField};
use rand::Rng;

#[derive(Clone)]
pub struct RandomField {
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),
    board: Vec<Vec<Cell>>,
}

impl MineSweeperField for RandomField {

    #[track_caller]
    fn new(width: u32, height: u32, mines: Mines) -> Self {
        if !mines.is_valid(width, height) {
            panic!("Invalid mine configuration!");
        }

        let percentage = mines.get_percentage(width, height);
        if percentage > 0.25 {
            println!("Warning: {}% of the fields are mines!", percentage * 100.0);
        }

        let board = vec![vec![Cell::Empty; height as usize]; width as usize];
        let mines = mines.get_fixed_count(width, height);

        let mut field = RandomField {
            width,
            height,
            mines,
            board,
            start_cell: (0, 0),
        };

        field.initialize();
        field
    }

    fn get_mines(&self) -> u32 {
        self.mines
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_start_cell(&self) -> (u32, u32) {
        self.start_cell
    }

    fn get_field(&self) -> Vec<Vec<Cell>> {
        self.board.clone()
    }

    fn get_cell(&self, x: u32, y: u32) -> Cell {
        self.board[x as usize][y as usize].clone()
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        self.board[x as usize][y as usize] = cell;
    }

}

impl RandomField {

    fn initialize(&mut self) {
        self.place_mines();
        self.assign_numbers();
        self.set_start_cell();
    }

    fn set_start_cell(&mut self) {
        let mut start_cell_candidates = vec![];
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == Cell::Empty {
                start_cell_candidates.push((x, y));
            }
        }

        // In the rare case there are no empty cells, fall back to any non-mine cell
        if start_cell_candidates.is_empty() {
            for (x, y) in self.sorted_fields() {
                if self.get_cell(x, y) != Cell::Mine {
                    start_cell_candidates.push((x, y));
                }
            }
        }

        // By now there is definitely at least one candidate, since the percentage is limited to .9 in the Mines struct (Mines.is_valid)

        let index = rand::rng().random_range(0..start_cell_candidates.len());
        self.start_cell = start_cell_candidates[index];
    }

    fn place_mines(&mut self) {
        let mut placed_mines = 0;
        let mut rng = rand::rng();

        while placed_mines < self.mines {
            let x = (rng.random_range(0..u64::MAX) % self.width as u64 ) as u32;
            let y = (rng.random_range(0..u64::MAX) % self.height as u64 ) as u32;

            if self.get_cell(x, y) == Cell::Empty {
                self.set_cell(x, y, Cell::Mine);
                placed_mines += 1;
            }
        }
    }
}