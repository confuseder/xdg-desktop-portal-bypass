use anyhow::{Context, Result};
use evdev::{AttributeSet, KeyCode, RelativeAxisCode, uinput::VirtualDevice};
use std::collections::HashMap;
use tracing::debug;
use zbus::zvariant;

#[derive(Debug)]
pub enum RemoteDesktopEvent {
    CreateSession(CreateSession),
    SelectDevices(SelectDevices),
    Start(Start),
    NotifyPointerMotion(NotifyPointerMotion),
    NotifyPointerMotionAbsolute(NotifyPointerMotionAbsolute),
    NotifyPointerButton(NotifyPointerButton),
    NotifyPointerAxis(NotifyPointerAxis),
    NotifyPointerAxisDiscrete(NotifyPointerAxisDiscrete),
    NotifyKeyboardKeycode(NotifyKeyboardKeycode),
    NotifyKeyboardKeysym(NotifyKeyboardKeysym),
}

pub struct RemoteDesktopHandler {
    device: VirtualDevice,
}

impl RemoteDesktopHandler {
    pub fn new() -> Result<Self> {
        let mut keys = AttributeSet::<KeyCode>::new();
        for i in 0x000..0x2e7 {
            keys.insert(KeyCode::new(i as u16));
        }
        let mut mouse_axis = AttributeSet::<RelativeAxisCode>::new();
        for i in 0x00..0x0c {
            mouse_axis.insert(RelativeAxisCode(i as u16));
        }

        let mut builder = VirtualDevice::builder()?;
        builder = builder.name("xdg-desktop-portal-bypass vitural input device");

        debug!("Try to build vitural device");
        let device = builder
            .with_keys(&keys)?
            .with_relative_axes(&mouse_axis)?
            .build()
            .with_context(|| "Failed to create virtual device")?;

        Ok(Self { device })
    }

    pub fn handle(&self, event: RemoteDesktopEvent) {
        match event {
            RemoteDesktopEvent::CreateSession(_) => todo!(),
            RemoteDesktopEvent::SelectDevices(_) => todo!(),
            RemoteDesktopEvent::Start(_) => todo!(),
            RemoteDesktopEvent::NotifyPointerMotion(_) => todo!(),
            RemoteDesktopEvent::NotifyPointerMotionAbsolute(_) => todo!(),
            RemoteDesktopEvent::NotifyPointerButton(_) => todo!(),
            RemoteDesktopEvent::NotifyPointerAxis(_) => todo!(),
            RemoteDesktopEvent::NotifyPointerAxisDiscrete(_) => todo!(),
            RemoteDesktopEvent::NotifyKeyboardKeycode(_) => todo!(),
            RemoteDesktopEvent::NotifyKeyboardKeysym(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct CreateSession {
    pub handle: zvariant::ObjectPath<'static>,
    pub session_handle: zvariant::ObjectPath<'static>,
    pub app_id: String,
    pub options: HashMap<String, zvariant::OwnedValue>,
}

#[derive(Debug)]
pub struct SelectDevices {
    pub handle: zvariant::ObjectPath<'static>,
    pub session_handle: zvariant::ObjectPath<'static>,
    pub app_id: String,
    pub options: HashMap<String, zvariant::OwnedValue>,
}

#[derive(Debug)]
pub struct Start {
    pub handle: zvariant::ObjectPath<'static>,
    pub session_handle: zvariant::ObjectPath<'static>,
    pub app_id: String,
    pub parent_window: String,
    pub options: HashMap<String, zvariant::OwnedValue>,
}

#[derive(Debug)]
pub struct NotifyPointerMotion {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug)]
pub struct NotifyPointerMotionAbsolute {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub stream: u32,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct NotifyPointerButton {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub button: i32,
    pub state: u32,
}

#[derive(Debug)]
pub struct NotifyPointerAxis {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Debug)]
pub struct NotifyPointerAxisDiscrete {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub axis: u32,
    pub steps: i32,
}

#[derive(Debug)]
pub struct NotifyKeyboardKeycode {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub keycode: i32,
    pub state: u32,
}

#[derive(Debug)]
pub struct NotifyKeyboardKeysym {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub options: HashMap<String, zvariant::OwnedValue>,
    pub keysym: i32,
    pub state: u32,
}

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use evdev::{EventType, InputEvent, KeyCode, RelativeAxisCode};

    use crate::event_handler::remote_desktop::RemoteDesktopHandler;

    #[test]
    fn test_keyboard() {
        let mut handler = RemoteDesktopHandler::new().unwrap();

        sleep(Duration::from_secs(3));

        for _ in 0..3 {
            handler
                .device
                .emit(&[InputEvent::new(EventType::KEY.0, KeyCode::KEY_A.code(), 1)])
                .unwrap();
            handler
                .device
                .emit(&[InputEvent::new(EventType::KEY.0, KeyCode::KEY_A.code(), 0)])
                .unwrap();

            sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn test_mouse_move() {
        let mut handler = RemoteDesktopHandler::new().unwrap();

        sleep(Duration::from_secs(3));

        for _ in 0..3 {
            handler
                .device
                .emit(&[
                    InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, 100),
                    InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, 100),
                ])
                .unwrap();

            sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn test_mouse_whell() {
        let mut handler = RemoteDesktopHandler::new().unwrap();

        sleep(Duration::from_secs(3));

        for _ in 0..3 {
            handler
                .device
                .emit(&[
                    InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_WHEEL.0, 1),
                    InputEvent::new(
                        EventType::RELATIVE.0,
                        RelativeAxisCode::REL_WHEEL_HI_RES.0,
                        120,
                    ),
                ])
                .unwrap();

            sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn test_mouse_button() {
        let mut handler = RemoteDesktopHandler::new().unwrap();

        sleep(Duration::from_secs(3));

        for _ in 0..3 {
            handler
                .device
                .emit(&[InputEvent::new(
                    EventType::KEY.0,
                    KeyCode::BTN_LEFT.code(),
                    1,
                )])
                .unwrap();

            handler
                .device
                .emit(&[InputEvent::new(
                    EventType::KEY.0,
                    KeyCode::BTN_LEFT.code(),
                    0,
                )])
                .unwrap();

            sleep(Duration::from_millis(100));
        }

        sleep(Duration::from_secs(3));

        for _ in 0..3 {
            handler
                .device
                .emit(&[InputEvent::new(
                    EventType::KEY.0,
                    KeyCode::BTN_RIGHT.code(),
                    1,
                )])
                .unwrap();

            handler
                .device
                .emit(&[InputEvent::new(
                    EventType::KEY.0,
                    KeyCode::BTN_RIGHT.code(),
                    0,
                )])
                .unwrap();

            sleep(Duration::from_millis(100));
        }
    }
}
