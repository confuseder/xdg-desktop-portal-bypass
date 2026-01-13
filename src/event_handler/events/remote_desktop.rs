use std::collections::HashMap;

use zbus::zvariant;

#[derive(Debug)]
pub enum RemoteDesktopEvent {
    SelectDevices(SelectDevices),
    Start(Start),
    NotifyPointerMotion(NotifyPointerMotion),
    NotifyPointerMotionAbsolute(NotifyPointerMotionAbsolute),
    NotifyPointerButton(NotifyPointerButton),
    NotifyPointerAxis(NotifyPointerAxis),
    NotifyPointerAxisDiscrete(NotifyPointerAxisDiscrete),
    NotifyKeyboardKeycode(NotifyKeyboardKeycode),
    NotifyKeyboardKeysym(NotifyKeyboardKeysym),
    GetPropertiesAvilableDeviceTypes,
    GetPropertiesVersion,
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
