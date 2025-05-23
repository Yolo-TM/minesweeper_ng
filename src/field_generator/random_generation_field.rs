use super::{
    MineSweeperCell,
    MineSweeperField,
    MineSweeperFieldCreation,
};

use rand::{
    Rng,
    rngs::StdRng,
    SeedableRng,
};

#[derive(Clone)]
pub struct RandomGenerationField {
    width: u32,
    height: u32,
    mines: u32,
    start_field: (u32, u32),
    board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField for RandomGenerationField {
    #[track_caller]
    fn new(width: u32, height: u32, mines: MineSweeperFieldCreation) -> Self {
        let percentage = mines.get_percentage(width, height);

        if percentage >= 0.9 {
            panic!("Too many mines, this won't be solvable!");
        }

        if percentage <= 0.0 {
            panic!("Negative or zero percentage of mines!");
        }

        if percentage > 0.25 {
            println!("Warning: {}% of the fields are mines!", percentage * 100.0);
        }

        let board = vec![vec![MineSweeperCell::Empty; height as usize]; width as usize];
        let mines = mines.get_fixed_count(width, height);

        let mut field = RandomGenerationField {
            width,
            height,
            mines,
            board,
            start_field: (0, 0),
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

    fn get_start_field(&self) -> (u32, u32) {
        self.start_field
    }

    fn get_field(&self) -> Vec<Vec<MineSweeperCell>> {
        self.board.clone()
    }

    fn get_cell(&self, x: u32, y: u32) -> MineSweeperCell {
        self.board[x as usize][y as usize].clone()
    }

    fn set_cell(&mut self, x: u32, y: u32, cell: MineSweeperCell) {
        self.board[x as usize][y as usize] = cell;
    }

}

impl RandomGenerationField {
    pub fn get_cells(&self) -> Vec<Vec<MineSweeperCell>> {
        self.board.clone()
    }

    pub fn initialize(&mut self) {
        self.place_mines();
        self.assign_numbers();
        self.set_start_field();
    }

    fn set_start_field(&mut self) {
        /*
        TODO:
        Set the start field to the first empty cell found
        Can later also be set to a random empty cell
        */
        for (x, y) in self.sorted_fields() {
            if self.get_cell(x, y) == MineSweeperCell::Empty {
                self.start_field = (x, y);
                return;
            }
        }
    }

    fn place_mines(&mut self) {
        let mut placed_mines = 0;
        /*
        TODO:
        Currently for testing purposes, but in the future a random seed will be used
        */
        let seed: u64 = 40;
        let mut rng = StdRng::seed_from_u64(seed);

        while placed_mines < self.mines {
            let x = (rng.random_range(0..u64::MAX) % self.width as u64 ) as u32;
            let y = (rng.random_range(0..u64::MAX) % self.height as u64 ) as u32;

            if self.get_cell(x, y) == MineSweeperCell::Empty {
                self.set_cell(x, y, MineSweeperCell::Mine);
                placed_mines += 1;
            }
        }
    }
}
