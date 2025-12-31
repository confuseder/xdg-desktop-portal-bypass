use crate::event_handler::Event;
use calloop::channel;
use tracing::error;
use zbus::blocking::Connection;

mod remote_desktop_listener;

pub struct DBusListener {
    pub remote_desktop: Option<Connection>,
}

impl DBusListener {
    pub fn new(channel: channel::Sender<Event>) -> Self {
        let remote_desktop = remote_desktop_listener::RemoteDesktopListener::new(channel);
        let remote_desktop = remote_desktop
            .start()
            .map_err(|e| {
                error!("Can't start remote desktop listener: {}", e);
            })
            .ok();

        Self { remote_desktop }
    }
}

pub trait Start {
    fn start(self) -> anyhow::Result<Connection>;
}
