use minesweeper_ng_gen::*;

fn main() {
    //let field = get_evil_ng_field();
    let field: RandomField = RandomField::new(60, 40, Mines::Density(0.20)).unwrap();
    field.show();
    solve_field(&field);
}
