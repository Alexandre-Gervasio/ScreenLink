use log::{info, error};
use crate::InputEvent;

#[cfg(target_os = "linux")]
pub async fn inject_event(event: &InputEvent) -> Result<(), Box<dyn std::error::Error>> {
    use uinput::event::keyboard::Key;
    use tokio::task;

    let event = event.clone();
    task::spawn_blocking(move || {
        match inject_event_sync(&event) {
            Ok(_) => {}
            Err(e) => error!("Failed to inject event: {}", e),
        }
    })
    .await?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn inject_event_sync(event: &InputEvent) -> Result<(), Box<dyn std::error::Error>> {
    use uinput::event::keyboard::Key;

    match event {
        InputEvent::KeyPress { keycode, down } => {
            inject_key_sync(*keycode, *down)?;
        }
        InputEvent::MouseClick { button, down } => {
            inject_mouse_click_sync(*button, *down)?;
        }
        InputEvent::MouseMove { x, y } => {
            inject_mouse_move_sync(*x, *y)?;
        }
        InputEvent::Scroll { delta } => {
            inject_scroll_sync(*delta)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn inject_key(
    _keycode: u32,
    _down: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn inject_key_sync(_keycode: u32, _down: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut device = uinput::default()?
        .name("kvm-pro-virtual")?
        .event(uinput::event::Keyboard::All)?
        .create()?;

    // TODO: Map keycode to uinput key
    // device.press(&Key::A)?;
    // device.synchronize()?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn inject_mouse_click(_button: u8, _down: bool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn inject_mouse_click_sync(_button: u8, _down: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement mouse click injection
    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn inject_mouse_move(_x: i32, _y: i32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn inject_mouse_move_sync(_x: i32, _y: i32) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement mouse move injection
    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn inject_scroll(_delta: i32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn inject_scroll_sync(_delta: i32) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement scroll injection
    Ok(())
}

// Windows implementations
#[cfg(target_os = "windows")]
pub async fn inject_event(_event: &InputEvent) -> Result<(), Box<dyn std::error::Error>> {
    info!("Windows event injection - Not yet implemented");
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn inject_key(_keycode: u32, _down: bool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn inject_mouse_click(_button: u8, _down: bool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn inject_mouse_move(_x: i32, _y: i32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn inject_scroll(_delta: i32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}