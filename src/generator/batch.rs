use std::sync::mpsc::{Sender, Receiver};
use std::{fs, io, thread, path::Path, sync::mpsc};
use std::time::Instant;
use super::CommandResult;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use crate::{RandomField, minesweeper_ng_field, MineSweeperField};

const DEFAULT_PROGRESS_TEMPLATE: &str = "Overall: [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | {wide_msg}";
const WORKER_PROGRESS_TEMPLATE: &str = "Worker {}: {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)";
const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏ .";

pub fn generate_fields(field_data: CommandResult) {
    setup_output_directory(&field_data.output).unwrap_or_else(|err| {
        eprintln!("Error setting up output directory '{}': {}", field_data.output, err);
        return;
    });

    let (worker_count, workload) = get_worker_infos(&field_data);
    println!("Using {} worker threads with {} fields per worker_count\n", worker_count, workload);

    let multi_progress = MultiProgress::new();
    let (tx, rx) : (Sender<()>, Receiver<()>)= mpsc::channel();

    let overall_progress = multi_progress.add(ProgressBar::new(field_data.count as u64));
    overall_progress.set_style(ProgressStyle::with_template(DEFAULT_PROGRESS_TEMPLATE).unwrap().progress_chars(PROGRESS_CHARS));
    overall_progress.set_message("Starting generation...");

    start_workers(
        worker_count,
        workload,
        &field_data,
        tx,
        &multi_progress,
    );

    overall_progress.set_message("Generating fields...");

    let start_time = Instant::now();
    for _ in 0..field_data.count {
        rx.recv().unwrap();
        overall_progress.inc(1);
    }
    overall_progress.finish_with_message(format!("Generation completed in {:.2?}", start_time.elapsed()));
    println!("Saved Files to '{}'", field_data.output);
}

fn setup_output_directory(output: &String) -> io::Result<()> {
    if Path::new(&output).exists() {
        println!("Directory '{}' already exists. Files will be added/overwritten.", output);
    } else {
        fs::create_dir_all(&output).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create directory '{}': {}", output, e)
            )
        })?;
        println!("Created directory: {}", output);
    }

    Ok(())
}

fn get_worker_infos(field_data: &CommandResult) -> (u32, u32) {
    let available_cpus = num_cpus::get() as u32;

    let worker_count = if field_data.count >= available_cpus {
            if field_data.no_guess {
                // keep cores free for multithreaded solving
                available_cpus / 2
            } else {
                available_cpus
            }
        } else {
            field_data.count
        };

    // worker_count - 1 is added so the division rounds up instead of down, the last worker does less work
    let workload = (field_data.count + worker_count - 1) / worker_count;
    (worker_count, workload)
}

fn start_workers(
    worker_count: u32,
    workload: u32,
    field_data: &CommandResult,
    tx: mpsc::Sender<()>,
    multi_progress: &MultiProgress,
) {
    for i in 0..worker_count {
        let workload_ids = i * workload + 1 ..=((i + 1) * workload).min(field_data.count);
        if workload_ids.start() > workload_ids.end() {
            continue; // our ceiling division created an extra worker with no work
        }

        let field_data = field_data.clone();
        let tx = tx.clone();
        let multi_progress = multi_progress.clone();
        thread::spawn({move || {
                    worker(i, workload_ids, field_data, tx, &multi_progress);
                }
            }
        );
    }
}

fn worker(
    worker_id: u32,
    ids: std::ops::RangeInclusive<u32>,
    field_data: CommandResult,
    tx: mpsc::Sender<()>,
    multi_progress: &MultiProgress,
) {
    let progress_bar = multi_progress.add(ProgressBar::new(ids.clone().count() as u64));
    progress_bar.set_style(ProgressStyle::with_template(&WORKER_PROGRESS_TEMPLATE.replace("{}", &worker_id.to_string())).unwrap().progress_chars(PROGRESS_CHARS));
    for id in ids {
        if field_data.no_guess {
            let field = minesweeper_ng_field(
                field_data.width,
                field_data.height,
                field_data.mine_spec,
            );
            let filename = format!("{}/{}.minesweeper", field_data.output, id);
            field.to_file(&filename).unwrap_or_else(|err| {
                multi_progress.println(format!("Error writing to file {}: {}", filename, err)).unwrap();
            });
        } else {
            let field = RandomField::new(
                field_data.width,
                field_data.height,
                field_data.mine_spec,
            );
            let filename = format!("{}/{}.minesweeper", field_data.output, id);
            field.to_file(&filename).unwrap_or_else(|err| {
                multi_progress.println(format!("Error writing to file {}: {}", filename, err)).unwrap();
            });
        }

        progress_bar.inc(1);
        tx.send(()).unwrap();
    }
}