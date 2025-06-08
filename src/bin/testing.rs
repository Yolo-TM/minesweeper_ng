use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    let field = get_evil_ng_field();
    let _ = field.to_file("evil_ng_field.minesweeper");
    let field = MineField::from_file("evil_ng_field.minesweeper").unwrap();
    solve(field, true);

    //let field = field_generator::get_small_test_field();
    //let json = field.as_json();
    //let new_field = TestField::from_json(&json).unwrap();
    //minesweeper_solver::solve(new_field, true);

    //let field = ng_generator::minesweeper_ng_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    //let field = minesweeper_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    //minesweeper_solver::solve(field, true);

    println!("Time elapsed: {:?}", start.elapsed());
}
