use minesweeper_ng_gen::*;

// minesweeper.online/de/help/patterns
fn main() {
    // B1
    let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(4));
    field.set_start_cell(1, 1);
    field.initialize(vec![(3, 1), (3, 2), (3, 3), (0, 4)]);
    field.show();

    // 1-1
    let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(2));
    field.set_start_cell(1, 1);
    field.initialize(vec![(0, 3), (3, 3)]);
    field.show();

    // 1-2
    let mut field = MineField::new(3, 4, MineSweeperFieldCreation::FixedCount(2));
    field.set_start_cell(0, 0);
    field.initialize(vec![(0, 2), (2, 2)]);
    field.show();

    // 1-2-2-1
    let mut field = MineField::new(4, 4, MineSweeperFieldCreation::FixedCount(2));
    field.set_start_cell(0, 0);
    field.initialize(vec![(1, 3), (2, 3)]);
    field.show();

    // 1-2-1 R
    let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(3));
    field.set_start_cell(0, 0);
    field.initialize(vec![(2, 2), (1, 4), (3, 4)]);
    field.show();

    // H3
    let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(3));
    field.set_start_cell(0, 0);
    field.initialize(vec![(1, 2), (4, 2), (1, 4)]);
    field.show();

    // 1-3-1
    let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(3));
    field.set_start_cell(0, 0);
    field.initialize(vec![(1, 2), (2, 2), (2, 1)]);
    field.show();

    // 1-3-1 (2)
    let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(3));
    field.set_start_cell(0, 0);
    field.initialize(vec![(0, 2), (2, 2), (2, 0)]);
    field.show();

    // 1-3-1 (3)
    let mut field = MineField::new(3, 3, MineSweeperFieldCreation::FixedCount(3));
    field.set_start_cell(0, 0);
    field.initialize(vec![(0, 2), (2, 2), (2, 1)]);
    field.show();

    // 2-2-2
    let mut field = MineField::new(5, 5, MineSweeperFieldCreation::FixedCount(8));
    field.set_start_cell(2, 2);
    field.initialize(vec![(1, 0), (0, 1), (0, 3), (3, 0), (1, 4), (4, 1), (4, 3), (3, 4)]);
    field.show();
}
