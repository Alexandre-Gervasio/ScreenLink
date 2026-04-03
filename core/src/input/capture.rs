use log::{info, debug, error, warn};
use std::fs;
use std::path::Path;

#[cfg(target_os = "linux")]
pub async fn capture_events() -> Result<(), Box<dyn std::error::Error>> {
    use evdev::{Device, InputEventKind};
    use tokio::task;
    use crate::input::keymap;

    info!("Starting event capture");
    
    // Run in a blocking task since evdev is synchronous
    task::spawn_blocking(|| {
        let devices = find_input_devices();
        info!("Found {} input devices", devices.len());
        
        if devices.is_empty() {
            error!("No input devices found. Try running with sudo.");
            return;
        }
        
        // Try to open all devices and process events
        for device_path in devices {
            match Device::open(&device_path) {
                Ok(mut device) => {
                    info!("Opened input device: {}", device_path);
                    if let Err(e) = process_device_events(&mut device) {
                        warn!("Error processing device {}: {}", device_path, e);
                    }
                }
                Err(e) => {
                    warn!("Failed to open input device {}: {}", device_path, e);
                }
            }
        }
    })
    .await?;
    
    Ok(())
}

#[cfg(target_os = "linux")]
fn find_input_devices() -> Vec<String> {
    let mut devices = Vec::new();
    
    if let Ok(entries) = fs::read_dir("/dev/input") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    // Prioritize keyboard and mouse devices
                    if name_str.starts_with("event") {
                        devices.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    
    devices
}

#[cfg(target_os = "linux")]
fn process_device_events(device: &mut evdev::Device) -> Result<(), Box<dyn std::error::Error>> {
    use evdev::InputEventKind;
    use crate::input::keymap;
    
    debug!("Processing events from device");
    
    loop {
        match device.fetch_events() {
            Ok(events) => {
                for ev in events {
                    match ev.kind() {
                        InputEventKind::Key(key) => {
                            let key_code = key.code();
                            let key_name = keymap::get_keycode_name(key_code);
                            let key_down = ev.value() != 0;
                            
                            debug!(
                                "Key event: {} (code: {}) - {}",
                                key_name,
                                key_code,
                                if key_down { "DOWN" } else { "UP" }
                            );
                            
                            // This event would be serialized and sent to network
                            // In a real implementation, this would send to server via TCP
                        }
                        InputEventKind::RelAxis(axis) => {
                            // Mouse movement: REL_X, REL_Y
                            let axis_code = axis.code();
                            let value = ev.value();
                            
                            if axis_code == 0 {
                                // REL_X
                                debug!("Mouse X movement: {}", value);
                            } else if axis_code == 1 {
                                // REL_Y
                                debug!("Mouse Y movement: {}", value);
                            }
                        }
                        InputEventKind::AbsAxis(_axis) => {
                            // Touchpad, tablet, etc
                            debug!("Absolute axis event: {:?}", _axis);
                        }
                        InputEventKind::Synchronization(_) => {
                            // Ignore sync events
                        }
                        InputEventKind::MiscellaneousDevice(_) => {
                            // Other events
                            debug!("Other event: {:?}", ev.kind());
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                error!("Error fetching events: {}", e);
                // Try to continue instead of breaking
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub async fn capture_events() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::task;
    
    info!("Windows event capture starting");
    
    task::spawn_blocking(|| {
        // TODO: Implement using Raw Input API or Windows hooks
        // Placeholder that continuously checks for input
        info!("Windows input capture - Structure ready for implementation");
        warn!("Windows input capture not yet fully implemented");
        
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    })
    .await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(target_os = "linux")]
    fn test_find_input_devices() {
        let devices = find_input_devices();
        // Should find at least one event device if the test is running on Linux with input
        println!("Found devices: {:?}", devices);
    }
}