use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::MineSweeperFieldCreation;

/// Prompt the user for a u32 input with validation and reprompt on error.
pub fn prompt_u32(prompt: &str) -> io::Result<u32> {
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

/// Prompt the user for a pair of dimensions (width, height) where both must be > 0.
pub fn prompt_dimensions() -> (u32, u32) {
    let width = loop {
        print!("Enter field width: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(v) if v > 0 => break v,
            Ok(_) => println!("Please enter a value greater than 0."),
            Err(_) => println!("Please enter a valid number."),
        }
    };

    let height = loop {
        print!("Enter field height: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(v) if v > 0 => break v,
            Ok(_) => println!("Please enter a value greater than 0."),
            Err(_) => println!("Please enter a valid number."),
        }
    };

    (width, height)
}

/// Create (if missing) and return an output directory path string.
/// Prints whether it created the directory or will reuse it.
pub fn ensure_output_dir(dir: &str) -> io::Result<()> {
    if Path::new(dir).exists() {
        println!("Directory '{}' already exists. Files will be added/overwritten.", dir);
    } else {
        fs::create_dir_all(dir)?;
        println!("Created directory: {}", dir);
    }
    Ok(())
}

/// Simple yes/no prompt. Returns true if user says yes.
pub fn prompt_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' for yes or 'n' for no."),
        }
    }
}

/// Prompt for mine specification, either fixed count or percentage.
pub fn prompt_mine_specification(width: u32, height: u32) -> MineSweeperFieldCreation {
    let total_cells = width * height;
    let max_mines = total_cells.saturating_sub(1);

    loop {
        println!("\nChoose mine specification:");
        println!("1. Fixed count (specify exact number of mines)");
        println!("2. Percentage (specify percentage of field to be mines)");
        print!("Enter choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                loop {
                    print!("Enter number of mines (1 to {}): ", max_mines);
                    io::stdout().flush().unwrap();

                    let mut mine_input = String::new();
                    io::stdin().read_line(&mut mine_input).unwrap();

                    match mine_input.trim().parse::<u32>() {
                        Ok(count) if count >= 1 && count <= max_mines => {
                            return MineSweeperFieldCreation::FixedCount(count);
                        }
                        Ok(_) => println!("Please enter a value between 1 and {}.", max_mines),
                        Err(_) => println!("Please enter a valid number."),
                    }
                }
            }
            "2" => {
                loop {
                    print!("Enter mine percentage (0.0 to 1.0): ");
                    io::stdout().flush().unwrap();

                    let mut percentage_input = String::new();
                    io::stdin().read_line(&mut percentage_input).unwrap();

                    match percentage_input.trim().parse::<f32>() {
                        Ok(percentage) if percentage > 0.0 && percentage < 1.0 => {
                            return MineSweeperFieldCreation::Percentage(percentage);
                        }
                        Ok(_) => println!("Please enter a percentage between 0.0 and 1.0."),
                        Err(_) => println!(
                            "Please enter a valid decimal number (e.g., 0.15 for 15%)."
                        ),
                    }
                }
            }
            _ => println!("Please enter 1 or 2."),
        }
    }
}
