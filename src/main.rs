#![warn(missing_docs)]

//! A CLI task manager writen with tui-rs, for personal use.

fn main() -> Result<(), std::io::Error> {
    ui::run_ui()
}
