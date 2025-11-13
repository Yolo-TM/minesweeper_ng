use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    let field: RandomField = RandomField::new(45, 26, Mines::Density(0.24));
    solve_field(&field);

    println!("Time elapsed: {:?}", start.elapsed());
}