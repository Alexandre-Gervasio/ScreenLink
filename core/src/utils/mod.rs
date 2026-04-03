//! Utility functions and helpers

use crate::InputEvent;

pub trait InputEventAdapter {
    fn to_platform_specific(&self) -> Box<dyn std::any::Any>;
}

pub fn serialize_event(event: &InputEvent) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(bincode::serialize(event)?)
}

pub fn deserialize_event(data: &[u8]) -> Result<InputEvent, Box<dyn std::error::Error>> {
    Ok(bincode::deserialize(data)?)
}

pub fn format_event(event: &InputEvent) -> String {
    match event {
        InputEvent::KeyPress { keycode, down } => {
            format!("KeyPress {{ keycode: {}, down: {} }}", keycode, down)
        }
        InputEvent::MouseClick { button, down } => {
            format!("MouseClick {{ button: {}, down: {} }}", button, down)
        }
        InputEvent::MouseMove { x, y } => {
            format!("MouseMove {{ x: {}, y: {} }}", x, y)
        }
        InputEvent::Scroll { delta } => {
            format!("Scroll {{ delta: {} }}", delta)
        }
    }
}
