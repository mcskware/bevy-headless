//! Provides the `AllLogs` resource for storing log messages in headless mode.

use std::sync::mpsc;

use bevy_app::{App, Update};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    system::{NonSend, ResMut},
};
use bevy_ecs_macros::Resource;
use bevy_utils::tracing::{self, Level, Subscriber};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer, Registry};

/// Holds received log messages
#[derive(Debug, Default, Resource)]
pub struct AllLogs {
    logs: Vec<String>,
}

impl AllLogs {
    /// Add a log to the record
    pub fn add_log(&mut self, message: String) {
        self.logs.push(message);
    }

    /// Gets all logs so far
    #[must_use]
    pub fn get(&self) -> String {
        self.logs.join("\r\n")
    }

    /// Gets the log count
    #[must_use]
    pub fn count(&self) -> usize {
        self.logs.len()
    }
}

/// System to transfer log events to a log holder
pub(crate) fn consume_log_events(mut events: EventReader<LogEvent>, mut logs: ResMut<AllLogs>) {
    for e in events.read() {
        logs.add_log(e.message.clone());
    }
}

/// A basic message. This is what we will be sending from the [`CaptureLayer`] to [`CapturedLogEvents`] non-send resource.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Event)]
pub(crate) struct LogEvent {
    message: String,
}

/// This non-send resource temporarily stores [`LogEvent`]s before they are
/// written to [`Events<LogEvent>`] by [`transfer_log_events`].
#[derive(Deref, DerefMut, Debug)]
pub(crate) struct CapturedLogEvents(pub mpsc::Receiver<LogEvent>);

/// Transfers information from the [`LogEvents`] resource to [`Events<LogEvent>`](LogEvent).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn transfer_log_events(
    receiver: NonSend<CapturedLogEvents>,
    mut log_events: EventWriter<LogEvent>,
) {
    // Make sure to use `try_iter()` and not `iter()` to prevent blocking.
    let _ = log_events.send_batch(receiver.try_iter());
}

/// This is the [`Layer`] that we will use to capture log events and then send them to Bevy's
/// ECS via it's [`mpsc::Sender`].
#[derive(Debug)]
pub(crate) struct CaptureLayer {
    /// tx for transfering logs to `CaptureLog`
    pub sender: mpsc::Sender<LogEvent>,
}
impl<S: Subscriber> Layer<S> for CaptureLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // In order to obtain the log message, we have to create a struct that implements
        // Visit and holds a reference to our string. Then we use the `record` method and
        // the struct to modify the reference to hold the message string.
        let mut message = None;
        event.record(&mut CaptureLayerVisitor(&mut message));
        if let Some(message) = message {
            // You can obtain metadata like this, but we wont use it for this example.
            let _metadata = event.metadata();
            self.sender
                .send(LogEvent { message })
                .expect("LogEvents resource no longer exists!");
        }
    }
}

/// A [`Visit`](tracing::field::Visit)or that records log messages that are transferred to [`CaptureLayer`].
struct CaptureLayerVisitor<'a>(&'a mut Option<String>);
impl tracing::field::Visit for CaptureLayerVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        // This if statement filters out unneeded events sometimes show up
        if field.name() == "message" {
            *self.0 = Some(format!("{value:?}"));
        }
    }
}

/// setup events, resources, systems for log capturing in headless mode
/// # Panics
/// If we are unable to set the log level
pub(crate) fn setup_logging(app: &mut App) {
    let (sender, receiver) = mpsc::channel();

    let layer = CaptureLayer { sender };
    let resource = CapturedLogEvents(receiver);

    let _ = app.insert_non_send_resource(resource);
    let _ = app.add_event::<LogEvent>();
    let _ = app.add_systems(Update, transfer_log_events);
    let _ = app.add_systems(Update, consume_log_events);
    let _ = app.insert_resource(AllLogs::default());

    let finished_subscriber;
    let default_filter = { format!("{},{}", Level::TRACE, "wgpu=warn,naga=debug") };
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&default_filter))
        .expect("Should be able to create log level filter");
    let subscriber = Registry::default().with(filter_layer);

    let subscriber = subscriber.with(layer);

    finished_subscriber = Box::new(subscriber);
    let _ = tracing::subscriber::set_global_default(finished_subscriber);
}
