use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

mod ui;

fn main() {
    create_ui();
}

async fn create_ui() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    ui::draw_main_layout(&terminal);

    loop {
        if let Ok(size) = terminal.backend().size() {
            terminal.autoresize();
        }
    }
}
