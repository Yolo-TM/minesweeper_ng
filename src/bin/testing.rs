use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //let field = get_evil_ng_field();
    //let _ = field.to_file("evil_ng_field.minesweeper");
    let field = MineField::from_file("evil_ng_field.minesweeper").unwrap();
    solve(field, true);

    //let field = get_small_test_field();
    //let json = field.as_json();
    //let new_field = MineField::from_json(&json).unwrap();
    //solve(new_field, true);
    //let field = minesweeper_ng_field(12, 12, MineSweeperFieldCreation::Percentage(0.25));
    //field.to_file("edge_case_field.minesweeper").unwrap();
    //let field = minesweeper_ng_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    //field.to_file("ng_field.minesweeper").unwrap();
    //let field = minesweeper_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    //solve(field, true);

    println!("Time elapsed: {:?}", start.elapsed());
}
