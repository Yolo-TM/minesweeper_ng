use super::params::Params;
use super::screen::ProgressEvent;
use minesweeper_ng_gen::*;
use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
        mpsc,
    },
};

pub fn start_generation(
    params: &Params,
) -> (
    Arc<AtomicU32>,
    Arc<AtomicU32>,
    mpsc::Receiver<ProgressEvent>,
) {
    let total = params.count_val().unwrap();
    let w = params.width_val().unwrap();
    let h = params.height_val().unwrap();
    let mines = params.mine_spec().unwrap();
    let no_guess = params.no_guess;
    let output = params.validate().unwrap();

    let done = Arc::new(AtomicU32::new(0));
    let errors = Arc::new(AtomicU32::new(0));
    let done2 = done.clone();
    let errors2 = errors.clone();

    let (tx, rx) = mpsc::channel::<ProgressEvent>();

    rayon::spawn(move || {
        fs::create_dir_all(&output).ok();

        let is_single = total == 1;
        rayon::scope(|s| {
            for id in 1..=total {
                let tx = tx.clone();
                let mines = mines.clone();
                let output = output.clone();
                let done2 = done2.clone();
                let errors2 = errors2.clone();
                s.spawn(move |_| {
                    let filename = if is_single {
                        format!("{}.minesweeper", output)
                    } else {
                        format!("{}/{}.minesweeper", output, id)
                    };
                    let success = if no_guess {
                        NoGuessField::new(w, h, mines)
                            .and_then(|f| f.to_file(&filename).map_err(|e| e.into()))
                            .is_ok()
                    } else {
                        RandomField::new(w, h, mines)
                            .and_then(|f| f.to_file(&filename).map_err(|e| e.into()))
                            .is_ok()
                    };
                    if !success {
                        errors2.fetch_add(1, Ordering::Relaxed);
                    }
                    done2.fetch_add(1, Ordering::Relaxed);
                    let _ = tx.send(ProgressEvent::Done);
                });
            }
        });
        let _ = tx.send(ProgressEvent::AllDone);
    });

    (done, errors, rx)
}
