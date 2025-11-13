use minesweeper_ng_gen::{solve_field, DefinedField, MineSweeperField, RandomField, Mines};

fn main() {
    let start = std::time::Instant::now();

    //solve_field(&DefinedField::from_file("src/generated/testing/extended_box_logic.minesweeper").unwrap());

    let field = RandomField::new(45, 26, Mines::Density(0.22));
    //field.show();
    solve_field(&field);

    println!("Time elapsed: {:?}", start.elapsed());
}
