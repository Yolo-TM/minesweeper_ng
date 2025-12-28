use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //let field: RandomField = RandomField::new(10, 10, Mines::Density(0.24));
    //let field: RandomField = RandomField::new(45, 26, Mines::Density(0.24));
    let field: DefinedField = DefinedField::from_file("src/generated/testing/benchmarking/7.minesweeper").unwrap();
    //let field: DefinedField = DefinedField::from_file("src/generated/testing/hard.minesweeper").unwrap();
    //solve_field(&field);
    is_solvable(&field);
    //field.to_svg("output.svg");

    println!("Time elapsed: {:?}", start.elapsed());
}
