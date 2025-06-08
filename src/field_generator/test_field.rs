use crate::field_generator::{MineSweeperCell, MineSweeperField, MineSweeperFieldCreation};

#[derive(Clone)]
pub struct TestField {
    width: u32,
    height: u32,
    mines: u32,
    start_field: (u32, u32),
    board: Vec<Vec<MineSweeperCell>>,
}

impl MineSweeperField for TestField {
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

        let field = TestField {
            width,
            height,
            mines,
            board,
            start_field: (0, 0),
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

impl TestField {
    pub fn initialize(&mut self, mine_positions: Vec<(u32, u32)>) {
        for &(x, y) in &mine_positions {
            self.set_cell(x, y, MineSweeperCell::Mine);
        }

        self.assign_numbers();
    }

    pub fn set_start_field(&mut self, x: u32, y: u32) {
        self.start_field = (x, y);
    }
}

pub fn get_evil_ng_field() -> impl MineSweeperField {

    let mut field = TestField::new(30, 20, MineSweeperFieldCreation::FixedCount(130));
    field.set_start_field(4, 6);

    // This are the mine positions of an evil field from minesweeper.online for testing purposes.
    field.initialize(vec![
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
        (10,0), (10,2), (10,4), (10,5), (10,15), (10,16), (10,19),
        (11,3), (11,6), (11,10), (11,15), (11,16), (11,19),
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
        (22,0), (22,10), (22,11), (22,12),
        (23,2), (23,3), (23,6), (23,13), (23,17), (23,18),
        (24,0), (24,5), (24,7), (24,15),
        (25,9), (25,11), (25,15), (25,16), (25,19),
        (26,2), (26,5), (26,13), (26,15),
        (27,1), (27,3), (27,5), (27,10), (27,11), (27,17), (27,18),
        (28,2), (28,16), (28,19),
        (29,2), (29,10), (29,11), (29,16)
    ]);

    field
}

pub fn get_small_test_field() -> impl MineSweeperField {

    let mut field = TestField::new(10, 10, MineSweeperFieldCreation::FixedCount(20));
    field.set_start_field(4, 7);

    field.initialize(vec![
        (6, 0), (2, 1), (4, 1), (4, 2), (5, 2),
        (0, 3), (1, 3), (4, 3), (5, 3), (7, 4),
        (0, 5), (1, 5), (5, 5), (7, 5), (0, 7),
        (1, 7), (6, 7), (2, 9), (5, 9), (6, 9),
    ]);

    field
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minesweeper_solver::{MineSweeperSolver, SolverSolution};

    #[test]
    fn field_setup_evil() {
        let field = get_evil_ng_field();
        assert_eq!(field.get_width(), 30);
        assert_eq!(field.get_height(), 20);
        assert_eq!(field.get_mines(), 130);
        assert_eq!(field.get_start_field(), (4, 6));
    }

    #[test]
    fn field_setup_small() {
        let field = get_small_test_field();
        assert_eq!(field.get_width(), 10);
        assert_eq!(field.get_height(), 10);
        assert_eq!(field.get_mines(), 20);
        assert_eq!(field.get_start_field(), (4, 7));
    }

    #[test]
    fn solve_evil() {
        let solved = MineSweeperSolver::new(get_evil_ng_field()).start(false);
        assert!(matches!(solved, SolverSolution::FoundSolution(_, _)));
    }

    #[test]
    fn solve_small() {
        let solved = MineSweeperSolver::new(get_small_test_field()).start(false);
        assert!(matches!(solved, SolverSolution::FoundSolution(_, _)));
    }
}