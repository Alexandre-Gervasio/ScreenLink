//! Clipboard sync module
//! Handles clipboard synchronization between machines

#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub text: Option<String>,
    pub timestamp: u64,
}

pub trait ClipboardProvider: Send + Sync {
    fn get_content(&self) -> Result<ClipboardContent, Box<dyn std::error::Error>>;
    fn set_content(&mut self, content: ClipboardContent) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(target_os = "linux")]
pub struct LinuxClipboard;

#[cfg(target_os = "windows")]
pub struct WindowsClipboard;

#[cfg(target_os = "linux")]
impl ClipboardProvider for LinuxClipboard {
    fn get_content(&self) -> Result<ClipboardContent, Box<dyn std::error::Error>> {
        log::warn!("Linux clipboard sync not yet implemented");
        Err("Not implemented".into())
    }

    fn set_content(&mut self, _content: ClipboardContent) -> Result<(), Box<dyn std::error::Error>> {
        log::warn!("Linux clipboard sync not yet implemented");
        Err("Not implemented".into())
    }
}

#[cfg(target_os = "windows")]
impl ClipboardProvider for WindowsClipboard {
    fn get_content(&self) -> Result<ClipboardContent, Box<dyn std::error::Error>> {
        log::warn!("Windows clipboard sync not yet implemented");
        Err("Not implemented".into())
    }

    fn set_content(&mut self, _content: ClipboardContent) -> Result<(), Box<dyn std::error::Error>> {
        log::warn!("Windows clipboard sync not yet implemented");
        Err("Not implemented".into())
    }
}
