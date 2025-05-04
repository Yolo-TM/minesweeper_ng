use crate::field_generator::MineSweeperField;

mod test_ng_field;

pub fn get_evil_field() -> MineSweeperField {
    test_ng_field::get_ng_minesweeper_field()
}

pub fn get_small_test_field() -> MineSweeperField {
    test_ng_field::get_small_test_field()
}