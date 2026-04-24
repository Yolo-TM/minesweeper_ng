use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //let field: RandomField = RandomField::new(10, 10, Mines::Density(0.24)).unwrap();
    let field: RandomField = RandomField::new(200, 100, Mines::Density(0.20)).unwrap();
    //let field: DefinedField = DefinedField::from_file("src/generated/testing/benchmarking/7.minesweeper").unwrap();
    //let field: DefinedField = DefinedField::from_file("src/generated/testing/extended_box_logic.minesweeper ").unwrap();

    let mut solver = create_solver(&field, 0); // 10 for output
    solver.solve();
    let solved = solver.is_solved();
    let solving_steps = solver.get_solving_steps();

    field.to_svg("output.svg", SVG_Mode::RevealSolver(solving_steps));

    println!("Time elapsed: {:?} - Solved: {:?}", start.elapsed(), solved);
}
