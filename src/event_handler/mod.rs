use calloop::{LoopSignal, signals::Signals};
use tracing::{debug, error};

use crate::event_handler::remote_desktop::{RemoteDesktopEvent, RemoteDesktopHandler};

pub mod remote_desktop;

pub struct EventHandler {
    pub stop_signal: LoopSignal,

    remote_desktop: Option<RemoteDesktopHandler>,
}

#[derive(Debug)]
pub enum Event {
    RemoteDesktop(RemoteDesktopEvent),
}

impl EventHandler {
    pub fn new(stop_signal: LoopSignal) -> Self {
        Self {
            stop_signal,

            remote_desktop: RemoteDesktopHandler::new().ok(),
        }
    }

    pub fn handle(&self, event: Event) {
        debug!("Event: {:#?}", event);
        match event {
            Event::RemoteDesktop(event) => match &self.remote_desktop {
                Some(remote_desktop) => remote_desktop.handle(event),
                None => error!("RemoteDesktopHandler is not initialized"),
            },
        }
    }
}
