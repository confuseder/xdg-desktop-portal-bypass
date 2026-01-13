use crate::dbus_listener::EventHandle;
use crate::dbus_listener::Start;
use crate::event_handler::Event;
use anyhow::Context;
use calloop::channel;
use futures::channel::oneshot;
use std::collections::HashMap;
use tracing::debug;
use zbus::blocking::Connection;
use zbus::blocking::connection::Builder;
use zbus::interface;
use zbus::zvariant;

pub struct RemoteDesktopListener {
    sender: channel::Sender<EventHandle>,
}

impl RemoteDesktopListener {
    pub fn new(sender: channel::Sender<EventHandle>) -> Self {
        Self { sender }
    }
}

#[interface(name = "org.freedesktop.impl.portal.RemoteDesktop")]
impl RemoteDesktopListener {
    async fn create_session(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zvariant::OwnedValue {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::CreateSession(CreateSession {
        //     handle: handle.into_owned(),
        //     session_handle: session_handle.into_owned(),
        //     app_id,
        //     options,
        // }));

        // debug!(
        //     "Interface called [RemoteDesktop.CreateSession] {:#?}",
        //     event
        // );

        // let (tx, rx) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));

        // match rx.await {
        //     Ok(value) => value,
        //     Err(_) => zvariant::OwnedValue::from(2),
        // }
        zvariant::OwnedValue::from(0)
    }

    async fn select_devices(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zvariant::OwnedValue {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::SelectDevices(SelectDevices {
        //     handle: handle.into_owned(),
        //     session_handle: session_handle.into_owned(),
        //     app_id,
        //     options,
        // }));

        // debug!(
        //     "Interface called [RemoteDesktop.SelectDevices] {:#?}",
        //     event
        // );

        // let (tx, rx) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));

        // match rx.await {
        //     Ok(value) => value,
        //     Err(_) => zvariant::OwnedValue::from(2),
        // }
        zvariant::OwnedValue::from(0)
    }

    async fn start(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zvariant::OwnedValue {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::Start(
        //     crate::event_handler::remote_desktop::Start {
        //         handle: handle.into_owned(),
        //         session_handle: session_handle.into_owned(),
        //         app_id,
        //         parent_window,
        //         options,
        //     },
        // ));

        // debug!("Interface called [RemoteDesktop.Start] {:#?}", event);

        // let (tx, rx) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));

        // match rx.await {
        //     Ok(value) => value,
        //     Err(_) => zvariant::OwnedValue::from(2),
        // }
        zvariant::OwnedValue::from(0)
    }

    pub fn notify_pointer_motion(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        dx: f64,
        dy: f64,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerMotion(
        //     NotifyPointerMotion {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         dx,
        //         dy,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyPointerMotion] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_pointer_motion_absolute(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        stream: u32,
        x: f64,
        y: f64,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerMotionAbsolute(
        //     NotifyPointerMotionAbsolute {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         stream,
        //         x,
        //         y,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyPointerMotionAbsolute] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_pointer_button(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        button: i32,
        state: u32,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerButton(
        //     NotifyPointerButton {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         button,
        //         state,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyPointerButton] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_pointer_axis(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        dx: f64,
        dy: f64,
    ) {
        // let event =
        //     Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerAxis(NotifyPointerAxis {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         dx,
        //         dy,
        //     }));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyPointerAxis] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_pointer_axis_discrete(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        axis: u32,
        steps: i32,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerAxisDiscrete(
        //     NotifyPointerAxisDiscrete {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         axis,
        //         steps,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyPointerAxisDiscrete] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_keyboard_keycode(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        keycode: i32,
        state: u32,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyKeyboardKeycode(
        //     NotifyKeyboardKeycode {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         keycode,
        //         state,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyKeyboardKeycode] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    pub fn notify_keyboard_keysym(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        keysym: i32,
        state: u32,
    ) {
        // let event = Event::RemoteDesktop(RemoteDesktopEvent::NotifyKeyboardKeysym(
        //     NotifyKeyboardKeysym {
        //         session_handle: session_handle.into_owned(),
        //         options,
        //         keysym,
        //         state,
        //     },
        // ));

        // debug!(
        //     "Interface called [RemoteDesktop.NotifyKeyboardKeysym] {:#?}",
        //     event
        // );

        // let (tx, _) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((event, tx));
    }

    /**
     * considered that no many devices need touch api
     * so if need we will add it later
     */

    #[zbus(property)]
    async fn available_device_types(&self) -> zvariant::OwnedValue {
        // let (tx, rx) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((
        //     Event::RemoteDesktop(RemoteDesktopEvent::GetPropertiesAvilableDeviceTypes(
        //         GetPropertiesAvilableDeviceTypes {},
        //     )),
        //     tx,
        // ));

        // match rx.await {
        //     Ok(value) => value,
        //     Err(_) => zvariant::OwnedValue::from(2),
        // }
        zvariant::OwnedValue::from(0)
    }

    #[zbus(property)]
    async fn version(&self) -> zvariant::OwnedValue {
        // let (tx, rx) = oneshot::channel::<zvariant::OwnedValue>();

        // self.sender.send((
        //     Event::RemoteDesktop(RemoteDesktopEvent::GetPropertiesVersion(
        //         GetPropertiesVersion {},
        //     )),
        //     tx,
        // ));

        // match rx.await {
        //     Ok(value) => value,
        //     Err(_) => zvariant::OwnedValue::from(2),
        // }
        zvariant::OwnedValue::from(0)
    }
}

impl Start for RemoteDesktopListener {
    fn start(self) -> anyhow::Result<Connection> {
        debug!("Starting RemoteDesktopListener");

        let connection = Builder::session()?
            .name("org.freedesktop.impl.portal.desktop.bypass")?
            .serve_at("/org/freedesktop/portal/desktop", self)?
            .build()
            .with_context(|| "Failed to start RemoteDesktopListener")?;

        Ok(connection)
    }
}
