#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn start_backend_server() {
    thread::spawn(|| {
        // Try to start the Node.js backend server
        // The server runs in the background and listens on port 3001
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", "npm run start --prefix ../../backend"]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(&["-c", "npm run start --prefix ../../backend"]);
            cmd
        };

        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok(); // Silently ignore if backend fails to start
    });

    // Give the server a moment to start
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    // Start the backend server before launching the Tauri app
    start_backend_server();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
