use minesweeper_ng_gen::{DefinedField, Mines, MineSweeperFieldMetrics};

fn make_field(width: u32, height: u32, mines: Vec<(u32, u32)>) -> DefinedField {
    let mut field = DefinedField::new(width, height, Mines::Count(mines.len() as u32)).unwrap();
    field.initialize(mines);
    field
}

// 3x3, mine in center — all 8 surrounding cells are Number(1), no empty cells.
// Every number is isolated → 3BV = 8.
//
//  1 1 1
//  1 X 1
//  1 1 1
#[test]
fn single_mine_center_all_isolated_numbers() {
    let field = make_field(3, 3, vec![(1, 1)]);
    assert_eq!(field.get_3bv(), 8);
}

// 3x3, mine in corner — surrounding cells are numbers, rest empty.
// The empty cells form one opening that claims the bordering numbers → 3BV = 1.
//
//  X 1 .
//  1 1 .
//  . . .
#[test]
fn single_mine_corner_one_opening() {
    let field = make_field(3, 3, vec![(0, 0)]);
    assert_eq!(field.get_3bv(), 1);
}

// 5x1 row, mines at both ends — two Number(1) cells in the middle, no empty cells.
// Each number is isolated → 3BV = 2.
//
//  X 1 . . X  — wait, 5 wide 1 tall: mine@(0,0) and mine@(4,0)
//  cells: Mine Number(1) Empty Empty Number(1) Mine  — 6 wide
//  Use 6x1:
//
//  X 1 . . 1 X
#[test]
fn two_mines_no_opening_isolated_numbers() {
    let field = make_field(4, 1, vec![(0, 0), (3, 0)]);
    // cells: Mine Number(1) Number(1) Mine → 2 isolated numbers, no opening
    assert_eq!(field.get_3bv(), 2);
}

// 5x3, column of mines in the middle splits the board into two openings.
// Left side and right side each have their own connected empty region → 3BV = 2.
//
//  . . X . .
//  . . X . .
//  . . X . .
#[test]
fn two_separate_openings() {
    let field = make_field(5, 3, vec![(2, 0), (2, 1), (2, 2)]);
    assert_eq!(field.get_3bv(), 2);
}

// 3x3, mines on all four corners — center cell is Number(4), edge cells are Number(2)
// or Number(1). No empty cells at all, every non-mine cell is an isolated number → 3BV = 5.
//
//  X 2 X
//  2 4 2
//  X 2 X
#[test]
fn four_corner_mines_all_isolated_numbers() {
    let field = make_field(3, 3, vec![(0, 0), (2, 0), (0, 2), (2, 2)]);
    assert_eq!(field.get_3bv(), 5);
}

// 6x1 row with a mine in the middle — left and right sides each become an opening → 3BV = 2.
//
//  . . X . .  (5 wide)  →  . X .  (3 wide)
//  Use 5x1: mine at (2,0)
//  cells: Empty Number(1) Mine Number(1) Empty  →  2 separate openings
#[test]
fn mine_splits_row_into_two_openings() {
    let field = make_field(5, 1, vec![(2, 0)]);
    assert_eq!(field.get_3bv(), 2);
}
