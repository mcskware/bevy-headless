//! This example demonstrates how to create a headless application that logs messages to the terminal.

use bevy_app::{App, PluginGroup, ScheduleRunnerPlugin, Update};
use bevy_ecs::system::{Res, ResMut};
use bevy_headless::{log::AllLogs, terminal::TerminalResource, HeadlessPlugins};
use bevy_utils::{tracing::error, Duration};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

fn main() {
    let mut app = &mut App::new();

    app = app.add_plugins(HeadlessPlugins.set(ScheduleRunnerPlugin::run_loop(
        Duration::from_secs_f64(1.0 / 60.0),
    )));
    app = app.add_systems(Update, render);

    app.run();
}

#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
fn render(logs: Res<AllLogs>, mut terminal: ResMut<TerminalResource>) {
    let res = terminal.as_mut().get_mut().draw(|frame| {
        let block = Block::default().title("Greeting").borders(Borders::ALL);
        let scroll_count: u16 =
            logs.count() as u16 - (block.inner(frame.size()).height).min(logs.count() as u16);
        let greeting = Paragraph::new(logs.get())
            .block(block)
            .wrap(Wrap { trim: true });
        let greeting = greeting.scroll((scroll_count, 0));
        frame.render_widget(greeting, frame.size());
    });
    if let Err(err) = res {
        error!("Failed to render terminal: {err}");
    }
}
