mod events;
mod generation;
mod generator;
mod params;
mod render;
mod screen;
mod tui;

use std::io;

fn main() -> io::Result<()> {
    if std::env::args().len() > 1 {
        simple_logger::init().unwrap();
        generator::generate();
        Ok(())
    } else {
        tui::run_tui()
    }
}
