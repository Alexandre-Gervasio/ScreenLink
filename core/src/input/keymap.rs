// Linux keyboard keycode mapping
// Maps evdev keycodes to human-readable names and cross-platform codes

use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Modifiers
    LeftShift, RightShift, LeftCtrl, RightCtrl, LeftAlt, RightAlt,
    LeftMeta, RightMeta, // Windows/Super key
    
    // Control
    Escape, Tab, CapsLock, Backspace, Enter, Space,
    
    // Arrows
    Left, Right, Up, Down, Home, End, PageUp, PageDown,
    
    // Edit
    Insert, Delete, PrintScreen, ScrollLock, Pause,
    
    // Numpad
    NumLock, KpDivide, KpMultiply, KpMinus, KpPlus, KpEnter, KpDecimal,
    Kp0, Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9,
    
    // Special
    Menu, // Right-click menu
    Unknown(u16),
}

pub struct KeyInfo {
    pub evdev_code: u16,
    pub name: &'static str,
    pub unicode: Option<char>,
}

lazy_static! {
    static ref KEYCODE_MAP: HashMap<u16, KeyInfo> = {
        let mut map = HashMap::new();
        
        // Letters (evdev KEY_A = 30, etc)
        map.insert(30, KeyInfo { evdev_code: 30, name: "A", unicode: Some('a') });
        map.insert(48, KeyInfo { evdev_code: 48, name: "B", unicode: Some('b') });
        map.insert(46, KeyInfo { evdev_code: 46, name: "C", unicode: Some('c') });
        map.insert(32, KeyInfo { evdev_code: 32, name: "D", unicode: Some('d') });
        map.insert(18, KeyInfo { evdev_code: 18, name: "E", unicode: Some('e') });
        map.insert(33, KeyInfo { evdev_code: 33, name: "F", unicode: Some('f') });
        map.insert(34, KeyInfo { evdev_code: 34, name: "G", unicode: Some('g') });
        map.insert(35, KeyInfo { evdev_code: 35, name: "H", unicode: Some('h') });
        map.insert(23, KeyInfo { evdev_code: 23, name: "I", unicode: Some('i') });
        map.insert(36, KeyInfo { evdev_code: 36, name: "J", unicode: Some('j') });
        map.insert(37, KeyInfo { evdev_code: 37, name: "K", unicode: Some('k') });
        map.insert(38, KeyInfo { evdev_code: 38, name: "L", unicode: Some('l') });
        map.insert(50, KeyInfo { evdev_code: 50, name: "M", unicode: Some('m') });
        map.insert(49, KeyInfo { evdev_code: 49, name: "N", unicode: Some('n') });
        map.insert(24, KeyInfo { evdev_code: 24, name: "O", unicode: Some('o') });
        map.insert(25, KeyInfo { evdev_code: 25, name: "P", unicode: Some('p') });
        map.insert(16, KeyInfo { evdev_code: 16, name: "Q", unicode: Some('q') });
        map.insert(19, KeyInfo { evdev_code: 19, name: "R", unicode: Some('r') });
        map.insert(31, KeyInfo { evdev_code: 31, name: "S", unicode: Some('s') });
        map.insert(20, KeyInfo { evdev_code: 20, name: "T", unicode: Some('t') });
        map.insert(22, KeyInfo { evdev_code: 22, name: "U", unicode: Some('u') });
        map.insert(47, KeyInfo { evdev_code: 47, name: "V", unicode: Some('v') });
        map.insert(17, KeyInfo { evdev_code: 17, name: "W", unicode: Some('w') });
        map.insert(45, KeyInfo { evdev_code: 45, name: "X", unicode: Some('x') });
        map.insert(21, KeyInfo { evdev_code: 21, name: "Y", unicode: Some('y') });
        map.insert(44, KeyInfo { evdev_code: 44, name: "Z", unicode: Some('z') });
        
        // Numbers (evdev KEY_1 = 2, .. KEY_0 = 11)
        map.insert(2, KeyInfo { evdev_code: 2, name: "1", unicode: Some('1') });
        map.insert(3, KeyInfo { evdev_code: 3, name: "2", unicode: Some('2') });
        map.insert(4, KeyInfo { evdev_code: 4, name: "3", unicode: Some('3') });
        map.insert(5, KeyInfo { evdev_code: 5, name: "4", unicode: Some('4') });
        map.insert(6, KeyInfo { evdev_code: 6, name: "5", unicode: Some('5') });
        map.insert(7, KeyInfo { evdev_code: 7, name: "6", unicode: Some('6') });
        map.insert(8, KeyInfo { evdev_code: 8, name: "7", unicode: Some('7') });
        map.insert(9, KeyInfo { evdev_code: 9, name: "8", unicode: Some('8') });
        map.insert(10, KeyInfo { evdev_code: 10, name: "9", unicode: Some('9') });
        map.insert(11, KeyInfo { evdev_code: 11, name: "0", unicode: Some('0') });
        
        // Function keys
        map.insert(59, KeyInfo { evdev_code: 59, name: "F1", unicode: None });
        map.insert(60, KeyInfo { evdev_code: 60, name: "F2", unicode: None });
        map.insert(61, KeyInfo { evdev_code: 61, name: "F3", unicode: None });
        map.insert(62, KeyInfo { evdev_code: 62, name: "F4", unicode: None });
        map.insert(63, KeyInfo { evdev_code: 63, name: "F5", unicode: None });
        map.insert(64, KeyInfo { evdev_code: 64, name: "F6", unicode: None });
        map.insert(65, KeyInfo { evdev_code: 65, name: "F7", unicode: None });
        map.insert(66, KeyInfo { evdev_code: 66, name: "F8", unicode: None });
        map.insert(67, KeyInfo { evdev_code: 67, name: "F9", unicode: None });
        map.insert(68, KeyInfo { evdev_code: 68, name: "F10", unicode: None });
        map.insert(87, KeyInfo { evdev_code: 87, name: "F11", unicode: None });
        map.insert(88, KeyInfo { evdev_code: 88, name: "F12", unicode: None });
        
        // Modifiers
        map.insert(42, KeyInfo { evdev_code: 42, name: "LeftShift", unicode: None });
        map.insert(54, KeyInfo { evdev_code: 54, name: "RightShift", unicode: None });
        map.insert(29, KeyInfo { evdev_code: 29, name: "LeftCtrl", unicode: None });
        map.insert(97, KeyInfo { evdev_code: 97, name: "RightCtrl", unicode: None });
        map.insert(56, KeyInfo { evdev_code: 56, name: "LeftAlt", unicode: None });
        map.insert(100, KeyInfo { evdev_code: 100, name: "RightAlt", unicode: None });
        map.insert(125, KeyInfo { evdev_code: 125, name: "LeftMeta", unicode: None });
        map.insert(126, KeyInfo { evdev_code: 126, name: "RightMeta", unicode: None });
        
        // Control
        map.insert(1, KeyInfo { evdev_code: 1, name: "Escape", unicode: None });
        map.insert(15, KeyInfo { evdev_code: 15, name: "Tab", unicode: Some('\t') });
        map.insert(58, KeyInfo { evdev_code: 58, name: "CapsLock", unicode: None });
        map.insert(14, KeyInfo { evdev_code: 14, name: "Backspace", unicode: Some('\x08') });
        map.insert(28, KeyInfo { evdev_code: 28, name: "Enter", unicode: Some('\n') });
        map.insert(57, KeyInfo { evdev_code: 57, name: "Space", unicode: Some(' ') });
        
        // Arrows
        map.insert(105, KeyInfo { evdev_code: 105, name: "Left", unicode: None });
        map.insert(106, KeyInfo { evdev_code: 106, name: "Right", unicode: None });
        map.insert(103, KeyInfo { evdev_code: 103, name: "Up", unicode: None });
        map.insert(108, KeyInfo { evdev_code: 108, name: "Down", unicode: None });
        map.insert(102, KeyInfo { evdev_code: 102, name: "Home", unicode: None });
        map.insert(107, KeyInfo { evdev_code: 107, name: "End", unicode: None });
        map.insert(104, KeyInfo { evdev_code: 104, name: "PageUp", unicode: None });
        map.insert(109, KeyInfo { evdev_code: 109, name: "PageDown", unicode: None });
        
        // Edit
        map.insert(110, KeyInfo { evdev_code: 110, name: "Insert", unicode: None });
        map.insert(111, KeyInfo { evdev_code: 111, name: "Delete", unicode: None });
        map.insert(99, KeyInfo { evdev_code: 99, name: "PrintScreen", unicode: None });
        map.insert(70, KeyInfo { evdev_code: 70, name: "ScrollLock", unicode: None });
        map.insert(119, KeyInfo { evdev_code: 119, name: "Pause", unicode: None });
        
        // Numpad
        map.insert(69, KeyInfo { evdev_code: 69, name: "NumLock", unicode: None });
        map.insert(98, KeyInfo { evdev_code: 98, name: "KpDivide", unicode: Some('/') });
        map.insert(55, KeyInfo { evdev_code: 55, name: "KpMultiply", unicode: Some('*') });
        map.insert(74, KeyInfo { evdev_code: 74, name: "KpMinus", unicode: Some('-') });
        map.insert(78, KeyInfo { evdev_code: 78, name: "KpPlus", unicode: Some('+') });
        map.insert(96, KeyInfo { evdev_code: 96, name: "KpEnter", unicode: None });
        map.insert(83, KeyInfo { evdev_code: 83, name: "KpDecimal", unicode: Some('.') });
        map.insert(82, KeyInfo { evdev_code: 82, name: "Kp0", unicode: Some('0') });
        map.insert(79, KeyInfo { evdev_code: 79, name: "Kp1", unicode: Some('1') });
        map.insert(80, KeyInfo { evdev_code: 80, name: "Kp2", unicode: Some('2') });
        map.insert(81, KeyInfo { evdev_code: 81, name: "Kp3", unicode: Some('3') });
        map.insert(75, KeyInfo { evdev_code: 75, name: "Kp4", unicode: Some('4') });
        map.insert(76, KeyInfo { evdev_code: 76, name: "Kp5", unicode: Some('5') });
        map.insert(77, KeyInfo { evdev_code: 77, name: "Kp6", unicode: Some('6') });
        map.insert(71, KeyInfo { evdev_code: 71, name: "Kp7", unicode: Some('7') });
        map.insert(72, KeyInfo { evdev_code: 72, name: "Kp8", unicode: Some('8') });
        map.insert(73, KeyInfo { evdev_code: 73, name: "Kp9", unicode: Some('9') });
        
        // Special
        map.insert(127, KeyInfo { evdev_code: 127, name: "Menu", unicode: None });
        
        map
    };
}

pub fn get_keycode_name(evdev_code: u16) -> String {
    KEYCODE_MAP
        .get(&evdev_code)
        .map(|k| k.name.to_string())
        .unwrap_or_else(|| format!("Unknown({})", evdev_code))
}

pub fn get_unicode(evdev_code: u16) -> Option<char> {
    KEYCODE_MAP.get(&evdev_code).and_then(|k| k.unicode)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keycode_mapping() {
        assert_eq!(get_keycode_name(30), "A");
        assert_eq!(get_keycode_name(11), "0");
        assert_eq!(get_keycode_name(28), "Enter");
        assert_eq!(get_unicode(57), Some(' '));
    }
}
