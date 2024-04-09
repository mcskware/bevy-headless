# bevy-headless

This crate supports the [Bevy game engine](https://github.com/bevyengine/bevy) by adding a HeadlessPlugins plugin group.
This plugin group will set up similarly to the [MinimalPlugins](http://dev-docs.bevyengine.org/bevy/struct.MinimalPlugins.html) group, with the addition of a new headless plugin which uses the [ratatui](https://github.com/ratatui-org/ratatui) crate to display to the terminal.
You can retrieve the ratatui [Terminal](https://docs.rs/ratatui/latest/ratatui/terminal/index.html) via the TerminalResource's get_mut function.
You can also retrieve all logged messages via the AllLogs resource.
