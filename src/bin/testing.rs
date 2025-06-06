use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    let field = ng_generator::get_evil_ng_field();
    minesweeper_solver::solve(field, true);

    let field = ng_generator::get_small_test_field();
    minesweeper_solver::solve(field, true);

    //let field = ng_generator::minesweeper_ng_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    let field = minesweeper_field(45, 26, MineSweeperFieldCreation::Percentage(0.22));
    minesweeper_solver::solve(field, true);

    println!("Time elapsed: {:?}", start.elapsed());
}
