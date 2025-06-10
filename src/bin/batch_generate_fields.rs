use std::fs;
use std::io::{self, Write};
use std::path::Path;

use minesweeper_ng_gen::*;

fn main() -> io::Result<()> {
    println!("=== Minesweeper Field Batch Generator ===\n");

    // Get field dimensions from user
    let width = get_user_input_u32("Enter field width: ")?;
    let height = get_user_input_u32("Enter field height: ")?;
    let mines = get_user_input_u32("Enter number of mines: ")?;
    let field_count = get_user_input_u32("Enter number of fields to generate: ")?;

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
    if Path::new(&output_dir).exists() {
        println!("Directory '{}' already exists. Files will be added/overwritten.", output_dir);
    } else {
        fs::create_dir_all(&output_dir)?;
        println!("Created directory: {}", output_dir);
    }

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

fn get_user_input_u32(prompt: &str) -> io::Result<u32> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(value) => return Ok(value),
            Err(_) => {
                println!("Invalid input. Please enter a positive number.");
                continue;
            }
        }
    }
}