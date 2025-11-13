use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    //solve_field(&DefinedField::from_file("src/generated/testing/extended_box_logic.minesweeper").unwrap());

    let field = RandomField::new(100, 100, Mines::Density(0.27));
    //field.show();
    let _is_solvable = is_solvable(&field);

    println!("Field solved {}", if _is_solvable { "successfully" } else { "unsuccessfully" });
    println!("Time elapsed: {:?}", start.elapsed());
}
