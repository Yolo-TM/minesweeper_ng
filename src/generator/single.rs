use super::CommandResult;
use crate::MineSweeperField;
use crate::RandomField;
use crate::minesweeper_ng_field;
use std::{path::Path, process::exit};

pub fn generate_field(field_data: CommandResult) {
    // Check if file already exists and warn user
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
        let field = minesweeper_ng_field(field_data.width, field_data.height, field_data.mine_spec);
        process_field!(field, &field_data.output);
    } else {
        let field = RandomField::new(field_data.width, field_data.height, field_data.mine_spec);
        process_field!(field, &field_data.output);
    }
}
