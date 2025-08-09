use std::fs;
use std::path::Path;
use std::io;
use clap::{Arg, Command, ArgGroup, ArgAction};
use indicatif::{ProgressBar, ProgressStyle};
use minesweeper_ng_gen::{minesweeper_field, minesweeper_ng_field, MineSweeperField, MineSweeperFieldCreation};

#[derive(Debug)]
enum Commands {
    Generate {
        width: u32,
        height: u32,
        mines: Option<u32>,
        percentage: Option<f32>,
        no_guess: bool,
    },
    Batch {
        width: u32,
        height: u32,
        mines: Option<u32>,
        percentage: Option<f32>,
        count: u32,
        output: Option<String>,
        no_guess: bool,
    },
}

fn build_cli() -> Command {
    Command::new("MineSweeper Field Generator")
        .about("A unified minesweeper field generator")
        .version(env!("CARGO_PKG_VERSION"))
        .disable_help_flag(true) // we want -h for height not default help
        .arg(
            Arg::new("help")
                .long("help")
                .help("Print help information")
                .action(ArgAction::Help),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate a single minesweeper field")
                .disable_help_flag(true)
                .arg(
                    Arg::new("help")
                        .long("help")
                        .help("Print help information")
                        .action(ArgAction::Help),
                )
                .group(ArgGroup::new("mine_spec").required(true).args(["mines", "percentage"]))
                .arg(
                    Arg::new("width")
                        .short('w')
                        .long("width")
                        .value_name("WIDTH")
                        .help("Width of the field")
                        .value_parser(clap::value_parser!(u32))
                        .required(true)
                )
                .arg(
                    Arg::new("height")
                        .short('h')
                        .long("height")
                        .value_name("HEIGHT")
                        .help("Height of the field")
                        .value_parser(clap::value_parser!(u32))
                        .required(true)
                )
                .arg(
                    Arg::new("mines")
                        .short('m')
                        .long("mines")
                        .value_name("MINES")
                        .help("Number of mines")
                        .value_parser(clap::value_parser!(u32))
                        .group("mine_spec")
                )
                .arg(
                    Arg::new("percentage")
                        .short('p')
                        .long("percentage")
                        .value_name("PERCENTAGE")
                        .help("Percentage of mines")
                        .value_parser(clap::value_parser!(f32))
                        .group("mine_spec")
                )
                .arg(
                    Arg::new("no_guess")
                        .short('n')
                        .long("no-guess")
                        .help("Generate no-guess fields")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("batch")
                .about("Generate multiple minesweeper fields")
                .disable_help_flag(true)
                .arg(
                    Arg::new("help")
                        .long("help")
                        .help("Print help information")
                        .action(ArgAction::Help),
                )
                .group(ArgGroup::new("mine_spec").required(true).args(["mines", "percentage"]))
                .arg(
                    Arg::new("width")
                        .short('w')
                        .long("width")
                        .value_name("WIDTH")
                        .help("Width of the field")
                        .value_parser(clap::value_parser!(u32))
                        .required(true)
                )
                .arg(
                    Arg::new("height")
                        .short('h')
                        .long("height")
                        .value_name("HEIGHT")
                        .help("Height of the field")
                        .value_parser(clap::value_parser!(u32))
                        .required(true)
                )
                .arg(
                    Arg::new("mines")
                        .short('m')
                        .long("mines")
                        .value_name("MINES")
                        .help("Number of mines")
                        .value_parser(clap::value_parser!(u32))
                        .group("mine_spec")
                )
                .arg(
                    Arg::new("percentage")
                        .short('p')
                        .long("percentage")
                        .value_name("PERCENTAGE")
                        .help("Percentage of mines")
                        .value_parser(clap::value_parser!(f32))
                        .group("mine_spec")
                )
                .arg(
                    Arg::new("count")
                        .short('c')
                        .long("count")
                        .value_name("COUNT")
                        .help("Number of fields to generate")
                        .value_parser(clap::value_parser!(u32))
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory")
                )
                .arg(
                    Arg::new("no_guess")
                        .short('n')
                        .long("no-guess")
                        .help("Generate no-guess fields")
                        .action(ArgAction::SetTrue)
                )
        )
}

fn parse_args() -> Commands {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let width = *sub_matches.get_one::<u32>("width").unwrap();
            let height = *sub_matches.get_one::<u32>("height").unwrap();
            let mines = sub_matches.get_one::<u32>("mines").copied();
            let percentage = sub_matches.get_one::<f32>("percentage").copied();
            let no_guess = sub_matches.get_flag("no_guess");

            Commands::Generate {
                width,
                height,
                mines,
                percentage,
                no_guess,
            }
        }
        Some(("batch", sub_matches)) => {
            let width = *sub_matches.get_one::<u32>("width").unwrap();
            let height = *sub_matches.get_one::<u32>("height").unwrap();
            let mines = sub_matches.get_one::<u32>("mines").copied();
            let percentage = sub_matches.get_one::<f32>("percentage").copied();
            let count = *sub_matches.get_one::<u32>("count").unwrap();
            let output = sub_matches.get_one::<String>("output").cloned();
            let no_guess = sub_matches.get_flag("no_guess");

            Commands::Batch {
                width,
                height,
                mines,
                percentage,
                count,
                output,
                no_guess,
            }
        }
        _ => {
            build_cli().print_help().unwrap();
            std::process::exit(1);
        }
    }
}

fn main() -> io::Result<()> {
    let command = parse_args();

    match command {
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
    }

    println!("Generating {}field with dimensions {}x{} and {}...",
        if is_noguess { "no-guess " } else { "" },
        width, height,
        format!("{} mines ({}%)", mine_spec.get_fixed_count(width, height), mine_spec.get_percentage(width, height) * 100.0));

    if is_noguess {
        println!("This may take some time as each field needs to be verified as solvable without guessing.\n");
    }

    let mut successful_generations = 0;
    let mut total_generation_time = std::time::Duration::ZERO;
    let progress_bar = ProgressBar::new(field_count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} | {wide_msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    progress_bar.set_message(format!("Starting {}Field Generation...", if is_noguess { "NoGuess-" } else { "" }));

    for i in 1..=field_count {
        let field_start = std::time::Instant::now();
        match generate_and_save_field(width, height, &mine_spec, &output_dir, i, field_count, is_noguess) {
            Ok(()) => {
                let generation_time = field_start.elapsed();
                total_generation_time += generation_time;
                successful_generations += 1;

                let avg_time = if successful_generations > 0 {
                    total_generation_time / successful_generations
                } else {
                    std::time::Duration::ZERO
                };

                progress_bar.set_message(format!("Avg: {:?} | Current: {:?}", avg_time, generation_time));
            }
            Err(e) => {
                progress_bar.println(format!("✗ Failed: {} (took {:?})", e, field_start.elapsed()));
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish();

    println!("\n=== Generation Complete ===");
    println!("Successfully generated: {}/{} {}fields", successful_generations, field_count, if is_noguess { "no-guess " } else { "" });
    println!("Output directory: {}", output_dir);

    Ok(())
}

fn generate_and_save_field(width: u32, height: u32, mine_spec: &MineSweeperFieldCreation, output_dir: &str, index: u32, max: u32, is_noguess: bool) -> io::Result<()> {

    let padding = max.to_string().len();
    let filename = if is_noguess {
        format!("noguess_field_{:0width$}.minesweeper", index, width = padding)
    } else {
        format!("field_{:0width$}.minesweeper", index, width = padding)
    };
    let filepath = Path::new(output_dir).join(&filename);

    if is_noguess {
        let field = minesweeper_ng_field(width, height, mine_spec.clone());
        field.to_file(filepath.to_str().unwrap())?;
    } else {
        let field = minesweeper_field(width, height, mine_spec.clone());
        field.to_file(filepath.to_str().unwrap())?;
    }

    Ok(())
}