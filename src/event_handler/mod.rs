use calloop::LoopSignal;
use futures::channel::oneshot;
use std::collections::HashMap;
use tracing::{debug, error};
use zbus::{
    Connection,
    zvariant::{self, OwnedObjectPath},
};

use crate::event_handler::events::remote_desktop::RemoteDesktopEvent;
use crate::event_handler::events::screen_cast::ScreenCastEvent;

pub mod events;
pub mod proxy;
pub mod server;

pub struct XdgBypass {
    pub config: XdgBypassConfig,

    pub stop_signal: LoopSignal,
    pub scheduler: calloop::futures::Scheduler<()>,
    pub connection: zbus::Connection,

    pub sessions: HashMap<OwnedObjectPath, Box<dyn EventHandler>>,
}

impl XdgBypass {
    pub fn new(
        config: XdgBypassConfig,
        stop_signal: LoopSignal,
        scheduler: calloop::futures::Scheduler<()>,
        connection: Connection,
    ) -> Self {
        Self {
            config,
            stop_signal,
            scheduler,
            connection,
            sessions: HashMap::new(),
        }
    }

    pub fn handle(&mut self, event: EventHandle) {
        debug!("Event: {:#?}", event);
    }
}

pub trait EventHandler {
    fn handle(&mut self, xdg_bypass: &mut XdgBypass, event: EventHandle) -> anyhow::Result<()>;

    fn new(
        xdg_bypass: &mut XdgBypass,
        session: OwnedObjectPath,
    ) -> anyhow::Result<Box<dyn EventHandler>>
    where
        Self: Sized;
}

pub struct XdgBypassConfig {
    pub remote_desktop_mode: WorkingMode,
}

pub enum WorkingMode {
    Server,
    Proxy(ProxyDestination),
}

pub struct ProxyDestination {
    service_name: String,
    object_path: zvariant::OwnedObjectPath,
}

#[derive(Debug)]
pub struct EventHandle {
    session: OwnedObjectPath,
    event: Event,
    return_tx: oneshot::Sender<EventResponse>,
}

pub fn return_response(
    return_tx: oneshot::Sender<EventResponse>,
    response: EventResponse,
    module: &str,
) {
    return_tx
        .send(response)
        .unwrap_or_else(|_| error!("[{}] Failed when return error", module))
}

pub enum EventResponse {
    Standard(u32, zvariant::OwnedValue),
    Value(zvariant::OwnedValue),
}

#[derive(Debug)]
pub enum Event {
    CreateSession(CreateSession),
    Close,
    RemoteDesktop(RemoteDesktopEvent),
    ScreenCast(ScreenCastEvent),
}

#[derive(Debug)]
pub struct CreateSession {
    pub handle: zvariant::ObjectPath<'static>,
    pub session_handle: zvariant::ObjectPath<'static>,
    pub app_id: String,
    pub options: HashMap<String, zvariant::OwnedValue>,
}
