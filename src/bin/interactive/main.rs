mod events;
mod popups;
mod render;
mod state;

use minesweeper_ng_gen::DefinedField;
use ratatui::{DefaultTerminal, layout::Rect};
use state::App;
use std::io;

fn main() -> io::Result<()> {
    simple_logger::init().unwrap();

    let args: Vec<String> = std::env::args().collect();

    let app = if args.len() > 1 && args[1] == "create" {
        let width: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
        let height: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(15);
        App::new_create(width, height)
    } else if args.len() > 1 {
        let field = DefinedField::from_file(&args[1])
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        App::new_play(field)
    } else {
        App::new_create(20, 15)
    };

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, app);
    ratatui::restore();

    if let Err(ref e) = result {
        eprintln!("Error: {}", e);
    }
    result
}

fn run(terminal: &mut DefaultTerminal, mut app: App) -> io::Result<()> {
    while !app.quit {
        terminal.draw(|frame| render::render(frame, &app))?;
        let size = terminal.size()?;
        let size_rect = Rect::new(0, 0, size.width, size.height);
        events::handle_event(&mut app, size_rect)?;
    }
    Ok(())
}
