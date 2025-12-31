use crate::dbus_listener::Start;
use crate::event_handler::Event;
use crate::event_handler::remote_desktop::*;
use anyhow::Context;
use calloop::channel;
use std::collections::HashMap;
use tracing::debug;
use zbus::blocking::Connection;
use zbus::blocking::connection::Builder;
use zbus::interface;
use zbus::zvariant;

pub(crate) struct RemoteDesktopListener {
    sender: channel::Sender<Event>,
}

impl RemoteDesktopListener {
    pub fn new(sender: channel::Sender<Event>) -> Self {
        Self { sender }
    }
}

#[interface(name = "org.freedesktop.impl.portal.RemoteDesktop")]
impl RemoteDesktopListener {
    pub fn create_session(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        let _ = self
            .sender
            .send(Event::RemoteDesktop(RemoteDesktopEvent::CreateSession(
                CreateSession {
                    handle: handle.into_owned(),
                    session_handle: session_handle.into_owned(),
                    app_id,
                    options,
                },
            )));
        (0, HashMap::new())
    }

    pub fn select_devices(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        let _ = self
            .sender
            .send(Event::RemoteDesktop(RemoteDesktopEvent::SelectDevices(
                SelectDevices {
                    handle: handle.into_owned(),
                    session_handle: session_handle.into_owned(),
                    app_id,
                    options,
                },
            )));
        (0, HashMap::new())
    }

    pub fn start(
        &self,
        handle: zvariant::ObjectPath<'_>,
        session_handle: zvariant::ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> (u32, HashMap<String, zvariant::OwnedValue>) {
        let _ = self
            .sender
            .send(Event::RemoteDesktop(RemoteDesktopEvent::Start(
                crate::event_handler::remote_desktop::Start {
                    handle: handle.into_owned(),
                    session_handle: session_handle.into_owned(),
                    app_id,
                    parent_window,
                    options,
                },
            )));
        (0, HashMap::new())
    }

    pub fn notify_pointer_motion(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        dx: f64,
        dy: f64,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyPointerMotion(NotifyPointerMotion {
                session_handle: session_handle.into_owned(),
                options,
                dx,
                dy,
            }),
        ));
    }

    pub fn notify_pointer_motion_absolute(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        stream: u32,
        x: f64,
        y: f64,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyPointerMotionAbsolute(NotifyPointerMotionAbsolute {
                session_handle: session_handle.into_owned(),
                options,
                stream,
                x,
                y,
            }),
        ));
    }

    pub fn notify_pointer_button(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        button: i32,
        state: u32,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyPointerButton(NotifyPointerButton {
                session_handle: session_handle.into_owned(),
                options,
                button,
                state,
            }),
        ));
    }

    pub fn notify_pointer_axis(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        dx: f64,
        dy: f64,
    ) {
        let _ = self
            .sender
            .send(Event::RemoteDesktop(RemoteDesktopEvent::NotifyPointerAxis(
                NotifyPointerAxis {
                    session_handle: session_handle.into_owned(),
                    options,
                    dx,
                    dy,
                },
            )));
    }

    pub fn notify_pointer_axis_discrete(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        axis: u32,
        steps: i32,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyPointerAxisDiscrete(NotifyPointerAxisDiscrete {
                session_handle: session_handle.into_owned(),
                options,
                axis,
                steps,
            }),
        ));
    }

    pub fn notify_keyboard_keycode(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        keycode: i32,
        state: u32,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyKeyboardKeycode(NotifyKeyboardKeycode {
                session_handle: session_handle.into_owned(),
                options,
                keycode,
                state,
            }),
        ));
    }

    pub fn notify_keyboard_keysym(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        keysym: i32,
        state: u32,
    ) {
        let _ = self.sender.send(Event::RemoteDesktop(
            RemoteDesktopEvent::NotifyKeyboardKeysym(NotifyKeyboardKeysym {
                session_handle: session_handle.into_owned(),
                options,
                keysym,
                state,
            }),
        ));
    }

    /**
     * considered that no many devices need touch api
     * so if need we will add it later
     */
    pub fn notify_touch_down(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
    ) {
        todo!()
    }

    pub fn notify_touch_motion(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
    ) {
        todo!()
    }

    pub fn notify_touch_up(
        &self,
        session_handle: zvariant::ObjectPath<'_>,
        options: HashMap<String, zvariant::OwnedValue>,
        slot: u32,
    ) {
        todo!()
    }

    #[zbus(property)]
    pub fn available_device_types(&self) -> u32 {
        3 // Keyboard (1) | Pointer (2) | Touchscreen (4)
    }

    #[zbus(property)]
    pub fn version(&self) -> u32 {
        1
    }
}

impl Start for RemoteDesktopListener {
    fn start(self) -> anyhow::Result<Connection> {
        debug!("Starting RemoteDesktopListener");

        let connection = Builder::session()?
            .name("org.freedesktop.impl.portal.RemoteDesktop")?
            .serve_at("/org/freedesktop/portal/RemoteDesktop", self)?
            .build()
            .with_context(|| "Failed to start RemoteDesktopListener")?;

        Ok(connection)
    }
}
