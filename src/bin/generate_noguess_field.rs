use std::io::{self, Write};
use minesweeper_ng_gen::{MineSweeperFieldCreation, MineSweeperField, minesweeper_ng_field};

fn main() {
    println!("=== No-Guess Minesweeper Field Generator ===");
    println!("Generate custom no-guess minesweeper fields based on your specifications");
    println!("No-guess fields are solvable without requiring guessing\n");

    loop {
        // Get field dimensions
        let (width, height) = get_field_dimensions();

        // Get mine specification
        let mines = get_mine_specification(width, height);

        // Generate the no-guess field
        println!("\nGenerating no-guess field...");
        println!("Note: No-guess field generation can take significantly longer than standard fields");
        let start = std::time::Instant::now();
        let field = minesweeper_ng_field(width, height, mines.clone());
        let generation_time = start.elapsed();
        println!("No-guess field generated in {:?}", generation_time);
        field.show();

        // Ask if user wants to generate another field
        if !ask_continue() {
            break;
        }
        println!();
    }
    println!("Goodbye!");
}

fn get_field_dimensions() -> (u32, u32) {
    let width = loop {
        print!("Enter field width: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(value) if value > 0 => break value,
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
            Ok(value) if value > 0 => break value,
            Ok(_) => println!("Please enter a value greater than 0."),
            Err(_) => println!("Please enter a valid number."),
        }
    };

    (width, height)
}

fn get_mine_specification(width: u32, height: u32) -> MineSweeperFieldCreation {
    let total_cells = width * height;
    let max_mines = total_cells - 1;

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
                        Err(_) => println!("Please enter a valid decimal number (e.g., 0.15 for 15%)."),
                    }
                }
            }
            _ => println!("Please enter 1 or 2."),
        }
    }
}

fn ask_continue() -> bool {
    loop {
        print!("\nDo you want to generate another no-guess field? (y/n): ");
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
