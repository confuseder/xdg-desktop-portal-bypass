use anyhow::Context;
use calloop::signals::Signals;
use calloop::{channel, signals::Signal};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::event_handler::{Event, EventHandler};

mod dbus_listener;
mod event_handler;

fn main() -> anyhow::Result<()> {
    // 1. 设置日志初始化逻辑
    tracing_subscriber::registry()
        .with(fmt::layer()) // 输出到控制台
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")), // 如果没设置 RUST_LOG，默认使用 debug
        )
        .try_init()
        .with_context(|| "Failed to init tracing")?;

    let (dbus_listener_tx, dbus_listener_rx) = channel::channel::<Event>();

    info!("Event loop created");
    let mut event_loop = calloop::EventLoop::<EventHandler>::try_new()
        .with_context(|| "Failed to create event loop")?;

    info!("DBus listener created");
    let dbus_listener = dbus_listener::DBusListener::new(dbus_listener_tx);

    info!("Event handler created");
    let mut event_handler = EventHandler::new(event_loop.get_signal());

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
