use crate::field_generator::minesweeper_field::MineSweeperField;

mod test_ng_field;
mod boxes;
mod surrounding_fields_iterator;

pub mod solver;

pub fn minesweeper_solver(field: MineSweeperField) {
    solver::start(field);
}

pub fn get_evil_field() -> MineSweeperField {
    test_ng_field::get_ng_minesweeper_field()
}