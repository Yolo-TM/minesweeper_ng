use crate::minesweeper_field::{
    MineSweeperField,
    MineSweeperCellState,
};

#[derive(Clone, PartialEq)]
pub struct MinesweeperGame {
    pub field: MineSweeperField,
    pub state: Vec<Vec<MineSweeperCellState>>,
    pub game_over: bool,
    pub time: u64,
}

