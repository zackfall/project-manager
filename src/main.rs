#![warn(missing_docs)]

//! A CLI task manager writen with tui-rs, for personal use.

use std::io;
use tui::backend::CrosstermBackend;

fn main() -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let app = ui::run_ui(backend)?;
    Ok(app)
}
