use minesweeper_ng_gen::{MineSweeperField, minesweeper_ng_field, cli_utils};

fn main() {
    println!("=== No-Guess Minesweeper Field Generator ===");
    println!("Generate custom no-guess minesweeper fields based on your specifications");
    println!("No-guess fields are solvable without requiring guessing\n");

    loop {
        let (width, height) = cli_utils::prompt_dimensions();
        let mines = cli_utils::prompt_mine_specification(width, height);

        println!("\nGenerating no-guess field...");
        println!("Note: No-guess field generation can take significantly longer than standard fields");
        let start = std::time::Instant::now();
        let field = minesweeper_ng_field(width, height, mines);
        let generation_time = start.elapsed();
        println!("No-guess field generated in {:?}", generation_time);
        field.show();

        if !cli_utils::prompt_yes_no("\nDo you want to generate another no-guess field? (y/n): ") { break; }
        println!();
    }
    println!("Goodbye!");
}
 
