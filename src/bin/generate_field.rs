use minesweeper_ng_gen::{minesweeper_field, MineSweeperField, cli_utils};

fn main() {
    println!("=== Minesweeper Field Generator ===");
    println!("Generate custom minesweeper fields based on your specifications\n");

    loop {
        let (width, height) = cli_utils::prompt_dimensions();
        let mines = cli_utils::prompt_mine_specification(width, height);

        println!("\nGenerating field...");
        let start = std::time::Instant::now();
        let field = minesweeper_field(width, height, mines);
        let generation_time = start.elapsed();
        println!("Field generated in {:?}", generation_time);
        field.show();

        if !cli_utils::prompt_yes_no("\nDo you want to generate another field? (y/n): ") { break; }
        println!();
    }

    println!("Goodbye!");
}