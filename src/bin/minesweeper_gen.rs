use std::{fs, io, thread, path::Path, sync::mpsc};
use std::time::{Duration, Instant};
use clap::{Arg, ArgAction, ArgGroup, Command};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use minesweeper_ng_gen::{minesweeper_field, minesweeper_ng_field, MineSweeperField, MineSweeperFieldCreation};

const DEFAULT_PROGRESS_TEMPLATE: &str = "Overall: [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | {wide_msg}";
const WORKER_PROGRESS_TEMPLATE: &str = "Worker {}: {{spinner:.green}} [{{bar:40.cyan/blue}}] {{pos}}/{{len}} ({{percent}}%) | {{wide_msg}}";
const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏  ";

#[derive(Debug)]
enum Commands {
    // mines or percentage must be specified for both commands
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
    }
}

fn main() -> io::Result<()> {
    let mut app = Command::new("minesweeper_gen")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Generate minesweeper fields")
        .subcommand({
            Command::new("generate")
                .about("Generate a single minesweeper field")
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
                .arg(Arg::new("help")
                    .long("help")
                    .help("Print help")
                    .action(ArgAction::Help))
                .group(ArgGroup::new("mine_spec")
                    .args(&["mines", "percentage"])
                    .required(true))
        })
        .subcommand(
            Command::new("batch")
                .about("Generate multiple minesweeper fields")
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
                .arg(Arg::new("count")
                    .short('c')
                    .long("count")
                    .help("Number of fields to generate")
                    .value_parser(clap::value_parser!(u32))
                    .required(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Output directory")
                    .value_parser(clap::value_parser!(String)))
                .arg(Arg::new("no-guess")
                    .long("no-guess")
                    .help("Generate no-guess fields (solvable without guessing)")
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("help")
                    .long("help")
                    .help("Print help")
                    .action(ArgAction::Help))
                .group(ArgGroup::new("mine_spec")
                    .args(&["mines", "percentage"])
                    .required(true))
        );

    let command = match app.clone().get_matches().subcommand() {
        Some(("generate", sub_matches)) => {
            let width = *sub_matches.get_one::<u32>("width").unwrap();
            let height = *sub_matches.get_one::<u32>("height").unwrap();
            let mines = sub_matches.get_one::<u32>("mines").copied();
            let percentage = sub_matches.get_one::<f32>("percentage").copied();
            let no_guess = sub_matches.get_flag("no-guess");

            Commands::Generate { width, height, mines, percentage, no_guess }
        }
        Some(("batch", sub_matches)) => {
            let width = *sub_matches.get_one::<u32>("width").unwrap();
            let height = *sub_matches.get_one::<u32>("height").unwrap();
            let mines = sub_matches.get_one::<u32>("mines").copied();
            let percentage = sub_matches.get_one::<f32>("percentage").copied();
            let count = *sub_matches.get_one::<u32>("count").unwrap();
            let output = sub_matches.get_one::<String>("output").cloned();
            let no_guess = sub_matches.get_flag("no-guess");

            Commands::Batch { width, height, mines, percentage, count, output, no_guess }
        }
        _ => {
            // Print clap-generated help and exit
            app.print_help().unwrap();
            println!();
            return Ok(());
        }
    };

    let mine_spec = match command {
        Commands::Generate { mines: Some(mines), .. } | Commands::Batch { mines: Some(mines), .. } => {
            MineSweeperFieldCreation::FixedCount(mines)
        }
        Commands::Generate { percentage: Some(percentage), .. } | Commands::Batch { percentage: Some(percentage), .. } => {
            if percentage < 0.0 || percentage > 1.0 {
                eprintln!("Error: Percentage must be between 0.0 and 1.0");
                return Ok(());
            }
            MineSweeperFieldCreation::Percentage(percentage)
        }
        _ => {
            eprintln!("Error: Either mines or percentage must be specified");
            return Ok(());
        }
    };

    match command {
        Commands::Generate { width, height, no_guess, .. } => {
            generate_single_field(width, height, mine_spec, no_guess)
        }
        Commands::Batch { width, height, count, output, no_guess, .. } => {
            batch_generate_fields(width, height, mine_spec, count, output, no_guess)
        }
    }
}

fn generate_single_field(width: u32, height: u32, mine_spec: MineSweeperFieldCreation, is_noguess: bool) -> io::Result<()> {
    println!("Generating {}field with dimensions {}x{} and {}...",
        if is_noguess { "no-guess " } else { "" },
        width, height,
        format!("{} mines ({}%)", mine_spec.get_fixed_count(width, height), mine_spec.get_percentage(width, height) * 100.0));

    let filename = if is_noguess {
        "noguess_field.minesweeper"
    } else {
        "field.minesweeper"
    };

    // Check if file already exists and warn user
    if Path::new(filename).exists() {
        println!("Warning: File '{}' already exists and will be overwritten.", filename);
    }

    if is_noguess {
        let field = minesweeper_ng_field(width, height, mine_spec);
        field.to_file(filename).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to save field to '{}': {}", filename, e)
            )
        })?;
        println!("Field generated and saved to: {}", filename);
        field.show();
    } else {
        let field = minesweeper_field(width, height, mine_spec);
        field.to_file(filename).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to save field to '{}': {}", filename, e)
            )
        })?;
        println!("Field generated and saved to: {}", filename);
        field.show();
    }

    Ok(())
}

fn setup_output_directory(output: Option<String>, width: u32, height: u32, mine_spec: &MineSweeperFieldCreation, is_noguess: bool) -> io::Result<String> {
    let output_dir = output.unwrap_or_else(|| {
        if is_noguess {
            format!("noguess_fields_{}x{}_{}_mines", width, height, mine_spec.get_fixed_count(width, height))
        } else {
            format!("fields_{}x{}_{}_mines", width, height, mine_spec.get_fixed_count(width, height))
        }
    });

    if Path::new(&output_dir).exists() {
        println!("Directory '{}' already exists. Files will be added/overwritten.", output_dir);
    } else {
        fs::create_dir_all(&output_dir).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create directory '{}': {}", output_dir, e)
            )
        })?;
        println!("Created directory: {}", output_dir);
    }

    Ok(output_dir)
}

fn batch_generate_fields(width: u32, height: u32, mine_spec: MineSweeperFieldCreation, field_count: u32, output: Option<String>, is_noguess: bool) -> io::Result<()> {
    if field_count == 0 {
        eprintln!("Error: Field count must be greater than 0");
        return Ok(());
    }

    let output_dir = setup_output_directory(output, width, height, &mine_spec, is_noguess)?;

    println!("Generating {}field with dimensions {}x{} and {}...",
        if is_noguess { "no-guess " } else { "" },
        width, height,
        format!("{} mines ({}%)", mine_spec.get_fixed_count(width, height), mine_spec.get_percentage(width, height) * 100.0));

    let worker_count = std::cmp::min(num_cpus::get(), field_count as usize) as u32;

    if is_noguess {
        println!("This may take some time as each field needs to be verified as solvable without guessing.\n");
    }

    println!("Using {} worker threads...\n", worker_count);
    batch_generate_with_workers(width, height, mine_spec, field_count, &output_dir, is_noguess, worker_count)
}

fn batch_generate_with_workers(
    width: u32,
    height: u32,
    mine_spec: MineSweeperFieldCreation,
    field_count: u32,
    output_dir: &str,
    is_noguess: bool,
    worker_count: u32,
) -> io::Result<()> {
    let controller = WorkerController::new(width, height, mine_spec, field_count, output_dir, is_noguess, worker_count);

    let main_progress_bar = controller.multi_progress.add(ProgressBar::new(field_count as u64));
    main_progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(DEFAULT_PROGRESS_TEMPLATE)
            .unwrap()
            .progress_chars(PROGRESS_CHARS)
    );
    main_progress_bar.set_message("Starting field generation...");

    let mut successful_generations = 0;
    let mut total_generation_time = Duration::ZERO;
    let mut results_received = 0;

    while results_received < field_count {
        match controller.receive_result() {
            Ok(result) => {
                results_received += 1;

                match result {
                    WorkResult::Success { generation_time, .. } => {
                        total_generation_time += generation_time;
                        successful_generations += 1;

                        let avg_time = if successful_generations > 0 {
                            total_generation_time / successful_generations
                        } else {
                            Duration::ZERO
                        };

                        main_progress_bar.set_message(format!(
                            "Generated: {}/{} | Avg: {:?}",
                            successful_generations, field_count, avg_time
                        ));
                    }
                    WorkResult::Error { e } => {
                        main_progress_bar.println(format!("Error in Worker Thread: {}", e));
                    }
                }

                main_progress_bar.inc(1);
            }
            Err(e) => {
                main_progress_bar.println(format!("Critical Error in Main Process: {}", e));
                break;
            }
        }
    }

    main_progress_bar.finish_with_message("Generation complete!");

    print_generation_summary(successful_generations, field_count, total_generation_time, output_dir, is_noguess);
    Ok(())
}

fn print_generation_summary(successful_count: u32, field_count: u32, total_time: Duration, output_dir: &str, is_noguess: bool) {
    println!("\n=== Generation Complete ===");
    println!("Successfully generated: {}/{} {}fields", successful_count, field_count, if is_noguess { "no-guess " } else { "" });

    if successful_count > 0 {
        let avg_generation_time = total_time / successful_count;
        println!("Average generation time per field: {:?}", avg_generation_time);
        println!("Total generation time (excluding overhead): {:?}", total_time);
    }

    println!("Output directory: {}", output_dir);
}

struct WorkTask {
    width: u32,
    height: u32,
    mine_spec: MineSweeperFieldCreation,
    output_dir: String,
    file_name_padding: usize,
    is_noguess: bool,

    start_index: u32,
    end_index: u32,
}

#[derive(Debug)]
enum WorkResult {
    Success {
        generation_time: Duration,
    },
    Error {
        e: std::io::Error,
    },
}

struct WorkerController {
    result_receiver: mpsc::Receiver<WorkResult>,
    multi_progress: MultiProgress,
}

impl WorkerController {
    fn new(
        width: u32,
        height: u32,
        mine_spec: MineSweeperFieldCreation,
        field_count: u32,
        output_dir: &str,
        is_noguess: bool,
        worker_count: u32,
    ) -> Self {
        let (result_sender, result_receiver) = mpsc::channel::<WorkResult>();
        let multi_progress = MultiProgress::new();

        // Create progress bars for each worker
        for worker_id in 0..worker_count {
            let pb = multi_progress.add(ProgressBar::new(field_count as u64 / worker_count as u64));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(&format!("{}", WORKER_PROGRESS_TEMPLATE).replace("{}", &worker_id.to_string()))
                    .unwrap()
                    .progress_chars(PROGRESS_CHARS)
            );
            pb.set_message("Waiting for tasks...");

            let result_sender = result_sender.clone();
            let task = Self::calculate_worker_task(width, height, mine_spec.clone(), field_count, output_dir, is_noguess, worker_id, worker_count);
            thread::spawn(move || {
                Self::worker_thread(task, result_sender, pb);
            });
        }

        Self {
            result_receiver,
            multi_progress,
        }
    }

    fn worker_thread(
        task: WorkTask,
        result_sender: mpsc::Sender<WorkResult>,
        progress_bar: ProgressBar,
    ) {
        let mut iteration: u32 = 0;
        while task.start_index + iteration < task.end_index {
            let task_id = task.start_index + iteration;
            progress_bar.set_message(format!("Generating field {} ({}x{})", task_id, task.width, task.height));

            let start_time = Instant::now();
            let result = Self::execute_task(&task, task_id);
            let generation_time = start_time.elapsed();

            let work_result = match result {
                Ok(()) => {
                    progress_bar.set_message(format!("Completed field {} in {:?}", task_id, generation_time));
                    WorkResult::Success {
                        generation_time,
                    }
                }
                Err(e) => {
                    progress_bar.set_message(format!("Failed field {}: {}",task_id, e));
                    progress_bar.println(e.to_string());
                    WorkResult::Error{e}
                }
            };
            progress_bar.inc(1);

            if result_sender.send(work_result).is_err() {
                progress_bar.finish_with_message("Main thread disconnected");
                break; // Main thread disconnected
            }
            iteration += 1;
        }
    }

    fn execute_task(task: &WorkTask, index: u32) -> io::Result<()> {
        let filename = if task.is_noguess {
            Path::new(&task.output_dir).join(format!("noguess_field_{:0width$}.minesweeper", index, width = task.file_name_padding)).to_string_lossy().to_string()
        } else {
            Path::new(&task.output_dir).join(format!("field_{:0width$}.minesweeper", index, width = task.file_name_padding)).to_string_lossy().to_string()
        };

        if task.is_noguess {
            let field = minesweeper_ng_field(task.width, task.height, task.mine_spec.clone());
            field.to_file(&filename)?;
        } else {
            let field = minesweeper_field(task.width, task.height, task.mine_spec.clone());
            field.to_file(&filename)?;
        }
        Ok(())
    }

    fn calculate_worker_task(
        width: u32,
        height: u32,
        mine_spec: MineSweeperFieldCreation,
        field_count: u32,
        output_dir: &str,
        is_noguess: bool,
        worker_id: u32,
        worker_count: u32
    ) -> WorkTask {
        let output_dir = output_dir.to_string();
        let file_name_padding = field_count.to_string().len();

        let start_index = field_count * worker_id / worker_count;
        let end_index = field_count * (worker_id + 1) / worker_count;

        WorkTask{width, height, mine_spec, output_dir, is_noguess, file_name_padding, start_index, end_index}
    }

    fn receive_result(&self) -> Result<WorkResult, mpsc::RecvError> {
        self.result_receiver.recv()
    }
}