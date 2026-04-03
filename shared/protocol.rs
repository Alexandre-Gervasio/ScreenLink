use serde::{Serialize, Deserialize};

/// Main input event type - core of KVM Pro protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseClick { button: u8, down: bool },
    KeyPress { keycode: u32, down: bool },
    Scroll { delta: i32 },
}

/// Network protocol version for compatibility checking
pub const PROTOCOL_VERSION: u32 = 1;

/// Maximum payload size (1 MB)
pub const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

/// Default server port
pub const DEFAULT_PORT: u16 = 5000;

/// Default discovery port (UDP)
pub const DISCOVERY_PORT: u16 = 5001;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_event_variants() {
        let _events = vec![
            InputEvent::MouseMove { x: 100, y: 200 },
            InputEvent::MouseClick { button: 1, down: true },
            InputEvent::KeyPress { keycode: 65, down: true },
            InputEvent::Scroll { delta: 3 },
        ];
    }
}