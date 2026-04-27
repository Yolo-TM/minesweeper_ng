use super::params::Params;
use std::sync::{Arc, atomic::AtomicU32, mpsc};
use std::time::{Duration, Instant};

pub enum ProgressEvent {
    Done,
    AllDone,
}

pub enum Screen {
    Form {
        params: Params,
        focused: super::params::Field,
        error: Option<String>,
    },
    Generating {
        params: Params,
        total: u32,
        done: Arc<AtomicU32>,
        errors: Arc<AtomicU32>,
        rx: mpsc::Receiver<ProgressEvent>,
        start: Instant,
        finished: bool,
        elapsed: Duration,
    },
}
