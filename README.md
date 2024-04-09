This crate supports the Bevy game engine by adding a HeadlessPlugins plugin group.
This plugin group will set up similarly to the MinimalPlugins group, with the addition of a new headless plugin which uses the ratatui crate to display to the terminal.
You can retrieve the ratatui Terminal via the TerminalResource's get_mut function.
You can also retrieve all logged messages via the AllLogs resource.
