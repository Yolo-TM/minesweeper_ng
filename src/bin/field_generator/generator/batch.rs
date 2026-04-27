use super::command::CommandResult;
use indicatif::{ProgressBar, ProgressStyle};
use minesweeper_ng_gen::*;
use rayon::prelude::*;
use std::time::Instant;
use std::{fs, io, path::Path};

const PROGRESS_TEMPLATE: &str =
    "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | {wide_msg}";
const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏ .";

pub fn generate_fields(field_data: CommandResult) {
    setup_output_directory(&field_data.output).unwrap_or_else(|err| {
        eprintln!(
            "Error setting up output directory '{}': {}",
            field_data.output, err
        );
        return;
    });

    let progress = ProgressBar::new(field_data.count as u64);
    progress.set_style(
        ProgressStyle::with_template(PROGRESS_TEMPLATE)
            .unwrap()
            .progress_chars(PROGRESS_CHARS),
    );
    progress.set_message("Generating fields...");

    let start_time = Instant::now();
    (1..=field_data.count).into_par_iter().for_each(|id| {
        let filename = format!("{}/{}.minesweeper", field_data.output, id);
        let result = if field_data.no_guess {
            NoGuessField::new(field_data.width, field_data.height, field_data.mine_spec)
                .map(|field| field.to_file(&filename))
        } else {
            RandomField::new(field_data.width, field_data.height, field_data.mine_spec)
                .map(|field| field.to_file(&filename))
        };
        if let Err(err) = result {
            progress.println(format!("Error generating field {}: {}", id, err));
        }
        progress.inc(1);
    });

    progress.finish_with_message(format!(
        "Generation completed in {:.2?}",
        start_time.elapsed()
    ));
    println!("Saved Files to '{}'", field_data.output);
}

fn setup_output_directory(output: &String) -> io::Result<()> {
    if Path::new(&output).exists() {
        println!(
            "Directory '{}' already exists. Files will be added/overwritten.",
            output
        );
    } else {
        fs::create_dir_all(&output).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create directory '{}': {}", output, e),
            )
        })?;
        println!("Created directory: {}", output);
    }

    Ok(())
}
