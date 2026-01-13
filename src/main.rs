use anyhow::Context;
use calloop::signals::Signals;
use calloop::{channel, signals::Signal};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::event_handler::{EventHandle, WorkingMode, XdgBypass, XdgBypassConfig};

mod dbus_listener;
mod event_handler;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")))
        .try_init()
        .with_context(|| "Failed to init log subscriber")?;

    let config = XdgBypassConfig {
        remote_desktop_mode: WorkingMode::Server,
    };

    let (dbus_listener_tx, dbus_listener_rx) = channel::channel::<EventHandle>();

    info!("Event loop created");
    let mut event_loop = calloop::EventLoop::<XdgBypass>::try_new()
        .with_context(|| "Failed to create event loop")?;

    info!("DBus listener created");
    let dbus_listener = dbus_listener::DBusListener::new(dbus_listener_tx);

    info!("Async executor created");
    let (executor, scheduler) =
        calloop::futures::executor::<()>().with_context(|| "Fail too create async executor")?;

    let connection = futures::executor::block_on(async {
        zbus::connection::Builder::session().unwrap().build().await
    })
    .with_context(|| "Failed when create DBus Connection for proxy")?;
    info!("Event handler created");
    let mut event_handler = XdgBypass::new(config, event_loop.get_signal(), scheduler, connection);

    event_loop
        .handle()
        .insert_source(dbus_listener_rx, |event, _, state| {
            if let channel::Event::Msg(msg) = event {
                state.handle(msg);
            } else {
                state.stop_signal.stop();
            }
        })
        .map_err(|e| e.error)
        .with_context(|| "Failed to listen for DBus events")?;

    let force_close_signal = Signals::new(&[Signal::SIGINT, Signal::SIGTERM])
        .with_context(|| "Failed to create stop signal")?;

    event_loop
        .handle()
        .insert_source(executor, |_, _, _| {})
        .map_err(|e| e.error)
        .with_context(|| "Failed to start async executor in calloop")?;

    event_loop
        .handle()
        .insert_source(force_close_signal, |_, _, state| {
            state.stop_signal.stop();
        })
        .with_context(|| "Failed to listen for stop signals")?;

    info!("Event loop started");
    event_loop
        .run(None, &mut event_handler, |_| {})
        .with_context(|| "Failed to run event loop")?;

    Ok(())
}
