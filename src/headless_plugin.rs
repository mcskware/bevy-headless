//! Custom logger for ratatui mode

use std::io::Stdout;

use bevy_app::{App, Plugin, Update};
use ratatui::prelude::*;

use super::{
    exit::{exit_system, poll_terminal_input, PollResult, ShouldQuit},
    log::setup_logging,
    terminal::{setup_terminal, TerminalResource},
};

/// Custom logger for headless mode
#[derive(Debug, Clone, Copy)]
pub struct HeadlessPlugin;

impl Plugin for HeadlessPlugin {
    fn build(&self, app: &mut App) {
        setup_logging(app);

        let _ = app.add_systems(Update, poll_terminal_input);
        let _ = app.insert_resource(ShouldQuit {
            result: PollResult::Continue,
        });
        let _ = app.add_systems(Update, exit_system);

        let terminal: Terminal<CrosstermBackend<Stdout>> =
            setup_terminal().expect("Terminal should be able to set up");
        let _ = app.insert_resource(TerminalResource { terminal });
    }
}
