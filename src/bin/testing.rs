use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //let field: RandomField = RandomField::new(10, 10, Mines::Density(0.24)).unwrap();
    let field: RandomField = RandomField::new(60, 30, Mines::Density(0.30)).unwrap();
    //let field: DefinedField = DefinedField::from_file("src/generated/testing/benchmarking/7.minesweeper").unwrap();
    //let field: DefinedField = DefinedField::from_file("src/generated/testing/extended_box_logic.minesweeper ").unwrap();
    //solve_field(&field);
    let solved = is_solvable(&field);
    field.to_svg("output.svg");

    println!("Time elapsed: {:?} - Solved: {:?}", start.elapsed(), solved);
}
