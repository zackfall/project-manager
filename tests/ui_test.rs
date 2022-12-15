use std::io;
use tui::backend::TestBackend;

#[test]
fn ui_initialization_test() {
    let backend = TestBackend::new(100, 100);
    assert!(ui::run_ui(backend).is_ok());
}
