use std::process::exit;
use clap::{Arg, ArgAction, ArgGroup, Command};
use crate::Mines;

#[derive(Clone)]
pub struct CommandResult {
    pub width: u32,
    pub height: u32,
    pub mine_spec: Mines,
    pub count: u32,
    pub no_guess: bool,
    pub output: String,
}

pub fn execute_command() -> CommandResult {
    let app = build_command();

    match app.get_matches().subcommand() {
        Some(("generate", sub_matches)) => {
            let (width, height, mine_spec, no_guess, output) = parse_common_args(sub_matches);

            CommandResult {
                width,
                height,
                mine_spec,
                count: 1,
                no_guess,
                output: output + ".minesweeper",
            }
        },
        Some(("batch", sub_matches)) => {
            let (width, height, mine_spec, no_guess, output) = parse_common_args(sub_matches);
            let count = *sub_matches.get_one::<u32>("count").unwrap();

            CommandResult {
                width,
                height,
                mine_spec,
                count,
                no_guess,
                output,
            }
        },
        _ => {
            build_command().print_help().unwrap();
            println!();
            exit(2);
        }
    }
}

fn build_command() -> Command {
    fn build_base_subcommand(name: &'static str, about: &'static str) -> Command {
        Command::new(name)
            .about(about)
            .disable_help_flag(true)
            .help_template("{about}\n\nUSAGE:\n    {usage}\n\nOPTIONS:\n{options}")
            .arg(Arg::new("width")
                .short('w')
                .long("width")
                .help("Width of the field")
                .value_parser(clap::value_parser!(u32))
                .required(true))
            .arg(Arg::new("height")
                .short('h')
                .long("height")
                .help("Height of the field")
                .value_parser(clap::value_parser!(u32))
                .required(true))
            .arg(Arg::new("mines")
                .short('m')
                .long("mines")
                .help("Number of mines")
                .value_parser(clap::value_parser!(u32)))
            .arg(Arg::new("percentage")
                .short('p')
                .long("percentage")
                .help("Percentage of mines (0.0-1.0)")
                .value_parser(clap::value_parser!(f32)))
            .arg(Arg::new("no-guess")
                .long("no-guess")
                .help("Generate no-guess fields (solvable without guessing)")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .help("Output Folder")
                .value_parser(clap::value_parser!(String)))
            .arg(Arg::new("help")
                .long("help")
                .help("Print help")
                .action(ArgAction::Help))
            .group(ArgGroup::new("mine_spec")
                .args(&["mines", "percentage"])
                .required(true))
    }

    Command::new("minesweeper_gen")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Generate minesweeper fields")
        .subcommand(build_base_subcommand("generate", "Generate a single minesweeper field"))
        .subcommand(build_base_subcommand("batch", "Generate multiple minesweeper fields")
            .arg(Arg::new("count")
                .short('c')
                .long("count")
                .help("Number of fields to generate")
                .value_parser(clap::value_parser!(u32))
                .required(true))
            )
}

fn parse_common_args(sub_matches: &clap::ArgMatches) -> (u32, u32, Mines, bool, String) {
    let width = *sub_matches.get_one::<u32>("width").unwrap();
    let height = *sub_matches.get_one::<u32>("height").unwrap();
    let mines = sub_matches.get_one::<u32>("mines").copied();
    let percentage = sub_matches.get_one::<f32>("percentage").copied();
    let no_guess = sub_matches.get_flag("no-guess");

    let mine_spec: Mines = match (mines, percentage) {
        (Some(mines), None) => Mines::Count(mines),
        (None, Some(percentage)) => Mines::Density(percentage),
        _ => {
            eprintln!("Error: Either mines or percentage must be specified");
            exit(2);
        }
    };

    if !mine_spec.is_valid(width, height) {
        eprintln!("Error: Invalid mine specification for the given field size");
        exit(2);
    }

    let output_directory = sub_matches.get_one::<String>("output").cloned().unwrap_or_else(|| {
        let prefix = if no_guess { "ng_" } else { "" };
        format!("{}{}x{}_{}_mines", prefix, width, height, mine_spec.get_fixed_count(width, height))
    });

    (width, height, mine_spec, no_guess, output_directory)
}