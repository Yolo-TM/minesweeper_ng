use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //let new_field = MineField::from_json(&json).unwrap();
    //solve(new_field, true);
    //let field = minesweeper_ng_field(12, 12, Mines::Density(0.25));
    //field.to_file("edge_case_field.minesweeper").unwrap();
    //let field = minesweeper_ng_field(45, 26, Mines::Density(0.22));
    //field.to_file("ng_field.minesweeper").unwrap();
    let field = RandomField::new(45, 26, Mines::Density(0.22));
    field.show();
    solve_field(&field);
    //solve(field, true);

    println!("Time elapsed: {:?}", start.elapsed());
}
