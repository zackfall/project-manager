mod app;
mod graphics;
mod input_mode;

use std::io;
use std::{thread, time};
use tui::{backend::CrosstermBackend, Terminal};

pub fn run_ui() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}
