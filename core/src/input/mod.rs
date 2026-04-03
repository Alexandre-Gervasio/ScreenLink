//! Input capture and injection module
//! Handles keyboard and mouse events across platforms

pub mod capture;
pub mod inject;
pub mod keymap;

#[cfg(target_os = "linux")]
pub use self::platform_linux as platform;

#[cfg(target_os = "windows")]
pub use self::platform_windows as platform;

#[cfg(target_os = "linux")]
mod platform_linux {
    use evdev::{Device, InputEventKind};
    use crate::utils::InputEventAdapter;

    pub fn capture_events() -> Result<(), Box<dyn std::error::Error>> {
        let mut device = Device::open("/dev/input/event0")?;

        loop {
            for ev in device.fetch_events()? {
                match ev.kind() {
                    InputEventKind::Key(key) => {
                        log::debug!("Key event: {:?}", key);
                    }
                    InputEventKind::RelAxis(_axis) => {
                        log::debug!("Mouse movement");
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn inject_event(event: &crate::InputEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            crate::InputEvent::KeyPress { keycode, down } => {
                super::super::inject::inject_key(*keycode, *down)?;
            }
            crate::InputEvent::MouseClick { button, down } => {
                super::super::inject::inject_mouse_click(*button, *down)?;
            }
            crate::InputEvent::MouseMove { x, y } => {
                super::super::inject::inject_mouse_move(*x, *y)?;
            }
            crate::InputEvent::Scroll { delta } => {
                super::super::inject::inject_scroll(*delta)?;
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod platform_windows {
    use winapi::um::winuser::*;
    use crate::InputEvent;

    pub fn capture_events() -> Result<(), Box<dyn std::error::Error>> {
        // Windows event capture will be implemented with raw input hooks
        log::info!("Windows event capture not yet implemented");
        Ok(())
    }

    pub fn inject_event(_event: &InputEvent) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Windows event injection placeholder");
        Ok(())
    }
}
