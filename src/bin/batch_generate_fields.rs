use std::io::{self, Write};
use std::path::Path;

use minesweeper_ng_gen::*;

fn main() -> io::Result<()> {
    println!("=== Minesweeper Field Batch Generator ===\n");

    // Get field dimensions from user
    let width = cli_utils::prompt_u32("Enter field width: ")?;
    let height = cli_utils::prompt_u32("Enter field height: ")?;
    let mines = cli_utils::prompt_u32("Enter number of mines: ")?;
    let field_count = cli_utils::prompt_u32("Enter number of fields to generate: ")?;

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
    let output_dir = format!("generated_fields_{}x{}_{}mines", width, height, mines);
    cli_utils::ensure_output_dir(&output_dir)?;

    println!("\nGenerating {} field(s) with dimensions {}x{} and {} mines...\n", field_count, width, height, mines);

    // Generate fields
    let mut successful_generations = 0;
    for i in 1..=field_count {
        print!("Generating field {}/{}... ", i, field_count);
        io::stdout().flush()?;

        match generate_and_save_field(width, height, mines, &output_dir, i) {
            Ok(filename) => {
                println!("✓ Saved as {}", filename);
                successful_generations += 1;
            }
            Err(e) => {
                println!("✗ Failed: {}", e);
            }
        }
    }

    println!("\n=== Generation Complete ===");
    println!("Successfully generated: {}/{} fields", successful_generations, field_count);
    println!("Output directory: {}", output_dir);

    Ok(())
}

fn generate_and_save_field(width: u32, height: u32, mines: u32, output_dir: &str, index: u32) -> io::Result<String> {
    let field = minesweeper_field(width, height, MineSweeperFieldCreation::FixedCount(mines));

    // Create filename
    let filename = format!("field_{:04}.minesweeper", index);
    let filepath = Path::new(output_dir).join(&filename);

    field.to_file(filepath.to_str().unwrap())?;

    Ok(filename)
}
