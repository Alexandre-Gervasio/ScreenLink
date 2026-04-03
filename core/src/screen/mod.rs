//! Screen awareness module
//! Handles mouse boundary detection and multi-monitor scenarios

pub mod layout;

#[derive(Debug, Clone)]
pub struct Screen {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
}

impl Screen {
    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        x >= self.x_offset
            && x < self.x_offset + self.width
            && y >= self.y_offset
            && y < self.y_offset + self.height
    }
}

#[derive(Debug)]
pub struct ScreenLayout {
    screens: Vec<Screen>,
}

impl ScreenLayout {
    pub fn new() -> Self {
        Self {
            screens: Vec::new(),
        }
    }

    pub fn add_screen(&mut self, screen: Screen) {
        self.screens.push(screen);
    }

    pub fn find_screen(&self, x: u32, y: u32) -> Option<&Screen> {
        self.screens.iter().find(|s| s.contains_point(x, y))
    }
}

impl Default for ScreenLayout {
    fn default() -> Self {
        Self::new()
    }
}
