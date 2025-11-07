use super::{Cell, MineSweeperField, Mines};

#[derive(Clone)]
pub struct DefinedField {
    width: u32,
    height: u32,
    mines: u32,
    start_cell: (u32, u32),
    board: Vec<Vec<Cell>>,
}

impl MineSweeperField for DefinedField {
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

        let field = DefinedField {
            width,
            height,
            mines,
            board,
            start_cell: (0, 0),
        };

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

impl DefinedField {
    pub fn initialize(&mut self, mine_positions: Vec<(u32, u32)>) {
        for &(x, y) in &mine_positions {
            self.set_cell(x, y, Cell::Mine);
        }

        self.assign_numbers();
    }

    pub fn set_start_cell(&mut self, x: u32, y: u32) {
        self.start_cell = (x, y);
    }
}
