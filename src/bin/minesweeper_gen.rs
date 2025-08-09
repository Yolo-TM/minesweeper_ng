use std::fs;
use std::path::Path;
use std::io::{self, Write};
use clap::{Parser, Subcommand, ArgGroup};
use minesweeper_ng_gen::{minesweeper_field, minesweeper_ng_field, MineSweeperField, MineSweeperFieldCreation};

#[derive(Parser)]
#[command(name = "MineSweeper Field Generator")]
#[command(about = "A unified minesweeper field generator")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(group(ArgGroup::new("mine_spec").required(true).args(["mines", "percentage"]),))]
    Generate {
        #[arg(short, long)]
        width: u32,
        #[arg(short = 'H', long)]
        height: u32,
        #[arg(short, long, group = "mine_spec")]
        mines: Option<u32>,
        #[arg(short, long, group = "mine_spec")]
        percentage: Option<f32>,
        #[arg(short = 'n', long)]
        no_guess: bool,
    },

    #[command(group(ArgGroup::new("mine_spec").required(true).args(["mines", "percentage"]),))]
    Batch {
        #[arg(short, long)]
        width: u32,
        #[arg(short = 'H', long)]
        height: u32,
        #[arg(short, long, group = "mine_spec")]
        mines: Option<u32>,
        #[arg(short, long, group = "mine_spec")]
        percentage: Option<f32>,
        #[arg(short, long)]
        count: u32,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short = 'n', long)]
        no_guess: bool,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            width,
            height,
            mines,
            percentage,
            no_guess,
        } => {
            let mine_spec = match (mines, percentage) {
                (Some(m), None) => MineSweeperFieldCreation::FixedCount(m),
                (None, Some(p)) => MineSweeperFieldCreation::Percentage(p),
                _ => unreachable!(),
            };

            generate_single_field(width, height, mine_spec, no_guess);
        }

        Commands::Batch {
            width,
            height,
            mines,
            percentage,
            count,
            output,
            no_guess,
        } => {
            let mine_spec = match (mines, percentage) {
                (Some(m), None) => MineSweeperFieldCreation::FixedCount(m),
                (None, Some(p)) => MineSweeperFieldCreation::Percentage(p),
                _ => unreachable!(),
            };

            batch_generate_fields(width, height, mine_spec, count, output, no_guess)?;
        }
    }
    Ok(())
}

fn generate_single_field(width: u32, height: u32, mine_spec: MineSweeperFieldCreation, is_noguess: bool) {
    println!("Generating {}field with dimensions {}x{} and {}...",
        if is_noguess { "no-guess " } else { "" },
        width, height,
        format!("{} mines ({}%)", mine_spec.get_fixed_count(width, height), mine_spec.get_percentage(width, height) * 100.0));

    if is_noguess {
        println!("Note: No-guess field generation can take significantly longer than standard fields");
    }
    
    let start = std::time::Instant::now();
    let generation_time = if is_noguess {
        let field = minesweeper_ng_field(width, height, mine_spec);
        let elapsed = start.elapsed();
        field.show();
        elapsed
    } else {
        let field = minesweeper_field(width, height, mine_spec);
        let elapsed = start.elapsed();
        field.show();
        elapsed
    };
    
    println!("{}Field generated in {:?}", if is_noguess { "NoGuess " } else { "" }, generation_time);
}

fn batch_generate_fields(width: u32, height: u32, mine_spec: MineSweeperFieldCreation, field_count: u32, output: Option<String>, is_noguess: bool) -> io::Result<()> {
    if field_count == 0 {
        eprintln!("Error: Field count must be greater than 0");
        return Ok(());
    }

    let output_dir = output.unwrap_or_else(|| {
        if is_noguess {
            format!("generated_noguess_fields_{}x{}_{}", width, height, mine_spec.get_fixed_count(width, height))
        } else {
            format!("generated_fields_{}x{}_{}", width, height, mine_spec.get_fixed_count(width, height))
        }
    });

    if Path::new(&output_dir).exists() {
        println!("Directory '{}' already exists. Files will be added/overwritten.", output_dir);
    } else {
        fs::create_dir_all(&output_dir)?;
        println!("Created directory: {}", output_dir);
    }

    println!("Generating {}field with dimensions {}x{} and {}...",
        if is_noguess { "no-guess " } else { "" },
        width, height,
        format!("{} mines ({}%)", mine_spec.get_fixed_count(width, height), mine_spec.get_percentage(width, height) * 100.0));

    if is_noguess {
        println!("This may take some time as each field needs to be verified as solvable without guessing.\n");
    }

    let mut successful_generations = 0;
    let start_time = std::time::Instant::now();

    for i in 1..=field_count {
        print!("Generating {}field {}/{}... ", if is_noguess { "no-guess " } else { "" }, i, field_count);
        io::stdout().flush()?;

        let field_start = std::time::Instant::now();
        match generate_and_save_field(width, height, &mine_spec, &output_dir, i, is_noguess) {
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
    println!("Successfully generated: {}/{} {}fields", successful_generations, field_count, if is_noguess { "no-guess " } else { "" });
    println!("Total time: {:?}", total_time);

    if successful_generations > 0 {
        println!("Average time per field: {:?}", total_time / successful_generations);
    }

    println!("Output directory: {}", output_dir);

    Ok(())
}

fn generate_and_save_field(
    width: u32,
    height: u32,
    mine_spec: &MineSweeperFieldCreation,
    output_dir: &str,
    index: u32,
    is_noguess: bool,
) -> io::Result<String> {
    // Create filename
    let filename = if is_noguess {
        format!("noguess_field_{:04}.minesweeper", index)
    } else {
        format!("field_{:04}.minesweeper", index)
    };
    let filepath = Path::new(output_dir).join(&filename);

    if is_noguess {
        let field = minesweeper_ng_field(width, height, mine_spec.clone());
        field.to_file(filepath.to_str().unwrap())?;
    } else {
        let field = minesweeper_field(width, height, mine_spec.clone());
        field.to_file(filepath.to_str().unwrap())?;
    }

    Ok(filename)
}