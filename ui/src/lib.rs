// mod app;
// mod graphics;
// mod input_mode;

use std::io;
use tui::{backend::Backend, Terminal};

pub fn run_ui<T: Backend>(backend: T) -> Result<(), io::Error> {
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}
