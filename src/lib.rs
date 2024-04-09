//! Plugin group for running in headless mode with ratatui.
//! Use this plugin group instead of [MinimalPlugins](https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html) to run in headless mode.
//! Includes a resource of type `TerminalResource` for ratatui terminal access.
//! Includes a resource of type `ShouldQuit` for quitting the application.
//! Includes a resource of type `AllLogs` for retrieving all logged messages.

use bevy_app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use bevy_time::TimePlugin;

mod exit;
mod headless_plugin;
pub mod log;
pub mod terminal;

/// Plugin Group for a headless server using a ratatui frontend.
/// Basically the `MinimalPlugins` plus the `HeadlessPlugin`
#[derive(Clone, Copy, Debug)]
pub struct HeadlessPlugins;

impl PluginGroup for HeadlessPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin)
            .add(FrameCountPlugin)
            .add(TimePlugin)
            .add(ScheduleRunnerPlugin::default())
            .add(headless_plugin::HeadlessPlugin)
    }
}
