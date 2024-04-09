# bevy-headless

This crate supports the [Bevy game engine](https://github.com/bevyengine/bevy) by adding a HeadlessPlugins plugin group.
This plugin group will set up similarly to the [MinimalPlugins](http://dev-docs.bevyengine.org/bevy/struct.MinimalPlugins.html) group, with the addition of a new headless plugin which uses the [ratatui](https://github.com/ratatui-org/ratatui) crate to display to the terminal.
You can retrieve the ratatui [Terminal](https://docs.rs/ratatui/latest/ratatui/terminal/index.html) via the TerminalResource's get_mut function.
You can also retrieve all logged messages via the AllLogs resource.

## Example

Here's a simple example which uses clap to accept a command line parameter to launch in windowed or headless mode:

```rust
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_headless::HeadlessPlugins;
use bevy_utils::Duration;
use clap::Parser;
use bevy_headless::log::AllLogs;
use bevy_headless::terminal::TerminalResource;

use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArgs {
    #[arg(short, long)]
    server: bool,
}

fn main() {
    let program_args = ProgramArgs::parse();

    let mut app = &mut App::new();

    if program_args.server {
        app = app.add_plugins(HeadlessPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )));
        app = app.add_systems(Update, render);
    } else {
        app = app.add_plugins(DefaultPlugins);
    };

    app.run();
}

pub fn render(logs: Res<AllLogs>, mut terminal: ResMut<TerminalResource>) {
    let _ = terminal.as_mut().get_mut().draw(|frame| {
        let block = Block::default().title("Greeting").borders(Borders::ALL);
        let scroll_count: u16 =
            logs.count() as u16 - (block.inner(frame.size()).height).min(logs.count() as u16);
        let greeting = Paragraph::new(logs.get())
            .block(block)
            .wrap(Wrap { trim: true });
        let greeting = greeting.scroll((scroll_count, 0));
        frame.render_widget(greeting, frame.size());
    });
}
```

In this example, we add the HeadlessPlugins group, and set our schedule runner to tick 60 times per second.
