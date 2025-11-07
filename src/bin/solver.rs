use minesweeper_ng_gen::*;

fn main() {
    let field = get_evil_ng_field();
    field.show();
    solve_field(&field);
}
