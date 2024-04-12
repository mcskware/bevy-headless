//! Things related to app exit

use bevy_app::AppExit;
use bevy_ecs::event::EventWriter;
use bevy_ecs::system::{Res, ResMut};
use bevy_ecs_macros::Resource;
use bevy_utils::tracing::error;
use bevy_utils::Duration;
use crossterm::event::{self, KeyModifiers};
use crossterm::event::{Event, KeyCode};

use super::terminal::{restore_terminal, TerminalResource};

/// Resource for holding state on whether we should quit or not
#[derive(Resource, Clone, Copy, Debug)]
pub struct ShouldQuit {
    pub(crate) result: PollResult,
}

impl ShouldQuit {
    /// Gets the result of whether we should quit or not
    #[must_use]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn result(&self) -> PollResult {
        self.result
    }
}

/// Should the application quit?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PollResult {
    /// Do not quit
    Continue,
    /// Quit
    Quit,
}

/// System for getting user input from ratatui terminal
/// # Panics
/// If we cannot poll the events
/// If we cannot get an event when poll says one is ready
pub fn poll_terminal_input(mut should_quit: ResMut<ShouldQuit>) {
    if event::poll(Duration::from_millis(0)).expect("Should read terminal events") {
        if let Event::Key(key) = event::read().expect("Should find an event") {
            if key_pressed(key, 'q') {
                should_quit.result = PollResult::Quit;
                return;
            }
            if key_pressed(key, 'c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                should_quit.result = PollResult::Quit;
                return;
            }
            // handle other keys here
        }
    }
    should_quit.result = PollResult::Continue;
}

fn key_pressed(key: event::KeyEvent, character: char) -> bool {
    KeyCode::Char(character) == key.code && key.kind == event::KeyEventKind::Press
}

/// Handle app exit
#[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
pub fn exit_system(
    mut exit: EventWriter<AppExit>,
    should_quit: Res<ShouldQuit>,
    mut terminal: ResMut<TerminalResource>,
) {
    if should_quit.result() == PollResult::Quit {
        let _event = exit.send(AppExit);
        if let Err(err) = restore_terminal(terminal.as_mut().get_mut()) {
            error!("Failed to restore terminal: {err}");
        }
    }
}
