//! Screen layout management
//! Manages multi-monitor awareness

#[derive(Debug, Clone)]
pub struct MonitorLayout {
    pub monitors: Vec<MonitorInfo>,
}

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl MonitorLayout {
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
        }
    }

    pub fn add_monitor(&mut self, monitor: MonitorInfo) {
        self.monitors.push(monitor);
    }

    pub fn get_monitor_at(&self, x: i32, y: i32) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|m| {
            x >= m.x && x < m.x + m.width as i32 && y >= m.y && y < m.y + m.height as i32
        })
    }

    pub fn get_total_bounds(&self) -> (i32, i32, u32, u32) {
        if self.monitors.is_empty() {
            return (0, 0, 0, 0);
        }

        let min_x = self.monitors.iter().map(|m| m.x).min().unwrap_or(0);
        let min_y = self.monitors.iter().map(|m| m.y).min().unwrap_or(0);
        let max_x = self.monitors
            .iter()
            .map(|m| m.x + m.width as i32)
            .max()
            .unwrap_or(0);
        let max_y = self.monitors
            .iter()
            .map(|m| m.y + m.height as i32)
            .max()
            .unwrap_or(0);

        (min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32)
    }

    pub fn check_screen_edge(x: i32, screen_width: i32) -> bool {
        x >= screen_width
    }
}

impl Default for MonitorLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_detection() {
        let mut layout = MonitorLayout::new();
        layout.add_monitor(MonitorInfo {
            id: 1,
            name: "HDMI-1".to_string(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        });

        assert!(layout.get_monitor_at(960, 540).is_some());
        assert!(layout.get_monitor_at(2000, 540).is_none());
    }
}