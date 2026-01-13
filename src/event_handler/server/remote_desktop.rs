use std::collections::HashMap;

use anyhow::Context;
use evdev::AttributeSet;
use evdev::EventType;
use evdev::InputEvent;
use evdev::KeyCode;
use evdev::RelativeAxisCode;
use evdev::uinput::VirtualDevice;
use tracing::debug;
use tracing::error;
use zbus::zvariant;
use zbus::zvariant::OwnedValue;
use zbus::zvariant::Value;

use crate::event_handler::Event;
use crate::event_handler::EventHandle;
use crate::event_handler::EventHandler;
use crate::event_handler::EventResponse;
use crate::event_handler::events::remote_desktop::RemoteDesktopEvent;
use crate::event_handler::return_response;

pub struct RemoteDesktopServer {
    device_select: u32,
    device: Option<VirtualDevice>,
}

impl EventHandler for RemoteDesktopServer {
    fn handle(
        &mut self,
        xdg_bypass: &mut crate::event_handler::XdgBypass,
        event_handle: EventHandle,
    ) -> anyhow::Result<()> {
        let EventHandle {
            event,
            return_tx: to_return,
            ..
        } = event_handle;

        match event {
            Event::RemoteDesktop(remote_desktop_event) => match remote_desktop_event {
                RemoteDesktopEvent::SelectDevices(select_devices) => {
                    if let Some(types) = select_devices
                        .options
                        .get("types")
                        .and_then(|v| v.clone().try_into().ok())
                    {
                        if types > 3 {
                            error!("[RemoteDesktop.SelectDevices] Unsupport touch device now.");
                            return_response(
                                to_return,
                                EventResponse::Standard(
                                    2,
                                    Value::from("Input vailed is uncorect.").try_into_owned()?,
                                ),
                                "RemoteDesktop.SelectDevices",
                            );
                        } else {
                            self.device_select = types;
                            return_response(
                                to_return,
                                EventResponse::Standard(
                                    0,
                                    OwnedValue::from(HashMap::<String, zvariant::OwnedValue>::new()),
                                ),
                                "RemoteDesktop.SelectDevices",
                            );
                        }
                    } else {
                        error!("[RemoteDesktop.SelectDevices] Failed when realizing inputs.");
                        return_response(
                            to_return,
                            EventResponse::Standard(
                                2,
                                Value::from("Input vailed is uncorect.").try_into_owned()?,
                            ),
                            "RemoteDesktop.SelectDevices",
                        );
                    }
                }
                RemoteDesktopEvent::Start(start) => {
                    let mut keys = AttributeSet::<KeyCode>::new();
                    if self.device_select & 1 != 0 {
                        for i in 0x000..0x2e7 {
                            keys.insert(KeyCode::new(i as u16));
                        }
                    }
                    let mut mouse_axis = AttributeSet::<RelativeAxisCode>::new();
                    if self.device_select & 2 != 0 {
                        for i in 0x00..0x0c {
                            mouse_axis.insert(RelativeAxisCode(i as u16));
                        }
                    }

                    let mut builder = VirtualDevice::builder()?;
                    builder = builder.name("xdg-desktop-portal-bypass vitural input device");

                    debug!("[RemoteDesktop.Start] Try to build virtual device.");
                    if let Ok(device) = builder
                        .with_keys(&keys)?
                        .with_relative_axes(&mouse_axis)?
                        .build()
                        .with_context(|| "[RemoteDesktop.Start] Failed to create virtual device.")
                    {
                        self.device = Some(device);
                        return_response(
                            to_return,
                            EventResponse::Standard(
                                0,
                                OwnedValue::from(HashMap::<String, zvariant::OwnedValue>::new()),
                            ),
                            "RemoteDesktop.Start",
                        );
                    } else {
                        error!("[RemoteDesktop.Start] Failed to create virtual device.");
                        return_response(
                            to_return,
                            EventResponse::Standard(
                                2,
                                Value::from("Failed to create virtual device.").try_into_owned()?,
                            ),
                            "RemoteDesktop.Start",
                        );
                    };
                }
                RemoteDesktopEvent::NotifyPointerMotion(notify_pointer_motion) => {
                    if let Some(device) = &mut self.device {
                        let _ = device.emit(&[
                            InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_X.0,
                                notify_pointer_motion.dx as i32,
                            ),
                            InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_Y.0,
                                notify_pointer_motion.dy as i32,
                            ),
                        ]);
                    } else {
                        error!(
                            "[RemoteDesktop.NotifyPointerMotion] No virtual device was created."
                        );
                    }
                }
                RemoteDesktopEvent::NotifyPointerMotionAbsolute(notify_pointer_motion_absolute) => {
                    if let Some(device) = &mut self.device {
                        let _ = device.emit(&[
                            InputEvent::new(
                                EventType::ABSOLUTE.0,
                                0x00,
                                notify_pointer_motion_absolute.x as i32,
                            ), // ABS_X
                            InputEvent::new(
                                EventType::ABSOLUTE.0,
                                0x01,
                                notify_pointer_motion_absolute.y as i32,
                            ), // ABS_Y
                        ]);
                    } else {
                        error!(
                            "[RemoteDesktop.NotifyPointerMotionAbsolute] No virtual device was created."
                        );
                    }
                }
                RemoteDesktopEvent::NotifyPointerButton(notify_pointer_button) => {
                    if let Some(device) = &mut self.device {
                        // button: 1=left, 2=right, 3=middle
                        let btn_code = match notify_pointer_button.button {
                            1 => 0x110, // BTN_LEFT
                            2 => 0x111, // BTN_RIGHT
                            3 => 0x112, // BTN_MIDDLE
                            _ => notify_pointer_button.button as u16,
                        };
                        let _ = device.emit(&[InputEvent::new(
                            EventType::KEY.0,
                            btn_code,
                            notify_pointer_button.state as i32,
                        )]);
                    } else {
                        error!(
                            "[RemoteDesktop.NotifyPointerButton] No virtual device was created."
                        );
                    }
                }
                RemoteDesktopEvent::NotifyPointerAxis(notify_pointer_axis) => {
                    if let Some(device) = &mut self.device {
                        let mut events = Vec::new();
                        if notify_pointer_axis.dx != 0.0 {
                            events.push(InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_WHEEL.0,
                                notify_pointer_axis.dx as i32,
                            ));
                        }
                        if notify_pointer_axis.dy != 0.0 {
                            events.push(InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_HWHEEL.0,
                                notify_pointer_axis.dy as i32,
                            ));
                        }
                        let _ = device.emit(&events);
                    } else {
                        error!("[RemoteDesktop.NotifyPointerAxis] No virtual device was created.");
                    }
                }
                RemoteDesktopEvent::NotifyPointerAxisDiscrete(notify_pointer_axis_discrete) => {
                    if let Some(device) = &mut self.device {
                        let events = if notify_pointer_axis_discrete.axis == 0 {
                            vec![InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_WHEEL.0,
                                notify_pointer_axis_discrete.steps,
                            )]
                        } else {
                            vec![InputEvent::new(
                                EventType::RELATIVE.0,
                                RelativeAxisCode::REL_HWHEEL.0,
                                notify_pointer_axis_discrete.steps,
                            )]
                        };
                        let _ = device.emit(&events);
                    } else {
                        error!(
                            "[RemoteDesktop.NotifyPointerAxisDiscrete] No virtual device was created."
                        );
                    }
                }
                RemoteDesktopEvent::NotifyKeyboardKeycode(notify_keyboard_keycode) => {
                    if let Some(device) = &mut self.device {
                        let _ = device.emit(&[InputEvent::new(
                            EventType::KEY.0,
                            notify_keyboard_keycode.keycode as u16,
                            notify_keyboard_keycode.state as i32,
                        )]);
                    } else {
                        error!(
                            "[RemoteDesktop.NotifyKeyboardKeycode] No virtual device was created."
                        );
                    }
                }
                RemoteDesktopEvent::NotifyKeyboardKeysym(_) => {
                    let mut map = HashMap::new();
                    map.insert("error", Value::from("Don't support Keysym event yet."));
                    return_response(
                        to_return,
                        EventResponse::Standard(2, OwnedValue::from(map)),
                        "RemoteDesktop.NotifyKeyboardKeysym",
                    );
                    error!("[RemoteDesktop.NotifyKeyboardKeysym] No virtual device was created.");
                }
                RemoteDesktopEvent::GetPropertiesAvilableDeviceTypes => {
                    return_response(
                        to_return,
                        EventResponse::Value(OwnedValue::from(3u32)), // Keyboard(1) | Mouse(2)
                        "RemoteDesktop.GetPropertiesAvilableDeviceTypes",
                    );
                }
                RemoteDesktopEvent::GetPropertiesVersion => {
                    return_response(
                        to_return,
                        EventResponse::Value(OwnedValue::from(1u32)),
                        "RemoteDesktop.GetPropertiesVersion",
                    );
                }
            },
            _ => anyhow::bail!("Must be a remote desktop event send in the constructor"),
        }
        Ok(())
    }

    fn new(
        xdg_bypass: &mut crate::event_handler::XdgBypass,
        session: zbus::zvariant::OwnedObjectPath,
    ) -> anyhow::Result<Box<dyn EventHandler>>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
