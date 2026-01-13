// use anyhow::Context;
// use calloop::channel;
// use zbus::blocking::Connection;

// use crate::dbus_listener::WorkingMode;
// use crate::event_handler::Event;

// pub struct ScreenCastListener {
//     working_mode: WorkingMode,

//     sender: Option<channel::Sender<Event>>,

//     proxy_client: Option<Connection>,
// }

// impl ScreenCastListener {
//     pub fn new(working_mode: WorkingMode, sender: Option<channel::Sender<Event>>) -> Self {
//         match working_mode {
//             WorkingMode::Server => Self {
//                 working_mode,
//                 sender: sender
//                     .with_context(|| "No sender provided in server mode")
//                     .ok(),
//             },
//             WorkingMode::Proxy => Self {
//                 working_mode,
//                 sender: None,
//             },
//         }
//     }
// }
