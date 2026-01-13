use std::collections::HashMap;

use zbus::zvariant;

#[derive(Debug)]
pub enum ScreenCastEvent {
    SelectSources(SelectSources),
    Start(Start),
    OpenPipeWireRemote(OpenPipeWireRemote),
    GetPropertiesAvailableSourceTypes,
    GetPropertiesAvailableCursorModes,
    GetPropertiesVersion,
}

#[derive(Debug)]
pub struct SelectSources {
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
pub struct OpenPipeWireRemote {
    pub session_handle: zvariant::ObjectPath<'static>,
    pub app_id: String,
    pub options: HashMap<String, zvariant::OwnedValue>,
}
