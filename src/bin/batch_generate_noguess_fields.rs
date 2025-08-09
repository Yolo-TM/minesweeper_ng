use std::io::{self, Write};
use std::path::Path;

use minesweeper_ng_gen::{
    minesweeper_ng_field, 
    MineSweeperField, 
    MineSweeperFieldCreation,
    cli_utils,
};

fn main() -> io::Result<()> {
    println!("=== No-Guess Minesweeper Field Batch Generator ===\n");
    println!("Generate multiple no-guess minesweeper fields that can be solved without guessing");
    println!("Note: No-guess field generation can take significantly longer than standard fields\n");

    // Get field dimensions from user
    let width = cli_utils::prompt_u32("Enter field width: ")?;
    let height = cli_utils::prompt_u32("Enter field height: ")?;
    let mines = cli_utils::prompt_u32("Enter number of mines: ")?;
    let field_count = cli_utils::prompt_u32("Enter number of no-guess fields to generate: ")?;

    // Validate input
    if mines >= width * height {
        eprintln!("Error: Number of mines ({}) must be less than total cells ({})", mines, width * height);
        return Ok(());
    }

    if width == 0 || height == 0 || field_count == 0 {
        eprintln!("Error: Width, height, and field count must be greater than 0");
        return Ok(());
    }

    // Create output directory
    let output_dir = format!("generated_noguess_fields_{}x{}_{}mines", width, height, mines);
    cli_utils::ensure_output_dir(&output_dir)?;

    println!("\nGenerating {} no-guess field(s) with dimensions {}x{} and {} mines...", field_count, width, height, mines);
    println!("This may take some time as each field needs to be verified as solvable without guessing.\n");

    // Generate fields
    let mut successful_generations = 0;
    let start_time = std::time::Instant::now();

    for i in 1..=field_count {
        print!("Generating no-guess field {}/{}... ", i, field_count);
        io::stdout().flush()?;

        let field_start = std::time::Instant::now();
        match generate_and_save_noguess_field(width, height, mines, &output_dir, i) {
            Ok(filename) => {
                let generation_time = field_start.elapsed();
                println!("✓ Saved as {} (took {:?})", filename, generation_time);
                successful_generations += 1;
            }
            Err(e) => {
                let generation_time = field_start.elapsed();
                println!("✗ Failed: {} (took {:?})", e, generation_time);
            }
        }
    }

    let total_time = start_time.elapsed();

    println!("\n=== Generation Complete ===");
    println!("Successfully generated: {}/{} no-guess fields", successful_generations, field_count);
    println!("Total time: {:?}", total_time);
    if successful_generations > 0 {
        println!("Average time per field: {:?}", total_time / successful_generations);
    }
    println!("Output directory: {}", output_dir);

    Ok(())
}

fn generate_and_save_noguess_field(width: u32, height: u32, mines: u32, output_dir: &str, index: u32) -> io::Result<String> {
    // Create a new no-guess minesweeper field
    let field = minesweeper_ng_field(width, height, MineSweeperFieldCreation::FixedCount(mines));

    // Create filename
    let filename = format!("noguess_field_{:04}.minesweeper", index);
    let filepath = Path::new(output_dir).join(&filename);

    // Save to file using the to_file method
    field.to_file(filepath.to_str().unwrap())?;

    Ok(filename)
}
 
