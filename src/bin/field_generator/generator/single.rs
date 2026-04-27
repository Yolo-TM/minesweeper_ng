use super::command::CommandResult;
use minesweeper_ng_gen::*;
use std::{path::Path, process::exit};

pub fn generate_field(field_data: CommandResult) {
    if Path::new(&field_data.output).exists() {
        println!(
            "Warning: File '{}' already exists and will be overwritten.",
            field_data.output
        );
    }

    macro_rules! process_field {
        ($field:expr, $filename:expr) => {{
            $field.to_file($filename).unwrap_or_else(|err| {
                eprintln!("Error writing to file {}: {}", $filename, err);
                exit(1);
            });
            println!("Field generated and saved to: {}", $filename);
            $field.show();
        }};
    }

    if field_data.no_guess {
        let field = NoGuessField::new(field_data.width, field_data.height, field_data.mine_spec)
            .unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                exit(1);
            });
        process_field!(field, &field_data.output);
    } else {
        let field = RandomField::new(field_data.width, field_data.height, field_data.mine_spec)
            .unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                exit(1);
            });
        process_field!(field, &field_data.output);
    }
}
