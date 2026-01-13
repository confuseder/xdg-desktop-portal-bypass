use crate::{
    dbus_listener::remote_desktop_listener::RemoteDesktopListener,
    event_handler::{Event, EventHandle},
};
use calloop::channel;
use tracing::error;
use zbus::blocking::Connection;

mod remote_desktop_listener;
mod screen_cast_listener;

pub struct DBusListener {
    pub remote_desktop: Option<Connection>,
}

impl DBusListener {
    pub fn new(channel: channel::Sender<EventHandle>) -> Self {
        let remote_desktop = RemoteDesktopListener::new(channel);
        let remote_desktop = remote_desktop
            .start()
            .map_err(|e| {
                error!("Can't start remote desktop listener: {:#?}", e);
            })
            .ok();

        Self { remote_desktop }
    }
}

pub trait Start {
    fn start(self) -> anyhow::Result<Connection>;
}
