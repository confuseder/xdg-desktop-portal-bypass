use std::collections::HashMap;

use anyhow::Context;
use zbus::zvariant::{ObjectPath, OwnedValue};
use zbus::{proxy, zvariant};

use crate::event_handler::EventHandler;

#[proxy(interface = "org.freedesktop.impl.portal.ScreenCast")]
trait ScreenCastProxySenderTrait {
    #[zbus(property)]
    fn available_source_types(&self) -> zbus::fdo::Result<u32>;

    #[zbus(property)]
    fn available_cursor_modes(&self) -> zbus::fdo::Result<u32>;

    #[zbus(property)]
    fn version(&self) -> zbus::fdo::Result<u32>;

    fn create_session(
        &self,
        handle: ObjectPath<'static>,
        session_handle: ObjectPath<'static>,
        app_id: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)>;

    fn select_sources(
        &self,
        handle: ObjectPath<'static>,
        session_handle: ObjectPath<'static>,
        app_id: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)>;

    fn start(
        &self,
        handle: ObjectPath<'static>,
        session_handle: ObjectPath<'static>,
        app_id: String,
        parent_window: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)>;

    fn open_pipe_wire_remote(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<zbus::zvariant::OwnedFd>;
}

#[allow(dead_code)]
pub struct ScreenCastProxy {
    proxyed_session_handle: zvariant::OwnedObjectPath,
    proxy: ScreenCastProxySenderTraitProxy<'static>,
}

impl EventHandler for ScreenCastProxy {
    fn handle(
        &mut self,
        xdg_bypass: &mut crate::event_handler::XdgBypass,
        event: crate::event_handler::EventHandle,
    ) -> anyhow::Result<()> {
        match event.event {
            crate::event_handler::Event::ScreenCast(screen_cast_event) => {
                match screen_cast_event {
                    crate::event_handler::events::screen_cast::ScreenCastEvent::SelectSources(select_sources) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.select_sources(select_sources.handle, select_sources.session_handle, select_sources.app_id, select_sources.options).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(response.0, OwnedValue::from(response.1)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::screen_cast::ScreenCastEvent::Start(start) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.start(start.handle, start.session_handle, start.app_id, start.parent_window, start.options).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(response.0, OwnedValue::from(response.1)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::screen_cast::ScreenCastEvent::OpenPipeWireRemote(open_pipe_wire_remote) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            // Note: open_pipe_wire_remote signature in dbus usually doesn't take app_id, but the event has it.
                            // We ignore app_id here unless required. 
                            let response = this_proxy.open_pipe_wire_remote(open_pipe_wire_remote.session_handle, open_pipe_wire_remote.options).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Value(zvariant::Value::from(zvariant::Fd::from(response)).try_to_owned().unwrap()));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::screen_cast::ScreenCastEvent::GetPropertiesAvailableSourceTypes => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.available_source_types().await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Value(OwnedValue::from(response)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::screen_cast::ScreenCastEvent::GetPropertiesAvailableCursorModes => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.available_cursor_modes().await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Value(OwnedValue::from(response)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::screen_cast::ScreenCastEvent::GetPropertiesVersion => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.version().await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Value(OwnedValue::from(response)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                }
            }
            _ => Err(anyhow::anyhow!(
                "[ScreenCastProxy] Wrong event type, should be ScreenCast"
            )),
        }
    }

    fn new(
        xdg_bypass: &mut crate::event_handler::XdgBypass,
        session: zbus::zvariant::OwnedObjectPath,
    ) -> anyhow::Result<Box<dyn EventHandler>>
    where
        Self: Sized,
    {
        match &xdg_bypass.config.remote_desktop_mode {
            crate::event_handler::WorkingMode::Proxy(destination) => {
                let proxy = futures::executor::block_on(async {
                    ScreenCastProxySenderTraitProxy::builder(&xdg_bypass.connection.clone().into())
                        .destination(destination.service_name.clone())?
                        .path(destination.object_path.clone())?
                        .build()
                        .await
                })
                .with_context(|| "[ScreenCastProxy] Fail to connect to proxy destination.")?;
                Ok(Box::new(Self {
                    proxyed_session_handle: session,
                    proxy,
                }))
            }
            crate::event_handler::WorkingMode::Server => Err(anyhow::anyhow!(
                "[ScreenCastProxy] Wrong handler type, should be proxy"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gnome_screencast_proxy() {
        let connection = zbus::Connection::session().await.unwrap();
        let proxy = ScreenCastProxySenderTraitProxy::builder(&connection)
            .destination("org.freedesktop.impl.portal.desktop.gnome")
            .unwrap()
            .path("/org/freedesktop/portal/desktop")
            .unwrap()
            .build()
            .await
            .unwrap();

        let available_source_types = proxy.available_source_types().await.unwrap();
        println!("Available source types: {}", available_source_types);

        let available_cursor_modes = proxy.available_cursor_modes().await.unwrap();
        println!("Available cursor modes: {}", available_cursor_modes);

        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/123/456")
            .unwrap()
            .into();
        let session_handle =
            ObjectPath::try_from("/org/freedesktop/portal/desktop/session/123/456")
                .unwrap()
                .into();
        let app_id = "org.example.Test".to_string();
        let options = HashMap::new();

        let (response, results) = proxy
            .create_session(handle, session_handle, app_id, options)
            .await
            .unwrap();
        println!(
            "CreateSession response: {}, results: {:?}",
            response, results
        );
    }
}
