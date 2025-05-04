use crate::field_generator::{MineSweeperField, MineSweeperCell};

pub fn get_ng_minesweeper_field() -> MineSweeperField {
    let width = 30;
    let height = 20;
    let board = vec![vec![MineSweeperCell::Empty; height]; width];

    let mut field = MineSweeperField{
        width: width,
        height: height,
        mines: 130,
        board,
        start_field: (0, 0),
    };

    // This are the mine positions of an evil field from minesweeper.online for testing purposes.
    let mine_positions: Vec<(usize, usize)> = vec![
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
        (10,0), (10,2), (10,4), (10,5), (10,15), (10,16), (10,18),
        (11,3), (11,6), (11,10), (11,15), (11,16), (11,18),
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
        (22,1), (22,10), (22,11), (22,12),
        (23,2), (23,3), (23,6), (23,13), (23,17), (23,18),
        (24,0), (24,5), (24,7), (24,15),
        (25,9), (25,11), (25,15), (25,16), (25,19),
        (26,2), (26,5), (26,13), (26,15),
        (27,1), (27,3), (27,5), (27,10), (27,11), (27,17), (27,18),
        (28,2), (28,16), (28,19),
        (29,2), (29,10), (29,11), (29,16)
    ];

    for &(x, y) in &mine_positions {
        field.board[x][y] = MineSweeperCell::Mine;
    }

    field.initialize();
    field.start_field = (4, 6);
    field
}

pub fn get_small_test_field() -> MineSweeperField {
    let width = 10;
    let height = 10;
    let board = vec![vec![MineSweeperCell::Empty; height]; width];

    let mut field = MineSweeperField{
        width: width,
        height: height,
        mines: 20,
        board,
        start_field: (4, 7),
    };

    // This are the mine positions of an evil field from minesweeper.online for testing purposes.
    let mine_positions: Vec<(usize, usize)> = vec![
        (6, 0), (2, 1), (4, 1), (4, 2), (5, 2),
        (0, 3), (1, 3), (4, 3), (5, 3), (7, 4),
        (0, 5), (1, 5), (5, 5), (7, 5), (0, 7),
        (1, 7), (6, 7), (2, 9), (5, 9), (6, 9),
    ];

    for &(x, y) in &mine_positions {
        field.board[x][y] = MineSweeperCell::Mine;
    }

    field.initialize();
    field.start_field = (4, 7);
    field
}