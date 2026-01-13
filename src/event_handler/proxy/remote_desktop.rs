use std::collections::HashMap;

use anyhow::Context;
use zbus::zvariant::{ObjectPath, OwnedValue};
use zbus::{proxy, zvariant};

use crate::event_handler::EventHandler;

#[proxy(interface = "org.freedesktop.portal.RemoteDesktop")]
trait RemoteDesktopProxySenderTrait {
    #[zbus(property)]
    fn available_device_types(&self) -> zbus::fdo::Result<u32>;

    #[zbus(property)]
    fn version(&self) -> zbus::fdo::Result<u32>;

    fn create_session(
        &self,
        handle: ObjectPath<'static>,
        session_handle: ObjectPath<'static>,
        app_id: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)>;

    fn select_devices(
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

    fn notify_pointer_motion(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        dx: f64,
        dy: f64,
    ) -> zbus::fdo::Result<()>;

    fn notify_pointer_motion_absolute(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        stream: u32,
        x: f64,
        y: f64,
    ) -> zbus::fdo::Result<()>;

    fn notify_pointer_button(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        button: i32,
        state: u32,
    ) -> zbus::fdo::Result<()>;

    fn notify_pointer_axis(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        dx: f64,
        dy: f64,
    ) -> zbus::fdo::Result<()>;

    fn notify_pointer_axis_discrete(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        axis: u32,
        steps: i32,
    ) -> zbus::fdo::Result<()>;

    fn notify_keyboard_keycode(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        keycode: i32,
        state: u32,
    ) -> zbus::fdo::Result<()>;

    fn notify_keyboard_keysym(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        keysym: i32,
        state: u32,
    ) -> zbus::fdo::Result<()>;

    fn notify_touch_down(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
    ) -> zbus::fdo::Result<()>;

    fn notify_touch_motion(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
    ) -> zbus::fdo::Result<()>;

    fn notify_touch_up(
        &self,
        session_handle: ObjectPath<'static>,
        options: HashMap<String, OwnedValue>,
        slot: u32,
    ) -> zbus::fdo::Result<()>;

    fn connect_to_eis(
        &self,
        session_handle: ObjectPath<'static>,
        app_id: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<zbus::zvariant::OwnedFd>;
}

#[allow(dead_code)]
pub struct RemoteDesktopProxy {
    proxyed_session_handle: zvariant::OwnedObjectPath,
    proxy: RemoteDesktopProxySenderTraitProxy<'static>,
}

impl EventHandler for RemoteDesktopProxy {
    fn handle(
        &mut self,
        xdg_bypass: &mut crate::event_handler::XdgBypass,
        event: crate::event_handler::EventHandle,
    ) -> anyhow::Result<()> {
        match event.event {
            crate::event_handler::Event::RemoteDesktop(remote_desktop_event) => {
                match remote_desktop_event {
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::SelectDevices(select_devices) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let respone = this_proxy.select_devices(select_devices.handle, select_devices.session_handle, select_devices.app_id, select_devices.options).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(respone.0, OwnedValue::from(respone.1)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::Start(start) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.start(start.handle, start.session_handle, start.app_id, start.parent_window, start.options).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(response.0, OwnedValue::from(response.1)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyPointerMotion(notify_pointer_motion) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_pointer_motion(notify_pointer_motion.session_handle, notify_pointer_motion.options, notify_pointer_motion.dx, notify_pointer_motion.dy).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyPointerMotionAbsolute(notify_pointer_motion_absolute) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_pointer_motion_absolute(notify_pointer_motion_absolute.session_handle, notify_pointer_motion_absolute.options, notify_pointer_motion_absolute.stream, notify_pointer_motion_absolute.x, notify_pointer_motion_absolute.y).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyPointerButton(notify_pointer_button) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_pointer_button(notify_pointer_button.session_handle, notify_pointer_button.options, notify_pointer_button.button, notify_pointer_button.state).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyPointerAxis(notify_pointer_axis) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_pointer_axis(notify_pointer_axis.session_handle, notify_pointer_axis.options, notify_pointer_axis.dx, notify_pointer_axis.dy).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyPointerAxisDiscrete(notify_pointer_axis_discrete) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_pointer_axis_discrete(notify_pointer_axis_discrete.session_handle, notify_pointer_axis_discrete.options, notify_pointer_axis_discrete.axis, notify_pointer_axis_discrete.steps).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyKeyboardKeycode(notify_keyboard_keycode) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_keyboard_keycode(notify_keyboard_keycode.session_handle, notify_keyboard_keycode.options, notify_keyboard_keycode.keycode, notify_keyboard_keycode.state).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::NotifyKeyboardKeysym(notify_keyboard_keysym) => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            this_proxy.notify_keyboard_keysym(notify_keyboard_keysym.session_handle, notify_keyboard_keysym.options, notify_keyboard_keysym.keysym, notify_keyboard_keysym.state).await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Standard(0, OwnedValue::from(HashMap::<String, OwnedValue>::new())));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::GetPropertiesAvilableDeviceTypes => {
                        let this_proxy = self.proxy.clone();
                        xdg_bypass.scheduler.schedule(async move {
                            let response = this_proxy.available_device_types().await.unwrap();
                            let _ = event.return_tx.send(crate::event_handler::EventResponse::Value(OwnedValue::from(response)));
                        }).with_context(|| "")?;
                        Ok(())
                    },
                    crate::event_handler::events::remote_desktop::RemoteDesktopEvent::GetPropertiesVersion => {
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
                "[RemoteDesktopProxy] Wrong event type, should be RemoteDesktop"
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
                    RemoteDesktopProxySenderTraitProxy::builder(
                        &xdg_bypass.connection.clone().into(),
                    )
                    .destination(destination.service_name.clone())?
                    .path(destination.object_path.clone())?
                    .build()
                    .await
                })
                .with_context(|| "[RemoteDesktopPoxy] Fail to connect to proxy destination.")?;
                Ok(Box::new(Self {
                    proxyed_session_handle: session,
                    proxy,
                }))
            }
            crate::event_handler::WorkingMode::Server => Err(anyhow::anyhow!(
                "[RemoteDesktopProxy] Wrong handler type, should be proxy"
            )),
        }
    }
}
