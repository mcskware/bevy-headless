//! terminal stuff for headless mode ratatui

use std::io::Stdout;

use bevy_ecs::system::Resource;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

type MyTerminal = Terminal<CrosstermBackend<Stdout>>;

/// Holds the ratatui terminal as a resource
#[derive(Resource, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct TerminalResource {
    pub(crate) terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalResource {
    /// Gets the ratatui terminal as mutable
    pub fn get_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

/// Sets up our ratatui terminal
/// # Errors
/// If we cannot enter raw mode
#[allow(clippy::module_name_repetitions)]
pub fn setup_terminal() -> Result<MyTerminal, std::io::Error> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

/// Returns to the original terminal
/// # Errors
/// If we cannot revert from raw mode
#[allow(clippy::module_name_repetitions)]
pub fn restore_terminal(terminal: &mut MyTerminal) -> Result<(), std::io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()
}
